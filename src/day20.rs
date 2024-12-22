use std::collections::HashMap;

pub struct Day20 {
    pub part1: usize,
    pub part2: usize,
}

#[derive(Clone, Debug)]
pub struct Day20Input {
    pub maze: String,
    pub cheat_threshold: usize,
}

const WALL: i8 = 0;
const TRACK: i8 = 1;
const START: i8 = 2;
const END: i8 = 3;

fn options_from_cell(width: usize, height: usize, i: usize) -> Vec<usize> {
    vec![
        // north
        if i > width - 1 { Some(i - width) } else { None },
        // east
        if i.rem_euclid(width) < width - 1 {
            Some(i + 1)
        } else {
            None
        },
        // south
        if i < (width * (height - 1)) {
            Some(i + width)
        } else {
            None
        },
        // west
        if i.rem_euclid(width) > 0 {
            Some(i - 1)
        } else {
            None
        },
    ]
    .iter()
    .filter(|o| o.is_some())
    .map(|o| o.unwrap())
    .collect::<Vec<usize>>()
}

fn parse_maze(maze: String) -> (usize, usize, Vec<i8>, Vec<usize>) {
    let width = maze.lines().next().unwrap().len();
    let height = maze.lines().count();

    let cells = maze
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '#' => WALL,
                    '.' => TRACK,
                    'S' => START,
                    'E' => END,
                    _ => panic!("Invalid character {}", ch),
                })
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<i8>>();

    let start = cells
        .iter()
        .enumerate()
        .find(|(_, cell)| **cell == START)
        .expect("Could not find start")
        .0;

    let track_crawled = false;
    let mut track = vec![start];
    let mut i = start;
    let mut prev: Option<usize> = None;
    while !track_crawled {
        let moves = options_from_cell(width, height, i);
        let options = moves
            .iter()
            .filter(|j| {
                !prev.is_some_and(|p| p == **j) && (cells[**j] == TRACK || cells[**j] == END)
            })
            .collect::<Vec<&usize>>();

        if options.len() != 1 {
            panic!("Could not crawl track");
        } else {
            track.push(options[0].clone());
            prev = Some(i);
            i = *options[0];
            if cells[*options[0]] == END {
                break;
            }
        }
    }

    (width, height, cells, track)
}

fn part1(
    Day20Input {
        maze,
        cheat_threshold,
    }: Day20Input,
) -> usize {
    let (width, height, cells, track) = parse_maze(maze);

    let length_base = track.len() - 1;

    let mut num_cheats = 0;
    let mut c = 0;
    while c < length_base - 2 {
        let track_rest = &track[c + 3..];

        options_from_cell(width, height, track[c])
            .iter()
            .filter(|j| cells[**j] == WALL)
            .for_each(|w| {
                options_from_cell(width, height, *w)
                    .iter()
                    .map(|j| {
                        track_rest
                            .iter()
                            .enumerate()
                            .find(|(d, k)| *k == j && d + 1 >= cheat_threshold)
                    })
                    .for_each(|option| {
                        if option.is_some() {
                            num_cheats += 1;
                        }
                    });
            });

        c += 1;
    }

    num_cheats
}

const MAX_CHEAT: usize = 20;

fn part2(
    Day20Input {
        maze,
        cheat_threshold,
    }: Day20Input,
) -> usize {
    let (width, _height, _cells, track) = parse_maze(maze);

    let length_base = track.len() - 1;

    let mut cheat_count = HashMap::new();
    let mut c = 0;
    while c < length_base - 1 - cheat_threshold {
        let track_rest = &track[c + cheat_threshold..];

        let x0 = track[c].rem_euclid(width);
        let y0 = track[c] / width;

        track_rest.iter().enumerate().for_each(|(d, dest)| {
            print!("\rc={}, d={}", c, d);
            let x1 = dest.rem_euclid(width);
            let y1 = dest / width;
            let manhattan_distance = x1.abs_diff(x0) + y1.abs_diff(y0);

            if manhattan_distance <= MAX_CHEAT && manhattan_distance <= d {
                let saved = cheat_threshold + d - manhattan_distance;
                if let Some(count) = cheat_count.get_mut(&saved) {
                    *count += 1;
                } else {
                    cheat_count.insert(saved, 1);
                }
            }
        });

        c += 1;
    }

    cheat_count.iter().fold(0, |sum, (_, count)| sum + count)
}

pub fn day20(input: Day20Input) -> Day20 {
    Day20 {
        part1: part1(input.clone()),
        part2: part2(input.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part1() {
        let input = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!(
            part1(Day20Input {
                maze: input.to_owned(),
                cheat_threshold: 0
            }),
            44
        );
        assert_eq!(
            part1(Day20Input {
                maze: input.to_owned(),
                cheat_threshold: 1
            }),
            44
        );
        assert_eq!(
            part1(Day20Input {
                maze: input.to_owned(),
                cheat_threshold: 2
            }),
            44
        );
        assert_eq!(
            part1(Day20Input {
                maze: input.to_owned(),
                cheat_threshold: 3
            }),
            30
        );
        assert_eq!(
            part1(Day20Input {
                maze: input.to_owned(),
                cheat_threshold: 4
            }),
            30
        );
        assert_eq!(
            part1(Day20Input {
                maze: input.to_owned(),
                cheat_threshold: 5
            }),
            16
        );
        assert_eq!(
            part1(Day20Input {
                maze: input.to_owned(),
                cheat_threshold: 6
            }),
            16
        );
        assert_eq!(
            part1(Day20Input {
                maze: input.to_owned(),
                cheat_threshold: 7
            }),
            14
        );
        assert_eq!(
            part1(Day20Input {
                maze: input.to_owned(),
                cheat_threshold: 8
            }),
            14
        );
        assert_eq!(
            part1(Day20Input {
                maze: input.to_owned(),
                cheat_threshold: 9
            }),
            10
        );
        assert_eq!(
            part1(Day20Input {
                maze: input.to_owned(),
                cheat_threshold: 10
            }),
            10
        );
        assert_eq!(
            part1(Day20Input {
                maze: input.to_owned(),
                cheat_threshold: 11
            }),
            8
        );
    }

    #[test]
    fn gets_part2() {
        let input = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!(
            part2(Day20Input {
                maze: input.to_owned(),
                cheat_threshold: 50
            }),
            32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
        );
        assert_eq!(
            part2(Day20Input {
                maze: input.to_owned(),
                cheat_threshold: 52
            }),
            31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
        );
        assert_eq!(
            part2(Day20Input {
                maze: input.to_owned(),
                cheat_threshold: 54
            }),
            29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
        );
    }
}
