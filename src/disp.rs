use core::fmt;
use std::io::Write;

use anyhow::Context;
use rayon::{prelude::ParallelIterator, slice::ParallelSliceMut};
use tabled::Table;

use crate::{
    matcher::{Combo, Pairing, StitchParts},
    Position,
};

#[derive(tabled::Tabled, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Row<'a> {
    #[tabled(rename = "Stitched")]
    stitch: crate::matcher::Whole<'a>,

    #[tabled(rename = "Pos-given")]
    pos_given: OptPos,

    #[tabled(rename = "Pos-expans")]
    pos_expans: Position,

    #[tabled(rename = "Valid")]
    valid: bool,

    #[tabled(rename = "I-sect")]
    isect: &'a str,

    #[tabled(rename = "Expansion")]
    expans: &'a str,

    #[tabled(rename = "Found")]
    found: &'a str,

    #[tabled(rename = "Rem-expans")]
    rem_expans: &'a str,

    #[tabled(rename = "Rem-found")]
    rem_found: &'a str,
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
            let StitchParts {
                trans,
                isect,
                rem_expans,
                rem_found,
            } = parts;
            let Pairing { expans, found } = trans.pair;

            Row {
                stitch: combo.stitch.whole(),
                pos_given: OptPos(combo.pos_given),
                pos_expans: trans.pos,
                valid: combo.valid,
                isect,
                expans,
                found,
                rem_expans,
                rem_found,
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
