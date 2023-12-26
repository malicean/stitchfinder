use core::fmt;
use std::collections::HashSet;

use crate::ext::*;
use rayon::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Copy, Clone)]
pub struct Pairing<'a> {
    pub given: &'a str,
    pub found: &'a str,
}

impl<'a> Pairing<'a> {
    fn max_isect_len(&self) -> usize {
        usize::min(self.given.len(), self.found.len())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Transform<'a> {
    pub pair: Pairing<'a>,
    pub pos: Position,
}

impl<'a> Transform<'a> {
    /// Fractures the transform into its intersection, remaining given, and remaining found (if the intersection exists)
    fn fracture(&self, isect_len: usize) -> Option<(&'a str, &'a str, &'a str)> {
        let Pairing { given, found } = self.pair;

        let (igiven, ifound, rgiven, rfound) = match self.pos {
            Position::Left => {
                let (rgiven, igiven) = given.rsplit_at(isect_len);
                let (ifound, rfound) = found.split_at(isect_len);

                (igiven, ifound, rgiven, rfound)
            }
            Position::Right => {
                let (rfound, ifound) = found.rsplit_at(isect_len);
                let (igiven, rgiven) = given.split_at(isect_len);

                (igiven, ifound, rgiven, rfound)
            }
        };

        // if "nacho" == found {
        //     dbg!((igiven, ifound, rgiven, rfound));
        // }

        (igiven == ifound).then_some((igiven, rgiven, rfound))
    }

    fn stitches(self) -> impl ParallelIterator<Item = Stitch<'a>> {
        (1..self.pair.max_isect_len())
            .into_par_iter()
            .filter_map(move |isect_len| Stitch::new(self, isect_len))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Stitch<'a> {
    trans: Transform<'a>,
    isect: &'a str,
    rem_given: &'a str,
    rem_found: &'a str,
}

impl<'a> Stitch<'a> {
    fn new(trans: Transform<'a>, isect_len: usize) -> Option<Self> {
        trans
            .fracture(isect_len)
            .map(|(isect, rem_given, rem_found)| Self {
                trans,
                isect,
                rem_given,
                rem_found,
            })
    }

    fn valid(&self, words: &HashSet<&str>) -> bool {
        let for_word = |word| word == "" || words.contains(word);

        for_word(self.rem_given) && for_word(self.rem_found)
    }

    pub fn whole(&self) -> Whole<'a> {
        match self.trans.pos {
            Position::Left => Whole {
                left: self.trans.pair.given,
                right: self.rem_found,
            },
            Position::Right => Whole {
                left: self.rem_found,
                right: self.trans.pair.given,
            },
        }
    }

    pub fn into_parts(self) -> StitchParts<'a> {
        self.into()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StitchParts<'a> {
    pub trans: Transform<'a>,
    pub isect: &'a str,
    pub rem_given: &'a str,
    pub rem_found: &'a str,
}

impl<'a> From<Stitch<'a>> for StitchParts<'a> {
    fn from(v: Stitch<'a>) -> Self {
        Self {
            trans: v.trans,
            isect: v.isect,
            rem_given: v.rem_given,
            rem_found: v.rem_found,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Whole<'a> {
    left: &'a str,
    right: &'a str,
}

impl fmt::Display for Whole<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.left)?;
        f.write_str(self.right)?;

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Combo<'a> {
    pub stitch: Stitch<'a>,
    pub valid: bool,
}

pub fn find_all<'i>(
    given: &'i str,
    founds: &'i HashSet<&str>,
) -> impl ParallelIterator<Item = Combo<'i>> {
    founds
        .into_par_iter()
        .map(|found| Pairing { given, found })
        .flat_map(|pair| {
            Position::all()
                .into_par_iter()
                .map(move |pos| Transform { pair, pos })
        })
        .flat_map(|trans| trans.stitches())
        .map(move |stitch| Combo {
            stitch,
            valid: stitch.valid(founds),
        })
}
