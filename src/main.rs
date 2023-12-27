use anyhow::Context;
use clap::{Parser, ValueEnum};
use core::fmt;
use std::{collections::HashSet, fs, path::PathBuf};

mod disp;
mod ext;
mod matcher;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let raw = fs::read_to_string(cli.founds).with_context(|| "failed to read words list")?;
    let ctx = {
        let founds = {
            let split = raw.split('\n');

            let mut alloc = {
                let count = split.clone().count();
                HashSet::with_capacity(count)
            };

            alloc.extend(split.map(str::trim));
            alloc.remove(&"");
            alloc
        };

        Ctx {
            disable_exp: cli.disable_expansion,
            valid: cli.valid,
            pos: cli.position,
            exp_pos: cli.expansion_position,
            given: cli.given,
            founds,
        }
    };

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

    let all = matcher::find_all(&ctx);
    disp::println(all)
}

#[derive(Parser)]
struct Cli {
    /// Requires that "X-given" and "X-pos" columns be empty, i.e. disables expansion.
    #[arg(long)]
    disable_expansion: bool,

    /// Requires that the "Valid" column be a value.
    #[arg(long)]
    valid: Option<bool>,

    /// Requires that the "Pos" column be a value.
    #[arg(long)]
    position: Option<Position>,

    /// Requires that the "X-pos" column be a value. Ignored if `--disable-expansion` is present.
    #[arg(long)]
    expansion_position: Option<Position>,

    /// Path to the file which contains the word list, delimited by newlines
    founds: PathBuf,

    /// The word to stitch into another word
    given: String,
}

struct Ctx<'a> {
    disable_exp: bool,
    valid: Option<bool>,
    pos: Option<Position>,
    exp_pos: Option<Position>,
    given: String,
    founds: HashSet<&'a str>,
}

#[derive(ValueEnum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Position {
    /// Given word is to the left of the found word
    /// <given> <found>
    Left,
    /// Given word is to the right of the found word
    /// <found> <given>
    Right,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lit = match self {
            Position::Left => "left",
            Position::Right => "right",
        };

        f.write_str(lit)
    }
}

impl Position {
    pub fn all() -> [Position; 2] {
        [Position::Left, Position::Right]
    }
}
