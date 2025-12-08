#[must_use]
pub fn task(input: &str) -> Option<String> {
    let boxes: Vec<(f64, f64, f64)> = input
        .lines()
        .filter_map(|line| {
            let nums: Vec<f64> = line.split(',').filter_map(|num| num.parse().ok()).collect();
            match nums.as_slice() {
                [l, w, h] => Some((*l, *w, *h)),
                _ => None,
            }
        })
        .collect();

    let mut in_circuit: Vec<Option<usize>> = vec![None; boxes.len()];
    let mut circuit_sizes: Vec<usize> = Vec::new();
    let mut current: usize = 0;
    let pairs = closest_pairs(&boxes);
    for (b1, b2) in pairs {
        match (in_circuit[b1], in_circuit[b2]) {
            (None, None) => {
                in_circuit[b1] = Some(current);
                in_circuit[b2] = Some(current);
                circuit_sizes.push(2);
                current += 1;
            }
            (None, Some(c)) | (Some(c), None) => {
                in_circuit[b1] = Some(c);
                in_circuit[b2] = Some(c);
                circuit_sizes[c] += 1;
            }
            (Some(c1), Some(c2)) if c1 != c2 => {
                // Merge circuits
                let size_c2 = circuit_sizes[c2];
                circuit_sizes[c1] += size_c2;
                circuit_sizes[c2] = 0;
                for c in in_circuit.iter_mut().flatten() {
                    *c = if *c == c2 { c1 } else { *c };
                }
            }
            (Some(_), Some(_)) => { /* both already in the same circuit */ }
        }
        if let Some(first) = in_circuit[b1]
            && in_circuit.iter().all(|c| *c == Some(first))
        {
            let (b1_pos, b2_pos) = (boxes[b1], boxes[b2]);
            return Some((b1_pos.0 * b2_pos.0).to_string());
        }
    }
    None
}

fn closest_pairs(boxes: &[(f64, f64, f64)]) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for (i, (x1, y1, z1)) in boxes.iter().enumerate() {
        for (j, (x2, y2, z2)) in boxes.iter().enumerate().skip(i + 1) {
            // Euclidean distance x^2 + y^2 + z^2
            let distance = (z2 - z1)
                .mul_add(z2 - z1, (x2 - x1).mul_add(x2 - x1, (y2 - y1).powi(2)))
                .sqrt();
            pairs.push((distance, (i, j)));
        }
    }
    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    pairs.into_iter().map(|(_, pair)| pair).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "25272");
    }
}
