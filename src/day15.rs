pub struct Day15 {
    pub part1: usize,
}

const EMPTY: i8 = 0;
const WALL: i8 = 1;
const BOX: i8 = 2;

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

fn print_board(board: &Board) {
    board.cells.iter().enumerate().for_each(|(j, row)| {
        row.iter().enumerate().for_each(|(i, ch)| {
            if i == board.robot.0 && j == board.robot.1 {
                print!("@");
            } else {
                match *ch {
                    EMPTY => print!("."),
                    WALL => print!("#"),
                    BOX => print!("O"),
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

pub fn day15(input: String) -> Day15 {
    Day15 {
        part1: part1(input.clone()),
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
}
