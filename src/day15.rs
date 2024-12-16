pub struct Day15 {
    pub part1: usize,
    pub part2: usize,
}

const EMPTY: i8 = 0;
const WALL: i8 = 1;
const BOX: i8 = 2;

const BOX_L: i8 = 3;
const BOX_R: i8 = 4;

const UP: i8 = 0;
const RIGHT: i8 = 1;
const DOWN: i8 = 2;
const LEFT: i8 = 3;

#[derive(Clone, Debug)]
struct Board {
    height: usize,
    width: usize,
    cells: Vec<Vec<i8>>,
    robot: (usize, usize),
}

type Program = Vec<i8>;

fn parse_input(input: String) -> (Board, Program) {
    let lines_board = input.lines().take_while(|line| line.trim().len() > 0);
    let program = input
        .lines()
        .skip_while(|line| line.trim().len() > 0)
        .skip(1)
        .flat_map(|line| {
            line.trim()
                .chars()
                .map(|ch| match ch {
                    '^' => UP,
                    '>' => RIGHT,
                    'v' => DOWN,
                    '<' => LEFT,
                    _ => panic!("Invalid character {}", ch),
                })
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<i8>>();

    let height = lines_board.clone().count();
    let width = lines_board.clone().next().unwrap().chars().count();
    let mut robot = (0, 0);
    let cells = lines_board
        .enumerate()
        .map(|(j, line)| {
            line.chars()
                .enumerate()
                .map(|(i, ch)| {
                    if ch == '@' {
                        robot = (i, j);
                    }
                    match ch {
                        '#' => WALL,
                        'O' => BOX,
                        _ => EMPTY,
                    }
                })
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<Vec<i8>>>();

    let board = Board {
        height,
        width,
        cells,
        robot,
    };

    (board, program)
}

fn parse_input_wide(input: String) -> (Board, Program) {
    let lines_board = input.lines().take_while(|line| line.trim().len() > 0);
    let program = input
        .lines()
        .skip_while(|line| line.trim().len() > 0)
        .skip(1)
        .flat_map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '^' => UP,
                    '>' => RIGHT,
                    'v' => DOWN,
                    '<' => LEFT,
                    _ => panic!("Invalid character {}", ch),
                })
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<i8>>();

    let height = lines_board.clone().count();
    let width = lines_board.clone().next().unwrap().chars().count() * 2;
    let mut robot = (0, 0);
    let cells = lines_board
        .enumerate()
        .map(|(j, line)| {
            line.chars()
                .enumerate()
                .flat_map(|(i, ch)| {
                    if ch == '@' {
                        robot = (2 * i, j);
                    }
                    match ch {
                        '#' => vec![WALL, WALL],
                        'O' => vec![BOX_L, BOX_R],
                        _ => vec![EMPTY, EMPTY],
                    }
                })
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<Vec<i8>>>();

    let board = Board {
        height,
        width,
        cells,
        robot,
    };

    (board, program)
}

fn print_board(board: &Board) {
    print!("\x1B[2J\x1B[1;1H");
    board.cells.iter().enumerate().for_each(|(j, row)| {
        row.iter().enumerate().for_each(|(i, ch)| {
            if i == board.robot.0 && j == board.robot.1 {
                if *ch != EMPTY {
                    panic!("Robot on non-empty square {},{}", i, j);
                }
                print!("@");
            } else {
                match *ch {
                    EMPTY => print!("."),
                    WALL => print!("#"),
                    BOX => print!("O"),
                    BOX_L => print!("["),
                    BOX_R => print!("]"),
                    _ => panic!(),
                };
            }
        });
        print!("\n");
    })
}

fn part1(input: String) -> usize {
    let (board, program) = parse_input(input);

    let result = program.iter().fold(board, |prev, step| {
        let Board {
            height,
            width,
            cells,
            robot: (rx, ry),
        } = prev;
        let mut next = cells.clone();
        match *step {
            UP => {
                let mut j = ry - 1;
                while j > 0 && cells[j][rx] != EMPTY && cells[j][rx] != WALL {
                    j -= 1;
                }
                if cells[j][rx] == WALL {
                    return Board {
                        height,
                        width,
                        cells,
                        robot: (rx, ry),
                    };
                }
                next[ry][rx] = EMPTY;
                for k in j..(ry - 1) {
                    next[k][rx] = cells[k + 1][rx];
                }
                Board {
                    height,
                    width,
                    cells: next,
                    robot: (rx, ry - 1),
                }
            }
            RIGHT => {
                let mut i = rx + 1;
                while i < width - 1 && cells[ry][i] != EMPTY && cells[ry][i] != WALL {
                    i += 1;
                }
                if cells[ry][i] == WALL {
                    return Board {
                        height,
                        width,
                        cells,
                        robot: (rx, ry),
                    };
                }
                next[ry][rx] = EMPTY;
                for l in (rx + 1)..i {
                    next[ry][l + 1] = cells[ry][l];
                }
                Board {
                    height,
                    width,
                    cells: next,
                    robot: (rx + 1, ry),
                }
            }
            DOWN => {
                let mut j = ry + 1;
                while j < height - 1 && cells[j][rx] != EMPTY && cells[j][rx] != WALL {
                    j += 1;
                }
                if cells[j][rx] == WALL {
                    return Board {
                        height,
                        width,
                        cells,
                        robot: (rx, ry),
                    };
                }
                next[ry][rx] = EMPTY;
                for k in (ry + 1)..j {
                    next[k + 1][rx] = cells[k][rx];
                }
                Board {
                    height,
                    width,
                    cells: next,
                    robot: (rx, ry + 1),
                }
            }
            LEFT => {
                let mut i = rx - 1;
                while i > 0 && cells[ry][i] != EMPTY && cells[ry][i] != WALL {
                    i -= 1;
                }
                if cells[ry][i] == WALL {
                    return Board {
                        height,
                        width,
                        cells,
                        robot: (rx, ry),
                    };
                }
                next[ry][rx] = EMPTY;
                for l in i..(rx - 1) {
                    next[ry][l] = cells[ry][l + 1];
                }
                Board {
                    height,
                    width,
                    cells: next,
                    robot: (rx - 1, ry),
                }
            }
            _ => panic!(),
        }
    });

    print_board(&result);

    result.cells.iter().enumerate().fold(0, |sum0, (j, row)| {
        row.iter()
            .enumerate()
            .filter(|(_, cell)| **cell == BOX)
            .fold(sum0, |sum1, (i, _)| sum1 + i + 100 * j)
    })
}

fn detect_collision(
    board: &Board,
    direction: isize,
    (bx, by): (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let by_next = ((by as isize) + direction) as usize;
    if by_next == 0 || by_next == board.height - 1 {
        return None;
    }

    let left = match board.cells[by_next][bx] {
        BOX_L => detect_collision(board, direction, (bx, by_next)),
        BOX_R => detect_collision(board, direction, (bx - 1, by_next)),
        WALL => None,
        _ => Some(vec![]),
    };
    let right = match board.cells[by_next][bx + 1] {
        BOX_L => detect_collision(board, direction, (bx + 1, by_next)),
        WALL => None,
        _ => Some(vec![]),
    };

    if left.is_none() || right.is_none() {
        None
    } else {
        let mut result = vec![(bx, by)];
        let mut l = left.unwrap();
        let mut r = right.unwrap();
        result.append(&mut l);
        result.append(&mut r);
        Some(result)
    }
}

fn part2(input: String) -> usize {
    let (board, program) = parse_input_wide(input);

    print_board(&board);

    let result = program.iter().fold(board, |prev, step| {
        let Board {
            height,
            width,
            robot: (rx, ry),
            ..
        } = prev;
        let mut cells = prev.cells.clone();
        let next = match *step {
            UP => match cells[ry - 1][rx] {
                EMPTY => Board {
                    height,
                    width,
                    cells,
                    robot: (rx, ry - 1),
                },
                WALL => Board {
                    height,
                    width,
                    cells,
                    robot: (rx, ry),
                },
                BOX_L | BOX_R => {
                    let box0 = if cells[ry - 1][rx] == BOX_L {
                        (rx, ry - 1)
                    } else {
                        (rx - 1, ry - 1)
                    };

                    let collisions = detect_collision(&prev, -1, box0);

                    if let Some(boxes_to_move) = collisions {
                        boxes_to_move.iter().for_each(|(bx, by)| {
                            cells[*by][*bx] = EMPTY;
                            cells[*by][bx + 1] = EMPTY;
                        });
                        boxes_to_move.iter().for_each(|(bx, by)| {
                            cells[by - 1][*bx] = BOX_L;
                            cells[by - 1][bx + 1] = BOX_R;
                        });

                        Board {
                            height,
                            width,
                            cells,
                            robot: (rx, ry - 1),
                        }
                    } else {
                        Board {
                            height,
                            width,
                            cells,
                            robot: (rx, ry),
                        }
                    }
                }
                _ => panic!(),
            },
            RIGHT => {
                let mut i = rx + 1;
                while i < width - 1 && cells[ry][i] != EMPTY && cells[ry][i] != WALL {
                    i += 1;
                }
                if cells[ry][i] == WALL {
                    return Board {
                        height,
                        width,
                        cells,
                        robot: (rx, ry),
                    };
                }
                for l in (rx + 1)..i {
                    cells[ry][i - (l - rx - 1)] = cells[ry][i - (l - rx)];
                }
                cells[ry][rx + 1] = EMPTY;
                Board {
                    height,
                    width,
                    cells,
                    robot: (rx + 1, ry),
                }
            }
            DOWN => match cells[ry + 1][rx] {
                EMPTY => Board {
                    height,
                    width,
                    cells,
                    robot: (rx, ry + 1),
                },
                WALL => Board {
                    height,
                    width,
                    cells,
                    robot: (rx, ry),
                },
                BOX_L | BOX_R => {
                    let box0 = if cells[ry + 1][rx] == BOX_L {
                        (rx, ry + 1)
                    } else {
                        (rx - 1, ry + 1)
                    };

                    if let Some(boxes_to_move) = detect_collision(&prev, 1, box0) {
                        boxes_to_move.iter().for_each(|(bx, by)| {
                            cells[*by][*bx] = EMPTY;
                            cells[*by][bx + 1] = EMPTY;
                        });
                        boxes_to_move.iter().for_each(|(bx, by)| {
                            cells[by + 1][*bx] = BOX_L;
                            cells[by + 1][bx + 1] = BOX_R;
                        });

                        Board {
                            height,
                            width,
                            cells,
                            robot: (rx, ry + 1),
                        }
                    } else {
                        Board {
                            height,
                            width,
                            cells,
                            robot: (rx, ry),
                        }
                    }
                }
                _ => panic!(),
            },
            LEFT => {
                let mut i = rx - 1;
                while i > 0 && cells[ry][i] != EMPTY && cells[ry][i] != WALL {
                    i -= 1;
                }
                if cells[ry][i] == WALL {
                    return Board {
                        height,
                        width,
                        cells,
                        robot: (rx, ry),
                    };
                }
                cells[ry][rx] = EMPTY;
                for l in i..rx {
                    cells[ry][l] = cells[ry][l + 1];
                }
                Board {
                    height,
                    width,
                    cells,
                    robot: (rx - 1, ry),
                }
            }
            _ => panic!(),
        };

        next
    });

    print_board(&result);

    result.cells.iter().enumerate().fold(0, |sum0, (j, row)| {
        row.iter()
            .enumerate()
            .filter(|(_, cell)| **cell == BOX_L)
            .fold(sum0, |sum1, (i, _)| sum1 + i + 100 * j)
    })
}

pub fn day15(input: String) -> Day15 {
    Day15 {
        part1: part1(input.clone()),
        part2: part2(input.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part1() {
        let input = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let result = day15(input.to_owned());
        assert_eq!(result.part1, 10092);
    }

    #[test]
    fn gets_part2() {
        let input = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let result = day15(input.to_owned());
        assert_eq!(result.part2, 9021);
    }
}
