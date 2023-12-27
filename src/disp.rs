use core::fmt;
use std::io::Write;

use anyhow::Context;
use rayon::{prelude::ParallelIterator, slice::ParallelSliceMut};
use tabled::Table;

use crate::matcher::{Combo, Position};

#[derive(tabled::Tabled, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Row<'a> {
    #[tabled(rename = "Stitched")]
    stitch: crate::matcher::Whole<'a>,

    #[tabled(rename = "Found")]
    found: &'a str,

    #[tabled(rename = "I-sect")]
    isect: &'a str,

    #[tabled(rename = "Rem-given")]
    rem_given: &'a str,

    #[tabled(rename = "Rem-found")]
    rem_found: &'a str,

    #[tabled(rename = "Pos")]
    pos: Position,

    #[tabled(rename = "Valid")]
    valid: bool,

    #[tabled(rename = "X-given")]
    expand_given: &'a str,

    #[tabled(rename = "X-pos")]
    expand_pos: OptPos,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct OptPos(Option<Position>);

impl fmt::Display for OptPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(pos) = self.0 {
            pos.fmt(f)
        } else {
            Ok(())
        }
    }
}

fn rows<'a>(combos: impl ParallelIterator<Item = Combo<'a>>) -> Vec<Row<'a>> {
    let mut vec: Vec<_> = combos
        .map(|combo| {
            let parts = combo.stitch.into_parts();

            Row {
                stitch: combo.stitch.whole(),
                found: parts.trans.pair.found,
                isect: parts.isect,
                rem_given: parts.rem_given,
                rem_found: parts.rem_found,
                pos: parts.trans.pos,
                valid: combo.valid,
                expand_given: combo.expand.map_or("", |(x_given, _)| x_given),
                expand_pos: OptPos(combo.expand.map(|(_, pos)| pos)),
            }
        })
        .collect();

    vec.par_sort_unstable();
    vec
}

pub fn println<'a>(combos: impl ParallelIterator<Item = Combo<'a>>) -> anyhow::Result<()> {
    use tabled::settings::*;

    let mut table = Table::new(rows(combos));
    table.with(Style::blank()).with(Alignment::left());

    std::io::stdout()
        .lock()
        .write_fmt(format_args!("{table}"))
        .with_context(|| "failed to print results")
}
