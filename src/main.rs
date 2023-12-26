use anyhow::Context;
use clap::Parser;
use std::{collections::HashSet, fs, path::PathBuf};

mod disp;
mod ext;
mod matcher;

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
            alloc.remove(&"");
            alloc
        };

        Ctx {
            given: cli.given,
            db,
        }
    };

    ctx.run()
}

#[derive(Parser)]
struct Cli {
    /// The file which contains the list of possible (valid) words, delimited by newlines
    db: PathBuf,

    /// The word too stitch into another word
    given: String,
}

struct Ctx<'a> {
    given: String,
    db: HashSet<&'a str>,
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

        let all = matcher::find_all(&self.given, &self.db);
        disp::println(all);

        Ok(())
    }
}
