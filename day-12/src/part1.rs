use crate::{Block, Region, parser};
use chumsky::prelude::*;

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let (blocks, regions) = parser::blocks_and_regions()
        .parse(input)
        .into_result()
        .ok()?;

    for region in &regions {
        if !enough_space(
            region,
            region
                .required
                .iter()
                .enumerate()
                .map(|(i, r)| (*r, &blocks[&i])),
        ) {
            continue;
        }
    }

    None
}

fn enough_space<'a, I: Iterator<Item = (u32, &'a Block)>>(region: &Region, blocks: I) -> bool {
    blocks.map(|(r, b)| r * b.area).sum::<u32>() <= region.area()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "2");
    }
}
