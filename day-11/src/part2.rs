use std::collections::HashMap;

use crate::part1::Device;

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let mut devices: HashMap<&str, Device<'_>> = input
        .lines()
        .filter_map(|line| {
            let (name, connections) = line.split_once(':')?;

            Some((
                name.trim(),
                Device::new(
                    name.trim(),
                    connections.split_whitespace().map(str::trim).collect(),
                ),
            ))
        })
        .collect();

    // "out" does not appear as a device in the input, so we add it manually
    devices.insert("out", Device::new("out", Vec::new()));
    let idx_map: HashMap<&str, usize> = devices
        .keys()
        .enumerate()
        .map(|(idx, &name)| (name, idx))
        .collect();

    let required = ["dac", "fft"];

    let mut queue = vec![("svr", vec![false; devices.len()])];
    let mut paths = 0;
    while let Some((current, mut visited)) = queue.pop() {
        if current == "out" && required.iter().all(|&r| visited[idx_map[r]]) {
            println!("Found valid path to 'out' via required devices");
            paths += 1;
            continue;
        }
        visited[idx_map[current]] = true;

        if let Some(device) = devices.get(current) {
            for &conn in &device.connections {
                if !visited[idx_map[conn]] {
                    queue.push((conn, visited.clone()));
                }
            }
        }
    }
    Some(paths.to_string())
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
