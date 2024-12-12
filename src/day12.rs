use std::collections::{HashMap, HashSet};

pub struct Day12 {
    pub part1: isize,
    pub part2: isize,
}

struct Board<'a> {
    cells: &'a Vec<Vec<char>>,
    width: isize,
    height: isize,
}

fn find_region(
    board: &Board,
    recorded: &mut HashSet<(isize, isize)>,
    region: &mut HashSet<(isize, isize)>,
    ch: &char,
    (i, j): (isize, isize),
) {
    let Board {
        cells,
        width,
        height,
    } = board;

    region.insert((i, j));
    recorded.insert((i, j));
    if j > 0 && cells[(j - 1) as usize][i as usize] == *ch && !region.contains(&(i, j - 1)) {
        find_region(board, recorded, region, ch, (i, j - 1));
    }
    if i < width - 1 && cells[j as usize][(i + 1) as usize] == *ch && !region.contains(&(i + 1, j))
    {
        find_region(board, recorded, region, ch, (i + 1, j));
    }
    if j < height - 1 && cells[(j + 1) as usize][i as usize] == *ch && !region.contains(&(i, j + 1))
    {
        find_region(board, recorded, region, ch, (i, j + 1));
    }
    if i > 0 && cells[j as usize][(i - 1) as usize] == *ch && !region.contains(&(i - 1, j)) {
        find_region(board, recorded, region, ch, (i - 1, j));
    }
}

fn part1(input: String) -> isize {
    let mut recorded = HashSet::new();
    let mut regions = vec![];

    let cells = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let height = cells.len() as isize;
    let width = cells[0].len() as isize;

    let board = Board {
        cells: &cells,
        width,
        height,
    };

    cells.iter().enumerate().for_each(|(j, line)| {
        line.iter().enumerate().for_each(|(i, ch)| {
            if recorded.contains(&(i as isize, j as isize)) {
                return;
            }

            let mut region = HashSet::new();
            find_region(
                &board,
                &mut recorded,
                &mut region,
                ch,
                (i as isize, j as isize),
            );

            regions.push((region, ch));
        });
    });

    regions.iter().fold(0, |sum, (region, ch)| {
        let area = region.len() as isize;

        let perimiter = region.iter().fold(0, |p, (i, j)| {
            let border_north = if *j == 0 || cells[(j - 1) as usize][*i as usize] != **ch {
                1
            } else {
                0
            };
            let border_east = if *i == width - 1 || cells[*j as usize][(i + 1) as usize] != **ch {
                1
            } else {
                0
            };
            let border_south = if *j == height - 1 || cells[(j + 1) as usize][*i as usize] != **ch {
                1
            } else {
                0
            };
            let border_west = if *i == 0 || cells[*j as usize][(i - 1) as usize] != **ch {
                1
            } else {
                0
            };

            p + border_north + border_east + border_south + border_west
        });

        sum + area * perimiter
    })
}

fn part2(input: String) -> isize {
    let mut recorded = HashSet::new();
    let mut regions = vec![];

    let cells = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let height = cells.len() as isize;
    let width = cells[0].len() as isize;

    let board = Board {
        cells: &cells,
        width,
        height,
    };

    cells.iter().enumerate().for_each(|(j, line)| {
        line.iter().enumerate().for_each(|(i, ch)| {
            if recorded.contains(&(i as isize, j as isize)) {
                return;
            }

            let mut region = HashSet::new();
            find_region(
                &board,
                &mut recorded,
                &mut region,
                ch,
                (i as isize, j as isize),
            );

            regions.push((region, ch));
        });
    });

    regions.iter().fold(0, |sum, (region, ch)| {
        let area = region.len() as isize;

        let mut perimiter: HashMap<(i32, isize, isize), Vec<isize>> = HashMap::new();

        region.iter().for_each(|(i, j)| {
            let border_north = *j == 0 || cells[(j - 1) as usize][*i as usize] != **ch;
            let border_east = *i == width - 1 || cells[*j as usize][(i + 1) as usize] != **ch;
            let border_south = *j == height - 1 || cells[(j + 1) as usize][*i as usize] != **ch;
            let border_west = *i == 0 || cells[*j as usize][(i - 1) as usize] != **ch;

            if border_north {
                let key = (0, j - 1, *j);
                (*perimiter.entry(key).or_insert(vec![])).push(*i);
            }
            if border_east {
                let key = (1, i + 1, *i);
                (*perimiter.entry(key).or_insert(vec![])).push(*j);
            }
            if border_south {
                let key = (0, j + 1, *j);
                (*perimiter.entry(key).or_insert(vec![])).push(*i);
            }
            if border_west {
                let key = (1, i - 1, *i);
                (*perimiter.entry(key).or_insert(vec![])).push(*j);
            }
        });

        let sides = perimiter.iter_mut().fold(0, |s0, (_, count)| {
            count.sort_by(|a, b| a.cmp(b));

            count
                .iter()
                .fold((count[0], s0), |(prev, s1), next| {
                    if *next == prev + 1 {
                        (*next, s1)
                    } else {
                        (*next, s1 + 1)
                    }
                })
                .1
        });

        sum + area * sides
    })
}

pub fn day12(input: String) -> Day12 {
    Day12 {
        part1: part1(input.clone()),
        part2: part2(input.clone()),
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

    #[test]
    fn gets_part2() {
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

        assert_eq!(result.part2, 1206);
    }
}
