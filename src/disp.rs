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

    #[tabled(rename = "Position")]
    pos: Position,

    #[tabled(rename = "Valid")]
    valid: bool,
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
            }
        })
        .collect();

    vec.par_sort_unstable();
    vec
}

pub fn println<'a>(combos: impl ParallelIterator<Item = Combo<'a>>) {
    use tabled::settings::*;

    let mut table = Table::new(rows(combos));
    table.with(Style::blank()).with(Alignment::left());

    println!("{table}");
}
