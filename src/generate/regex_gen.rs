use rand::distributions::{Distribution, Standard, Uniform};
use rand::prelude::*;
use regex_syntax::hir::{self, Hir, HirKind};
use regex_syntax::Parser;
use std::iter::FromIterator;

use encoding::all::UTF_8;
use encoding::{DecoderTrap, EncoderTrap, Encoding};

const MAX_REPEAT: u32 = 100;

struct RandomizeState<'a, R: Rng> {
    pub rng: &'a mut R,
}

fn randomize_alternation<R: Rng>(
    rstate: &mut RandomizeState<R>,
    mut exprs: Vec<Hir>,
) -> Result<String, ()> {
    exprs.shuffle(rstate.rng);
    if !exprs.is_empty() {
        randomize_for(rstate, exprs[0].kind().clone())
    } else {
        Err(())
    }
}

fn randomize_word_boundry<R: Rng>(
    _rstate: &mut RandomizeState<R>,
    _wb: hir::WordBoundary,
) -> Result<String, ()> {
    Ok(String::from(" "))
}

fn randomize_anchor<R: Rng>(
    _rstate: &mut RandomizeState<R>,
    _anchor: hir::Anchor,
) -> Result<String, ()> {
    Ok(String::from(""))
}

fn randomize_group<R: Rng>(
    rstate: &mut RandomizeState<R>,
    group: hir::Group,
) -> Result<String, ()> {
    randomize_for(rstate, group.hir.kind().clone())
}

fn randomize_literal<R: Rng>(
    _rstate: &mut RandomizeState<R>,
    literal: hir::Literal,
) -> Result<String, ()> {
    match literal {
        hir::Literal::Unicode(c) => Ok(String::from_iter([c].iter())),
        hir::Literal::Byte(_) => Err(()),
    }
}

fn randomize_concat<R: Rng>(rstate: &mut RandomizeState<R>, exprs: Vec<Hir>) -> Result<String, ()> {
    let mut s = String::new();
    for e in &exprs {
        s += &randomize_for(rstate, e.kind().clone())?;
    }
    Ok(s)
}

fn repeat_exactly<R: Rng>(rstate: &mut RandomizeState<R>, h: Hir, n: u32) -> Result<String, ()> {
    let s = (0..n)
        .map(|_| randomize_for(rstate, h.kind().clone()).unwrap())
        .collect::<Vec<String>>()
        .join("");
    Ok(s)
}

fn repeat_at_least<R: Rng + RngCore>(
    rstate: &mut RandomizeState<R>,
    h: Hir,
    n: u32,
) -> Result<String, ()> {
    let dist = Uniform::from(n..MAX_REPEAT);
    let n = dist.sample(rstate.rng);
    let s = (0..n)
        .map(|_| randomize_for(rstate, h.kind().clone()).unwrap())
        .collect::<Vec<String>>()
        .join("");
    Ok(s)
}

fn repeat_bounded<R: Rng + RngCore>(
    rstate: &mut RandomizeState<R>,
    h: Hir,
    mn: u32,
    mx: u32,
) -> Result<String, ()> {
    let mx = mx + 1;
    let dist = Uniform::from(mn..mx);
    let n = dist.sample(rstate.rng);
    let s = (0..n)
        .map(|_| randomize_for(rstate, h.kind().clone()).unwrap())
        .collect::<Vec<String>>()
        .join("");
    Ok(s)
}

fn randomize_unicode_class<R: Rng + RngCore>(
    rstate: &mut RandomizeState<R>,
    cls: hir::ClassUnicode,
) -> Result<String, ()> {
    let mut chars: Vec<char> = Vec::new();

    for r in cls.iter() {
        let s = r.start();
        let e = r.end();
        if let (Ok(s), Ok(e)) = (
            UTF_8.encode(&String::from_iter([s].iter()), EncoderTrap::Strict),
            UTF_8.encode(&String::from_iter([e].iter()), EncoderTrap::Strict),
        ) {
            if s.len() > 0 && e.len() > 0 {
                let s = s[0];
                let e = e[0] + 1;

                for byte in s..e {
                    if let Ok(s) = UTF_8.decode(&[byte], DecoderTrap::Strict) {
                        let c = s.chars().nth(0).unwrap();
                        chars.push(c);
                    }
                }
            }
        }
    }
    chars.shuffle(rstate.rng);

    Ok(String::from_iter([chars[0]].iter()))
}

fn randomize_class<R: Rng + RngCore>(
    rstate: &mut RandomizeState<R>,
    cls: hir::Class,
) -> Result<String, ()> {
    match cls {
        hir::Class::Unicode(cls) => randomize_unicode_class(rstate, cls),
        _ => Err(()),
    }
}

fn randomize_repetition<R: Rng>(
    rstate: &mut RandomizeState<R>,
    rep: hir::Repetition,
) -> Result<String, ()> {
    let hir = rep.hir;

    match rep.kind {
        hir::RepetitionKind::ZeroOrOne => repeat_bounded(rstate, hir.as_ref().clone(), 0, 1),
        hir::RepetitionKind::ZeroOrMore => {
            repeat_bounded(rstate, hir.as_ref().clone(), 0, MAX_REPEAT)
        }
        hir::RepetitionKind::OneOrMore => {
            repeat_bounded(rstate, hir.as_ref().clone(), 1, MAX_REPEAT)
        }
        hir::RepetitionKind::Range(range) => match range {
            hir::RepetitionRange::Exactly(n) => repeat_exactly(rstate, hir.as_ref().clone(), n),
            hir::RepetitionRange::AtLeast(n) => repeat_at_least(rstate, hir.as_ref().clone(), n),
            hir::RepetitionRange::Bounded(mn, mx) => {
                repeat_bounded(rstate, hir.as_ref().clone(), mn, mx)
            }
        },
    }
}

fn randomize_for<R: Rng>(rstate: &mut RandomizeState<R>, kind: HirKind) -> Result<String, ()> {
    match kind {
        HirKind::Alternation(exprs) => randomize_alternation(rstate, exprs),
        HirKind::Literal(lit) => randomize_literal(rstate, lit),
        HirKind::Concat(exprs) => randomize_concat(rstate, exprs),
        HirKind::Repetition(rep) => randomize_repetition(rstate, rep),
        HirKind::Group(grp) => randomize_group(rstate, grp),
        HirKind::Class(cls) => randomize_class(rstate, cls),
        HirKind::Anchor(a) => randomize_anchor(rstate, a),
        HirKind::WordBoundary(wb) => randomize_word_boundry(rstate, wb),
        _ => Err(()),
    }
}

pub struct RegexGen {
    hir: Hir,
}

impl RegexGen {
    pub fn new(pattern: &str) -> Option<Self> {
        if let Ok(hir) = Parser::new().parse(pattern) {
            Some(Self { hir })
        } else {
            None
        }
    }
    pub fn kind(&self) -> &HirKind {
        self.hir.kind()
    }
    pub fn randomize(&self, rng: &mut impl Rng) -> Result<String, ()> {
        let mut rstate = RandomizeState { rng: rng };
        randomize_for(&mut rstate, self.kind().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_pcg::Pcg64;
    #[test]
    fn hir_randomize_test() {
        let mut rng = Pcg64::seed_from_u64(0);
        let gen = RegexGen::new("([a-zA-Z]){1,3}").unwrap();
        if let Ok(s) = gen.randomize(&mut rng) {
            println!("{}", s);
        }
    }
    #[test]
    fn hir_parser_test() {
        let hir = Parser::new().parse("a|b").unwrap();
        assert_eq!(
            hir,
            Hir::alternation(vec![
                Hir::literal(hir::Literal::Unicode('a')),
                Hir::literal(hir::Literal::Unicode('b')),
            ])
        );
    }
}
