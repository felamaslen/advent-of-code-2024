pub struct Day20 {
    pub part1: usize,
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
        if i < (width * height - 1) {
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

fn part1(
    Day20Input {
        maze,
        cheat_threshold,
    }: Day20Input,
) -> usize {
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
    let end = cells
        .iter()
        .enumerate()
        .find(|(_, cell)| **cell == END)
        .expect("Could not find end")
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

pub fn day20(input: Day20Input) -> Day20 {
    Day20 {
        part1: part1(input.clone()),
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
}
