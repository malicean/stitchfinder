use anyhow::Context;
use clap::Parser;
use core::fmt;
use rayon::prelude::*;
use std::{collections::HashSet, fs, path::PathBuf};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let raw = fs::read_to_string(cli.db).with_context(|| "failed to read words list")?;
    let ctx = {
        let db = {
            let split = raw.split('\n');

            let mut alloc = {
                let count = split.clone().count();
                HashSet::with_capacity(count)
            };

            alloc.extend(split.map(str::trim));
            alloc
        };

        Ctx {
            subsume: cli.subsume,
            given: cli.given,
            db,
        }
    };

    ctx.run()
}

#[derive(Parser)]
struct Cli {
    /// Allows the found word to subsume the given word.
    /// Subsuming entails that the found word has the entire given word within it, and that the given word is at its start or end.
    #[arg(short, long)]
    subsume: bool,

    /// The file which contains the list of possible (valid) words, delimited by newlines
    db: PathBuf,

    /// The word too stitch into another word
    given: String,
}

struct Ctx<'a> {
    subsume: bool,
    given: String,
    db: HashSet<&'a str>,
}

#[derive(Debug, Copy, Clone)]
enum Position {
    Left,
    Right,
}

#[derive(tabled::Tabled)]
struct Row<'a> {
    #[tabled(rename = "Stitched")]
    stitched: String,

    #[tabled(rename = "Intersection")]
    isect_len: usize,

    #[tabled(rename = "Position")]
    pos: Position,

    #[tabled(rename = "Found")]
    found: &'a str,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Position::Left => "left",
            Position::Right => "right",
        };

        f.write_str(symbol)
    }
}

impl Position {
    /// returns the remaining given and intersection in a tuple
    pub fn fracture(self, given: &str, isect_len: usize) -> (&str, &str) {
        match self {
            Position::Left => {
                let (rem, isect) = given.split_at(given.len() - isect_len);
                (rem, isect)
            }
            Position::Right => {
                let (isect, rem) = given.split_at(isect_len);
                (rem, isect)
            }
        }
    }

    pub fn valid(self, db: &HashSet<&str>, rem: &str, isect: &str, found: &str) -> bool {
        match self {
            Position::Left => {
                found.starts_with(isect) && db.contains(&rem) && db.contains(&found[isect.len()..])
            }
            Position::Right => {
                found.ends_with(isect)
                    && db.contains(&rem)
                    && db.contains(&found[found.len() - isect.len()..])
            }
        }
    }

    fn make_row<'f>(self, given: &'f str, found: &'f str, isect_len: usize) -> Row<'f> {
        match self {
            Position::Left => {
                let found_no_isect = &found[isect_len..];
                let stitched = format!("{given}{found_no_isect}");

                Row {
                    stitched,
                    isect_len,
                    pos: self,
                    found,
                }
            }
            Position::Right => {
                let found_no_isect = &found[..(found.len() - isect_len)];
                let stitched = format!("{found_no_isect}{given}");

                Row {
                    stitched,
                    isect_len,
                    pos: self,
                    found,
                }
            }
        }
    }
}

impl<'a> Ctx<'a> {
    fn run(&self) -> anyhow::Result<()> {
        // left
        //
        // twink + knight:
        // twinkknight =>
        // - twink night
        // - twin knight
        //
        // Notice that twink is the given word, so it is assumed valid, and knight is a dictionary word, so it is assumed valid.
        // Also notice that when twink has the k, night must be a valid word, and when knight has the k, twin must be a valid word.
        //
        // twin[ k]night
        // twin[k ]night
        //
        // Thus, both the given word and the found word must be valid words without the intersecting text.
        // The checks can be diagrammed as such:
        //
        // twin[ k]night | <check> <known>
        // twin[k ]night | <known> <check>
        //
        // i.e.
        //
        // - twink: valid (given)
        // - twin: unknown
        // - knight: valid (found)
        // - night: unknown
        //
        // Sometimes the start of the found word aligns with the start of the given word, i.e. the found word encompasses the given word with left-alignment:
        //
        // theo + theoretical
        // theotheoretical =>
        // - theo    retical
        // - [empty] theoretical
        //
        // Although retical is a valid word, what could be in its place may not be, but theoretical remains valid. This should be allowed.
        // Thus, the following pattern should exist:
        //
        // theo        retical | <given> <discard>
        // [empty] theoretical | <empty> <result>
        //
        // i.e.
        //
        // - theo: valid (given)
        // - [E]: valid (with this rule)
        // - theoretical: valid (found)
        // - retical: discard

        // right
        //
        //     01 234
        // rev.EL.ENA
        //
        //   01234
        // m.ALICE
        // e.LENA

        // special case: found word subsumes given word
        let mut rows = if self.subsume {
            self.db
                .par_iter()
                .filter_map(|found| {
                    if found.starts_with(&self.given) {
                        Some(Row {
                            stitched: found.to_string(),
                            isect_len: self.given.len(),
                            pos: Position::Left,
                            found,
                        })
                    } else if found.ends_with(&self.given) {
                        Some(Row {
                            stitched: found.to_string(),
                            isect_len: self.given.len(),
                            pos: Position::Right,
                            found,
                        })
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            Vec::new()
        };

        // standard case: found word intersects with given word
        let mut rest: Vec<_> = self
            .db
            .par_iter()
            .flat_map(|found| {
                (1..(self.given.len() - 1))
                    .into_par_iter()
                    .rev()
                    .flat_map(move |isect_len| {
                        [Position::Left, Position::Right]
                            .into_par_iter()
                            .map(move |pos| (pos, isect_len, found))
                    })
            })
            .filter(|&(pos, isect_len, found)| {
                let (rem, isect) = pos.fracture(&self.given, isect_len);

                pos.valid(&self.db, rem, isect, found)
            })
            .map(|(pos, isect_len, found)| pos.make_row(&self.given, found, isect_len))
            .collect();

        rows.append(&mut rest);

        let pretty = tabled::Table::new(rows)
            .with(tabled::settings::Style::blank())
            .with(tabled::settings::Alignment::left())
            .to_string();
        println!("{pretty}");

        Ok(())
    }
}
