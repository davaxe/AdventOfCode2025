use std::collections::HashMap;

use chumsky::{extra::Full, prelude::*};

use crate::{Block, Region};

type R<'a> = Full<Rich<'a, char>, (), ()>;

fn cell<'a>() -> impl Parser<'a, &'a str, bool, R<'a>> {
    just('#').to(true).or(just('.').to(false))
}

fn row<'a>() -> impl Parser<'a, &'a str, [bool; 3], R<'a>> {
    cell()
        .repeated()
        .exactly(3)
        .collect::<Vec<_>>()
        .map(|vec| {
            let mut arr = [false; 3];
            arr.copy_from_slice(&vec);
            arr
        })
        .then_ignore(text::newline().repeated())
}

fn block<'a>() -> impl Parser<'a, &'a str, Block, R<'a>> {
    let occupancy = row().repeated().exactly(3).collect::<Vec<_>>();
    let occupancy = occupancy.map(|vec| {
        let mut arr = [[false; 3]; 3];
        for (i, row) in vec.into_iter().enumerate() {
            arr[i] = row;
        }
        arr
    });
    occupancy.map(Block::new)
}

fn block_index<'a>() -> impl Parser<'a, &'a str, usize, R<'a>> {
    text::int(10)
        .map(|s: &str| s.parse::<usize>().expect("Failed to parse usize"))
        .then_ignore(just(":"))
        .then_ignore(text::newline())
}

fn blocks<'a>() -> impl Parser<'a, &'a str, HashMap<usize, Block>, R<'a>> {
    block_index()
        .then(block())
        .repeated()
        .collect::<Vec<(usize, Block)>>()
        .map(|vec| vec.into_iter().collect())
        .then_ignore(text::newline().repeated())
}

fn size<'a>() -> impl Parser<'a, &'a str, (u32, u32), R<'a>> {
    text::int(10)
        .map(|s: &str| s.parse::<u32>().expect("Failed to parse u32"))
        .then_ignore(just("x"))
        .then(text::int(10).map(|s: &str| s.parse::<u32>().expect("Failed to parse u32")))
        .map(|(w, h)| (w, h))
        .then_ignore(just(": "))
}

fn required_blocks<'a>() -> impl Parser<'a, &'a str, Vec<u32>, R<'a>> {
    text::int(10)
        .map(|s: &str| s.parse::<u32>().expect("Failed to parse u32"))
        .separated_by(just(" "))
        .collect()
        .then_ignore(text::newline().repeated())
}

fn region<'a>() -> impl Parser<'a, &'a str, Region, R<'a>> {
    size()
        .then(required_blocks())
        .map(|((width, height), required)| Region {
            width,
            height,
            required,
        })
}

#[must_use]
pub fn blocks_and_regions<'a>()
-> impl Parser<'a, &'a str, (HashMap<usize, Block>, Vec<Region>), R<'a>> {
    blocks()
        .then(region().repeated().collect())
        .map(|(b, r)| (b, r))
}
