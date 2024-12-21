use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

pub struct Day19 {
    pub part1: usize,
    pub part2: usize,
}

const WHITE: i8 = 0;
const BLUE: i8 = 1;
const BLACK: i8 = 2;
const RED: i8 = 3;
const GREEN: i8 = 4;

fn parse_color(ch: &str) -> Option<i8> {
    match ch {
        "w" => Some(WHITE),
        "u" => Some(BLUE),
        "b" => Some(BLACK),
        "r" => Some(RED),
        "g" => Some(GREEN),
        _ => None,
    }
}

fn parse_color_array(line: String) -> Vec<i8> {
    line.split("")
        .map(parse_color)
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .collect::<Vec<i8>>()
}

fn matches_start(pattern: &Vec<i8>, section: &Vec<i8>, offset: usize) -> bool {
    section.len() + offset <= pattern.len()
        && (0..section.len())
            .into_iter()
            .all(|i| section[i] == pattern[i + offset])
}

#[derive(Clone, Eq, PartialEq)]
struct Node {
    offset: usize,
    tail: Vec<usize>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.offset.cmp(&other.offset)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn construct_pattern(pattern: &Vec<i8>, sections: &Vec<Vec<i8>>) -> bool {
    let mut heap = BinaryHeap::new();
    heap.push(Node {
        offset: 0,
        tail: vec![],
    });

    let mut tried = HashSet::new();

    while let Some(Node { offset, .. }) = heap.pop() {
        if offset == pattern.len() {
            return true;
        }

        sections
            .iter()
            .filter(|p| matches_start(pattern, p, offset))
            .for_each(|p| {
                let offset_next = offset + p.len();
                if !tried.contains(&offset_next) {
                    heap.push(Node {
                        offset: offset_next,
                        tail: vec![],
                    });
                    tried.insert(offset_next);
                }
            });
    }

    false
}

fn get_num_possible_patterns_from_offset(
    pattern: &Vec<i8>,
    sections: &Vec<Vec<i8>>,
    offset: usize,
    cache: &mut HashMap<usize, usize>,
) -> usize {
    let mut result = 0;
    for p in sections
        .iter()
        .filter(|p| matches_start(pattern, p, offset))
    {
        let offset_next = offset + p.len();
        if offset_next == pattern.len() {
            result += 1
        } else if let Some(r) = cache.get(&offset_next) {
            result += r;
        } else {
            let r = get_num_possible_patterns_from_offset(pattern, sections, offset_next, cache);
            cache.insert(offset_next, r);
            result += r;
        }
    }

    result
}

fn get_num_possible_patterns(pattern: &Vec<i8>, sections: &Vec<Vec<i8>>) -> usize {
    let mut cache = HashMap::new();
    get_num_possible_patterns_from_offset(pattern, sections, 0, &mut cache)
}

fn part1(input: String) -> usize {
    let sections = input
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .map(|line| parse_color_array(line.to_owned()))
        .collect::<Vec<Vec<i8>>>();

    input
        .lines()
        .skip(2)
        .map(|line| parse_color_array(line.to_owned()))
        .filter(|pattern| construct_pattern(pattern, &sections))
        .count()
}

fn part2(input: String) -> usize {
    let sections = input
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .map(|line| parse_color_array(line.to_owned()))
        .collect::<Vec<Vec<i8>>>();

    input
        .lines()
        .skip(2)
        .map(|line| parse_color_array(line.to_owned()))
        .map(|pattern| get_num_possible_patterns(&pattern, &sections))
        .fold(0, |sum, n| sum + n)
}

pub fn day19(input: String) -> Day19 {
    Day19 {
        part1: part1(input.clone()),
        part2: part2(input.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part1() {
        let input = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        let result = part1(input.to_owned());
        assert_eq!(result, 6);
    }

    #[test]
    fn gets_part2() {
        let input = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        let result = part2(input.to_owned());
        assert_eq!(result, 16);
    }
}
