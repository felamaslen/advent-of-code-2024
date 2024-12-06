use std::collections::HashSet;

pub struct Day6 {
    pub part1: usize,
    pub part2: i16,
}

const UP: i8 = 0;
const RIGHT: i8 = 1;
const DOWN: i8 = 2;
const LEFT: i8 = 3;

pub fn day6(input: String) -> Day6 {
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

    let mut pos = (pos_col.0, pos_line.0);
    let mut direction = UP;

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

    let part1 = positions.len();

    Day6 { part1, part2: 0 }
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
}
