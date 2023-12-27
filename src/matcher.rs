use core::fmt;
use std::collections::HashSet;

use crate::{ext::*, Ctx, Position};
use rayon::prelude::*;

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
    pub expand: Option<(&'a str, Position)>,
}

/// Provides all the extrapolations of a given word. Does not include the given word itself.
fn extrap<'f>(ctx: &'f Ctx<'_>) -> impl ParallelIterator<Item = (&'f str, Position)> {
    let lambda: Box<dyn Fn(&&'f str) -> Option<(&'f str, Position)> + Send + Sync> =
        if ctx.disable_exp {
            Box::new(|_| None)
        } else {
            Box::new(move |&found: &&'f str| {
                if ctx.given.len() >= found.len() {
                    None
                } else {
                    let starts = || found.starts_with(&ctx.given);
                    let ends = || found.starts_with(&ctx.given);

                    match ctx.exp_pos {
                        Some(Position::Left) => starts().then_some((found, Position::Left)),
                        Some(Position::Right) => ends().then_some((found, Position::Right)),
                        None => {
                            let pos = if starts() {
                                Some(Position::Left)
                            } else if ends() {
                                Some(Position::Right)
                            } else {
                                None
                            };

                            pos.map(|p| (found, p))
                        }
                    }
                }
            })
        };

    ctx.founds.par_iter().filter_map(lambda)
}

pub fn find_all<'f>(ctx: &'f Ctx<'_>) -> impl ParallelIterator<Item = Combo<'f>> {
    let extrap = extrap(ctx).map(|(found, pos)| (found, Some(pos)));

    [(ctx.given.as_str(), None)]
        .into_par_iter()
        .chain(extrap)
        .flat_map(move |(expand_word, expand_pos)| {
            ctx.founds
                .par_iter()
                .map(|&found| Pairing {
                    given: expand_word,
                    found,
                })
                .flat_map(|pair| {
                    Position::all()
                        .into_par_iter()
                        .filter(|&pos| ctx.pos.map_or(true, |p| p == pos))
                        .map(move |pos| Transform { pair, pos })
                })
                .flat_map(|trans| trans.stitches())
                .map(move |stitch| Combo {
                    stitch,
                    valid: stitch.valid(&ctx.founds),
                    expand: expand_pos.map(|pos| (expand_word, pos)),
                })
        })
        .filter(|combo| ctx.valid.map_or(true, |b| b == combo.valid))
}
