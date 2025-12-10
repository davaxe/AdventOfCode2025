use chumsky::{error::Rich, extra::Full, prelude::*};

use crate::{Button, Light, Machine};

type R<'a> = Full<Rich<'a, char>, (), ()>;

// -----------------------------------------------------------------------------
// Parsers
// -----------------------------------------------------------------------------

fn light<'a>() -> impl Parser<'a, &'a str, Light, R<'a>> {
    just('#')
        .to(Light::On)
        .or(just('.').to(Light::Off))
        .labelled("light symbol (# or .)")
}

fn lights<'a>() -> impl Parser<'a, &'a str, Vec<Light>, R<'a>> {
    just('[')
        .ignore_then(light().repeated().at_least(1).collect())
        .then_ignore(just(']'))
        .labelled("light pattern [.#.#]")
}

fn numbers<'a>(first: char, last: char, delim: char) -> impl Parser<'a, &'a str, Vec<u32>, R<'a>> {
    just(first)
        .ignore_then(
            text::int(10)
                .try_map(|s: &str, span| {
                    s.parse::<u32>()
                        .map_err(|_| Rich::custom(span, "invalid integer"))
                })
                .separated_by(just(delim))
                .collect(),
        )
        .then_ignore(just(last))
        .labelled("number list")
}

fn button<'a>() -> impl Parser<'a, &'a str, Button, R<'a>> {
    numbers('(', ')', ',')
        .map(|connections| Button {
            connections: connections.into_iter().map(|n| n as usize).collect(),
        })
        .labelled("button [1,2,3]")
}

fn requirements<'a>() -> impl Parser<'a, &'a str, Vec<u32>, R<'a>> {
    numbers('{', '}', ',').labelled("requirements {1,2,3}")
}

fn machine<'a>() -> impl Parser<'a, &'a str, Machine, R<'a>> {
    lights()
        .then_ignore(text::whitespace())
        .then(
            button()
                .separated_by(text::whitespace().at_least(1))
                .collect(),
        )
        .then_ignore(text::whitespace().at_least(1))
        .then(requirements())
        .map(|((target_lights, buttons), requirements)| Machine {
            lights: vec![Light::Off; target_lights.len()],
            target: target_lights,
            buttons,
            requirements,
        })
        .labelled("machine")
}

#[must_use]
pub fn machines<'a>() -> impl Parser<'a, &'a str, Vec<Machine>, R<'a>> {
    machine()
        .separated_by(text::newline())
        .collect()
        .then_ignore(text::newline().repeated())
        .labelled("machines")
}
