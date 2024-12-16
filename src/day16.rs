use core::fmt;
use std::collections::{HashMap, HashSet};

pub struct Day16 {
    pub part1: usize,
}

const EAST: i8 = 0;
const NORTH: i8 = 1;
const WEST: i8 = 2;
const SOUTH: i8 = 3;

const EMPTY: i8 = 0;
const WALL: i8 = 1;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Position(usize, usize, i8);

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},{} ({})",
            self.0,
            self.1,
            match self.2 {
                EAST => ">",
                NORTH => "^",
                WEST => "<",
                SOUTH => "v",
                _ => panic!(),
            }
        )
    }
}

#[derive(Clone)]
struct Board {
    height: usize,
    width: usize,
    cells: Vec<Vec<i8>>,
    position: Position,
    goal: (usize, usize),
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Board:\n{}",
            (0..self.height)
                .into_iter()
                .enumerate()
                .map(|(j, _)| {
                    (0..self.width)
                        .into_iter()
                        .enumerate()
                        .map(|(i, _)| {
                            if self.position == Position(i, j, self.position.2) {
                                "S"
                            } else if self.goal == (i, j) {
                                "E"
                            } else {
                                match self.cells[j][i] {
                                    EMPTY => ".",
                                    WALL => "#",
                                    _ => panic!(),
                                }
                            }
                        })
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

fn parse_board(input: String) -> Board {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
    let cells = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '#' => WALL,
                    _ => EMPTY,
                })
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<Vec<i8>>>();
    let (position, goal) =
        input
            .lines()
            .enumerate()
            .fold((None, None), |(prev_position, prev_goal), (j, line)| {
                if prev_position.is_some() && prev_goal.is_some() {
                    (prev_position, prev_goal)
                } else {
                    line.chars()
                        .enumerate()
                        .fold((prev_position, prev_goal), |next, (i, ch)| match ch {
                            'S' => (Some(Position(i, j, EAST)), next.1),
                            'E' => (next.0, Some((i, j))),
                            _ => next,
                        })
                }
            });

    Board {
        height,
        width,
        cells,
        position: position.unwrap(),
        goal: goal.unwrap(),
    }
}

fn get_next_position(board: &Board, Position(x, y, dir): Position) -> Option<(Position, i8)> {
    match dir {
        EAST => {
            if x < board.width - 1 {
                Some((Position(x + 1, y, dir), board.cells[y][x + 1]))
            } else {
                None
            }
        }
        NORTH => {
            if y > 0 {
                Some((Position(x, y - 1, dir), board.cells[y - 1][x]))
            } else {
                None
            }
        }
        WEST => {
            if x > 0 {
                Some((Position(x - 1, y, dir), board.cells[y][x - 1]))
            } else {
                None
            }
        }
        SOUTH => {
            if y < board.height - 1 {
                Some((Position(x, y + 1, dir), board.cells[y + 1][x]))
            } else {
                None
            }
        }
        _ => panic!(),
    }
}

#[derive(Clone, Debug)]
struct StackElem {
    position: Position,
    score: usize,
    tail: HashSet<(usize, usize)>,
}

fn solve(board: &mut Board) -> usize {
    let mut solutions = vec![];

    let mut stack = vec![StackElem {
        position: board.position,
        score: 0,
        tail: HashSet::from([(board.position.0, board.position.1)]),
    }];
    let mut min_score_to: HashMap<Position, usize> = HashMap::new();

    while stack.len() != 0 {
        let next = stack.pop().unwrap();

        if let Some(min_score) = min_score_to.get_mut(&next.position) {
            if *min_score > next.score {
                *min_score = next.score;
            } else {
                continue;
            }
        } else {
            min_score_to.insert(next.position, next.score);
        }

        let filter_loops = |pos: Position| -> Option<Position> {
            if next.tail.contains(&(pos.0, pos.1)) {
                None
            } else {
                Some(pos)
            }
        };

        let linear = match get_next_position(board, next.position) {
            Some((pos, EMPTY)) => filter_loops(pos),
            _ => None,
        };

        let rotate = vec![-1, 1]
            .into_iter()
            .map(|r| {
                Position(
                    next.position.0,
                    next.position.1,
                    (next.position.2 + r + 4).rem_euclid(4),
                )
            })
            .filter(|pos| {
                match get_next_position(board, *pos) {
                    Some((pos, EMPTY)) => filter_loops(pos),
                    _ => None,
                }
                .is_some()
            })
            .collect::<Vec<Position>>();

        if linear.is_some_and(|Position(x, y, _)| x == board.goal.0 && y == board.goal.1) {
            solutions.push(next.score + 1);
        } else {
            if let Some(pos) = linear {
                let mut tail_next = next.tail.clone();
                tail_next.insert((pos.0, pos.1));
                stack.push(StackElem {
                    position: pos,
                    score: next.score + 1,
                    tail: tail_next,
                });
            }

            rotate.iter().for_each(|pos| {
                stack.push(StackElem {
                    position: *pos,
                    score: next.score + 1000,
                    tail: next.tail.clone(),
                });
            });
        }
    }

    solutions.sort();
    *solutions.first().unwrap()
}

fn part1(input: String) -> usize {
    let mut board = parse_board(input);
    solve(&mut board)
}

pub fn day16(input: String) -> Day16 {
    Day16 {
        part1: part1(input.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part1() {
        let input = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        let result = day16(input.to_owned());
        assert_eq!(result.part1, 7036);
    }
}
