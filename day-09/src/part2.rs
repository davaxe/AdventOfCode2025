#![allow(clippy::cast_precision_loss)]

use itertools::Itertools;

struct Edge {
    from: (i64, i64),
    to: (i64, i64),
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Rect {
    fn from_points((p1x, p1y): (i64, i64), (p2x, p2y): (i64, i64)) -> Self {
        Self {
            min_x: i64::min(p1x, p2x),
            max_x: i64::max(p1x, p2x),
            min_y: i64::min(p1y, p2y),
            max_y: i64::max(p1y, p2y),
        }
    }

    fn area(&self) -> i64 {
        // +1 to include boundary cells
        (self.max_x - self.min_x + 1) * (self.max_y - self.min_y + 1)
    }

    /// Checks if a vertical or horizontal segment strictly intersects the interior of the rect
    fn intersects_segment_interior(&self, p1: (i64, i64), p2: (i64, i64)) -> bool {
        match (p1, p2) {
            // Vertical case: X is equal
            ((x1, y1), (x2, y2)) if x1 == x2 => {
                // Check if X is strictly within horizontal bounds
                if x1 <= self.min_x || x1 >= self.max_x {
                    return false;
                }
                // Check if Y segments overlap
                let seg_min = y1.min(y2);
                let seg_max = y1.max(y2);
                seg_min.max(self.min_y) < seg_max.min(self.max_y)
            }
            // Horizontal case: Y is equal
            ((x1, y1), (x2, y2)) if y1 == y2 => {
                // Check if Y is strictly within vertical bounds
                if y1 <= self.min_y || y1 >= self.max_y {
                    return false;
                }

                // Check if X segments overlap
                let seg_min = x1.min(x2);
                let seg_max = x1.max(x2);
                seg_min.max(self.min_x) < seg_max.min(self.max_x)
            }
            _ => false,
        }
    }
}

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let vertices: Vec<(i64, i64)> = input
        .lines()
        .filter_map(|line| {
            let (x, y) = line.split_once(',')?;
            Some((x.parse::<i64>().ok()?, y.parse::<i64>().ok()?))
        })
        .collect();

    let mut edges: Vec<Edge> = Vec::with_capacity(vertices.len());
    for i in 0..vertices.len() {
        edges.push(Edge {
            from: vertices[i],
            to: vertices[(i + 1) % vertices.len()],
        });
    }

    let res: i64 = vertices
        .iter()
        .combinations(2)
        .filter_map(|a| match a.as_slice() {
            &[p1, p2] => Some(Rect::from_points(*p1, *p2)),
            _ => None,
        })
        .fold(0, |area, rect| max_area(area, &edges, &rect));

    Some(res.to_string())
}

fn max_area(best_area: i64, edges: &[Edge], rect: &Rect) -> i64 {
    let current_area = rect.area();

    // 1. Optimization: Prune if this rect isn't better than what we have
    if current_area <= best_area {
        return best_area;
    }

    // 2. Validity Check: Ensure no edge cuts through the rectangle
    // We use .any() for a cleaner short-circuit check
    if edges
        .iter()
        .any(|e| rect.intersects_segment_interior(e.from, e.to))
    {
        return best_area;
    }

    let center_y = rect.min_y + rect.max_y;
    let center_x = rect.min_x + rect.max_x;

    let mut intersections = 0;

    for edge in edges {
        let (p1, p2) = (edge.from, edge.to);

        let y1_2 = p1.1 * 2;
        let y2_2 = p2.1 * 2;

        let spans_y = (y1_2 > center_y) != (y2_2 > center_y);

        if spans_y && p1.0 == p2.0 && p1.0 * 2 > center_x {
            intersections += 1;
        }
    }

    // If odd intersections, point is inside.
    if intersections % 2 == 1 {
        current_area
    } else {
        best_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "24");
    }
}
