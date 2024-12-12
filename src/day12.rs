use std::collections::HashSet;

pub struct Day12 {
    pub part1: usize,
    pub part2: usize,
}

struct Board<'a> {
    cells: &'a Vec<Vec<char>>,
    width: usize,
    height: usize,
}

fn find_region(
    board: &Board,
    recorded: &mut HashSet<(usize, usize)>,
    region: &mut HashSet<(usize, usize)>,
    ch: &char,
    (i, j): (usize, usize),
) {
    let Board {
        cells,
        width,
        height,
    } = board;

    region.insert((i, j));
    recorded.insert((i, j));
    if j > 0 && cells[j - 1][i] == *ch && !region.contains(&(i, j - 1)) {
        find_region(board, recorded, region, ch, (i, j - 1));
    }
    if i < width - 1 && cells[j][i + 1] == *ch && !region.contains(&(i + 1, j)) {
        find_region(board, recorded, region, ch, (i + 1, j));
    }
    if j < height - 1 && cells[j + 1][i] == *ch && !region.contains(&(i, j + 1)) {
        find_region(board, recorded, region, ch, (i, j + 1));
    }
    if i > 0 && cells[j][i - 1] == *ch && !region.contains(&(i - 1, j)) {
        find_region(board, recorded, region, ch, (i - 1, j));
    }
}

fn part1(input: String) -> usize {
    let mut recorded = HashSet::new();
    let mut regions = vec![];

    let cells = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let height = cells.len();
    let width = cells[0].len();

    let board = Board {
        cells: &cells,
        width,
        height,
    };

    cells.iter().enumerate().for_each(|(j, line)| {
        line.iter().enumerate().for_each(|(i, ch)| {
            if recorded.contains(&(i, j)) {
                return;
            }

            let mut region = HashSet::new();
            find_region(&board, &mut recorded, &mut region, ch, (i, j));

            regions.push((region, ch));
        });
    });

    regions.iter().fold(0, |sum, (region, ch)| {
        let area = region.len();

        let perimiter = region.iter().fold(0, |p, (i, j)| {
            let border_north = if *j == 0 || cells[j - 1][*i] != **ch {
                1
            } else {
                0
            };
            let border_east = if *i == width - 1 || cells[*j][i + 1] != **ch {
                1
            } else {
                0
            };
            let border_south = if *j == height - 1 || cells[j + 1][*i] != **ch {
                1
            } else {
                0
            };
            let border_west = if *i == 0 || cells[*j][i - 1] != **ch {
                1
            } else {
                0
            };

            p + border_north + border_east + border_south + border_west
        });

        sum + area * perimiter
    })
}

pub fn day12(input: String) -> Day12 {
    Day12 {
        part1: part1(input.clone()),
        part2: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part1() {
        let input = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let result = day12(input.to_owned());

        assert_eq!(result.part1, 1930);
    }
}
