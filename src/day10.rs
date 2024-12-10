use std::collections::HashSet;

pub struct Day10 {
    pub part1: usize,
    pub part2: usize,
}

struct Board<'a> {
    cells: &'a Vec<Vec<usize>>,
    height: usize,
    width: usize,
}

const EAST: i8 = 0;
const SOUTH: i8 = 1;
const WEST: i8 = 2;
const NORTH: i8 = 3;

fn options_from_point(
    board: &Board,
    point: &(usize, usize),
    direction: i8,
    value: usize,
) -> Vec<(i8, (usize, usize))> {
    let east = if direction != WEST
        && point.0 < board.width - 1
        && board.cells[point.1][point.0 + 1] == value + 1
    {
        Some((point.0 + 1, point.1))
    } else {
        None
    };
    let south = if direction != NORTH
        && point.1 < board.height - 1
        && board.cells[point.1 + 1][point.0] == value + 1
    {
        Some((point.0, point.1 + 1))
    } else {
        None
    };
    let west = if direction != EAST && point.0 > 0 && board.cells[point.1][point.0 - 1] == value + 1
    {
        Some((point.0 - 1, point.1))
    } else {
        None
    };
    let north =
        if direction != SOUTH && point.1 > 0 && board.cells[point.1 - 1][point.0] == value + 1 {
            Some((point.0, point.1 - 1))
        } else {
            None
        };

    vec![east, south, west, north]
        .iter()
        .enumerate()
        .filter(|(_, option)| option.is_some())
        .map(|(direction, option)| (direction as i8, option.unwrap()))
        .collect::<Vec<(i8, (usize, usize))>>()
}

fn trails_from_point(
    board: &Board,
    point: &(usize, usize),
    direction: i8,
    tail: Vec<(usize, usize)>,
) -> Vec<Vec<(usize, usize)>> {
    let value = board.cells[point.1][point.0];

    if value == 9 {
        return vec![tail];
    }

    let options = options_from_point(board, point, direction, value);

    let trails = options
        .iter()
        .flat_map(|(direction_next, option)| {
            let mut trail = tail.clone();
            trail.push(*option);

            trails_from_point(board, option, *direction_next, trail)
        })
        .filter(|trail| trail.len() == 10)
        .collect::<Vec<Vec<(usize, usize)>>>();

    trails
}

fn part1(input: String) -> usize {
    let cells = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| str::parse::<usize>(ch.to_string().as_str()).unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let height = cells.len();
    let width = cells[0].len();

    let board = Board {
        height,
        width,
        cells: &cells,
    };

    let zeroes = cells
        .iter()
        .enumerate()
        .flat_map(|(j, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, v)| **v == 0)
                .map(move |(i, _)| (i, j))
        })
        .collect::<Vec<(usize, usize)>>();

    zeroes.iter().fold(0, |score, zero| {
        let trails = trails_from_point(&board, zero, 4, vec![*zero]);

        let mut unique_trails = HashSet::new();
        trails.iter().for_each(|trail| {
            unique_trails.insert(trail[trail.len() - 1]);
        });

        score + unique_trails.len()
    })
}

fn part2(input: String) -> usize {
    let cells = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| str::parse::<usize>(ch.to_string().as_str()).unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let height = cells.len();
    let width = cells[0].len();

    let board = Board {
        height,
        width,
        cells: &cells,
    };

    let zeroes = cells
        .iter()
        .enumerate()
        .flat_map(|(j, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, v)| **v == 0)
                .map(move |(i, _)| (i, j))
        })
        .collect::<Vec<(usize, usize)>>();

    zeroes.iter().fold(0, |score, zero| {
        let trails = trails_from_point(&board, zero, 4, vec![*zero]);

        score + trails.len()
    })
}

pub fn day10(input: String) -> Day10 {
    Day10 {
        part1: part1(input.clone()),
        part2: part2(input.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part1() {
        let input = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        let result = day10(input.to_owned());

        assert_eq!(result.part1, 36);
    }

    #[test]
    fn gets_part2() {
        let input = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        let result = day10(input.to_owned());

        assert_eq!(result.part2, 81);
    }
}
