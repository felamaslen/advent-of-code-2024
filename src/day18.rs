use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    usize,
};

pub struct Day18 {
    pub part1: usize,
    pub part2: String,
}

#[derive(Clone, Debug)]
pub struct InputDay18 {
    pub bytes: String,
    pub grid_size: usize,
    pub falls: usize,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Node {
    p: usize,
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.p.cmp(&other.p))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(grid_size: usize, corrupted_bytes: &HashSet<usize>) -> Option<usize> {
    // Djikstra's algorithm
    let mut heap = BinaryHeap::new();
    heap.push(Node { p: 0, cost: 0 });

    let goal = grid_size.pow(2) - 1;

    let mut dist = (0..goal + 1)
        .into_iter()
        .map(|_| usize::MAX)
        .collect::<Vec<usize>>();
    dist[0] = 0;

    while let Some(Node { p, cost }) = heap.pop() {
        let px = p.rem_euclid(grid_size);
        if p == goal {
            return Some(cost);
        };

        if cost > dist[p] {
            continue;
        }

        for n in [
            if px < grid_size - 1 {
                // east
                Some(p + 1)
            } else {
                None
            },
            if p < goal - grid_size {
                // south
                Some(p + grid_size)
            } else {
                None
            },
            if px > 0 {
                // west
                Some(p - 1)
            } else {
                None
            },
            if p > grid_size - 1 {
                // north
                Some(p - grid_size)
            } else {
                None
            },
        ]
        .iter()
        .filter(|n| n.is_some_and(|q| !corrupted_bytes.contains(&q)))
        {
            let next = Node {
                p: n.unwrap(),
                cost: cost + 1,
            };
            if next.cost < dist[next.p] {
                heap.push(next);
                dist[next.p] = next.cost;
            }
        }
    }

    None
}

fn part1(input: InputDay18) -> usize {
    let InputDay18 {
        grid_size, falls, ..
    } = input;

    let corrupted_bytes = input
        .bytes
        .lines()
        .take(falls)
        .fold(HashSet::new(), |mut h, line| {
            let (xs, ys) = line.split_once(",").unwrap();
            let x = str::parse::<usize>(xs).unwrap();
            let y = str::parse::<usize>(ys).unwrap();
            h.insert(y * grid_size + x);
            h
        });

    shortest_path(grid_size, &corrupted_bytes).expect("No shortest path")
}

fn part2(input: InputDay18) -> String {
    let InputDay18 {
        grid_size, falls, ..
    } = input;

    let parse_byte = |line: &str| -> (usize, usize) {
        let (xs, ys) = line.split_once(",").unwrap();
        let x = str::parse::<usize>(xs).unwrap();
        let y = str::parse::<usize>(ys).unwrap();
        (x, y)
    };

    let mut corrupted_bytes = HashSet::new();
    for line in input.bytes.lines().take(falls) {
        let (x, y) = parse_byte(line);
        corrupted_bytes.insert(y * grid_size + x);
    }

    let mut remaining_bytes = input.bytes.lines().skip(falls);
    let mut last_fallen_byte: Option<(usize, usize)> = None;
    while let Some(fallen_byte) = remaining_bytes.next() {
        let (x, y) = parse_byte(fallen_byte);
        last_fallen_byte = Some((x, y));
        corrupted_bytes.insert(y * grid_size + x);

        if shortest_path(grid_size, &corrupted_bytes).is_none() {
            break;
        }
    }

    let (bx, by) = last_fallen_byte.expect("Never blocked");

    format!("{},{}", bx, by)
}

pub fn day18(input: InputDay18) -> Day18 {
    Day18 {
        part1: part1(input.clone()),
        part2: part2(input.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part1() {
        let input = InputDay18 {
            bytes: r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
            .to_owned(),
            grid_size: 7,
            falls: 12,
        };
        let result = part1(input);
        assert_eq!(result, 22);
    }

    #[test]
    fn gets_part2() {
        let input = InputDay18 {
            bytes: r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
            .to_owned(),
            grid_size: 7,
            falls: 12,
        };
        let result = part2(input);
        assert_eq!(result, "6,1");
    }
}
