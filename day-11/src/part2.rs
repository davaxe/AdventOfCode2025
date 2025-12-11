use std::collections::HashMap;

use crate::part1;

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let mut devices: HashMap<&str, part1::Device<'_>> = input
        .lines()
        .filter_map(|line| {
            let (name, connections) = line.split_once(':')?;

            Some((
                name.trim(),
                part1::Device::new(
                    name.trim(),
                    connections.split_whitespace().map(str::trim).collect(),
                ),
            ))
        })
        .collect();

    // "out" does not appear as a device in the input, so we add it manually
    devices.insert("out", part1::Device::new("out", Vec::new()));
    let count_chain = |nodes: &[&str]| -> usize {
        nodes
            .windows(2)
            .map(|pair| part1::paths_between_devices(&devices, pair[0], pair[1]))
            .product()
    };

    Some(
        (count_chain(&["svr", "dac", "fft", "out"]) + count_chain(&["svr", "fft", "dac", "out"]))
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example_part2.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "2");
    }
}
