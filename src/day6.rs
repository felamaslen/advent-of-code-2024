use std::collections::HashSet;

pub struct Day6 {
    pub part1: usize,
    pub part2: usize,
}

const UP: i8 = 0;
const RIGHT: i8 = 1;
const DOWN: i8 = 2;
const LEFT: i8 = 3;

fn init(input: &str) -> (usize, usize, (usize, usize), i8) {
    let pos_line = input
        .lines()
        .enumerate()
        .skip_while(|(_, line)| !line.contains('^'))
        .next()
        .unwrap();
    let pos_col = pos_line
        .1
        .chars()
        .enumerate()
        .find(|(_, ch)| *ch == '^')
        .unwrap();

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let pos = (pos_col.0, pos_line.0);
    let direction = UP;

    (height, width, pos, direction)
}

fn crawl(input: String) -> HashSet<(usize, usize)> {
    let (height, width, mut pos, mut direction) = init(&input);

    let mut positions = HashSet::new();
    positions.insert(pos);

    loop {
        let next_position = match direction {
            UP => {
                if pos.1 == 0 {
                    None
                } else {
                    Some((pos.0, pos.1 - 1))
                }
            }
            RIGHT => {
                if pos.0 == width - 1 {
                    None
                } else {
                    Some((pos.0 + 1, pos.1))
                }
            }
            DOWN => {
                if pos.1 == height - 1 {
                    None
                } else {
                    Some((pos.0, pos.1 + 1))
                }
            }
            LEFT => {
                if pos.0 == 0 {
                    None
                } else {
                    Some((pos.0 - 1, pos.1))
                }
            }
            _ => panic!("invalid direction"),
        };

        if let Some(pos_next) = next_position {
            let char_next = input
                .lines()
                .nth(pos_next.1)
                .unwrap()
                .chars()
                .nth(pos_next.0)
                .unwrap();

            if char_next == '#' {
                direction = (direction + 1) % 4;
            } else {
                positions.insert(pos_next);
                pos = pos_next;
            }
        } else {
            break;
        }
    }

    positions
}

fn part1(input: String) -> usize {
    crawl(input).len()
}

fn part2(input: String) -> usize {
    let visited_init = crawl(input.clone());

    let (height, width, position, direction) = init(&input);

    let check_obstruction = |obstruction: (usize, usize)| {
        let mut vector = (position.0, position.1, direction);
        let mut visited = HashSet::new();

        visited.insert(vector);

        let mut is_obstructed = false;

        loop {
            let next_position = match vector.2 {
                UP => {
                    if vector.1 == 0 {
                        None
                    } else {
                        Some((vector.0, vector.1 - 1))
                    }
                }
                RIGHT => {
                    if vector.0 == width - 1 {
                        None
                    } else {
                        Some((vector.0 + 1, vector.1))
                    }
                }
                DOWN => {
                    if vector.1 == height - 1 {
                        None
                    } else {
                        Some((vector.0, vector.1 + 1))
                    }
                }
                LEFT => {
                    if vector.0 == 0 {
                        None
                    } else {
                        Some((vector.0 - 1, vector.1))
                    }
                }
                _ => panic!("invalid direction"),
            };

            if let Some(pos_next) = next_position {
                let char_next = input
                    .lines()
                    .nth(pos_next.1)
                    .unwrap()
                    .chars()
                    .nth(pos_next.0)
                    .unwrap();

                let vector_next = (pos_next.0, pos_next.1, vector.2);

                if pos_next == obstruction || char_next == '#' {
                    vector.2 = (vector.2 + 1) % 4;
                } else if visited.contains(&vector_next) {
                    is_obstructed = true;
                    break;
                } else {
                    vector = vector_next;
                    visited.insert(vector);
                }
            } else {
                break;
            }
        }

        is_obstructed
    };

    visited_init.iter().skip(1).fold(0, |sum, position| {
        if check_obstruction(*position) {
            sum + 1
        } else {
            sum
        }
    })
}

pub fn day6(input: String) -> Day6 {
    Day6 {
        part1: part1(input.clone()),
        part2: part2(input.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part1() {
        let input = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let result = day6(input.to_owned());

        assert_eq!(result.part1, 41);
    }

    #[test]
    fn gets_part2() {
        let input = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let result = day6(input.to_owned());

        assert_eq!(result.part2, 6);
    }
}
