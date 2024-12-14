use std::collections::HashMap;

pub struct Day14 {
    pub part1: usize,
    pub part2: usize,
}

#[derive(Clone)]
pub struct Day14Input<'a> {
    pub robots: &'a str,
    pub width: isize,
    pub height: isize,
}

fn part1(
    Day14Input {
        robots,
        width,
        height,
    }: Day14Input,
) -> usize {
    let (q0, q1, q2, q3) = robots
        .lines()
        .map(|line| {
            let (position, velocity) = line.split_once(' ').unwrap();
            let (px_str, py_str) = position.split_once('=').unwrap().1.split_once(',').unwrap();
            let px = str::parse::<isize>(px_str).unwrap();
            let py = str::parse::<isize>(py_str).unwrap();

            let (vx_str, vy_str) = velocity
                .trim_end()
                .split_once('=')
                .unwrap()
                .1
                .split_once(',')
                .unwrap();
            let vx = str::parse::<isize>(vx_str).unwrap();
            let vy = str::parse::<isize>(vy_str).unwrap();

            ((px, py), (vx, vy))
        })
        .map(|((px, py), (vx, vy))| {
            (
                (px + vx * 100).rem_euclid(width),
                (py + vy * 100).rem_euclid(height),
            )
        })
        .fold((0, 0, 0, 0), |(q0, q1, q2, q3), (px, py)| {
            let left = px < width / 2;
            let top = py < height / 2;

            if (px + 1) == (width + 1) / 2 || (py + 1) == (height + 1) / 2 {
                (q0, q1, q2, q3)
            } else if top {
                if left {
                    (q0 + 1, q1, q2, q3)
                } else {
                    (q0, q1 + 1, q2, q3)
                }
            } else if left {
                (q0, q1, q2 + 1, q3)
            } else {
                (q0, q1, q2, q3 + 1)
            }
        });

    q0 * q1 * q2 * q3
}

type Vectors = Vec<((isize, isize), (isize, isize))>;

fn print_board(Day14Input { width, height, .. }: &Day14Input<'_>, vectors: &Vectors) {
    for j in (0..*height).into_iter() {
        println!(
            "{}",
            (0..*width)
                .enumerate()
                .map(|(i, _)| {
                    let num_robots = vectors
                        .iter()
                        .filter(|((px, py), _)| *px == i as isize && *py == j)
                        .count();

                    if num_robots == 0 {
                        format!(".")
                    } else {
                        format!("o")
                        // format!("{}", num_robots)
                    }
                })
                .collect::<Vec<String>>()
                .join("")
        );
    }
}

fn is_christmas_tree(vectors: &Vectors) -> bool {
    let mut rows: HashMap<&isize, Vec<&isize>> = HashMap::new();
    vectors.iter().for_each(|((px, py), _)| {
        if let Some(row) = rows.get_mut(py) {
            row.push(px);
        } else {
            rows.insert(py, vec![px]);
        }
    });

    let mut row_sizes: HashMap<isize, isize> = HashMap::new();

    rows.iter_mut().for_each(|(_py, row)| {
        row.sort();
        row.iter()
            .skip(1)
            .fold((1, row.first().unwrap()), |(length, tx), x| {
                if **x > (**tx) + 1 {
                    if length > 2 {
                        if let Some(rec) = row_sizes.get_mut(&length) {
                            *rec += 1;
                        } else {
                            row_sizes.insert(length, 1);
                        }
                    }
                    (1, x)
                } else if **x == **tx + 1 {
                    (length + 1, x)
                } else {
                    (length, x)
                }
            });
    });

    row_sizes.len() > 3
}

fn part2(input: Day14Input) -> usize {
    let Day14Input {
        robots,
        width,
        height,
    } = input;
    let vectors = robots
        .lines()
        .map(|line| {
            let (position, velocity) = line.split_once(' ').unwrap();
            let (px_str, py_str) = position.split_once('=').unwrap().1.split_once(',').unwrap();
            let px = str::parse::<isize>(px_str).unwrap();
            let py = str::parse::<isize>(py_str).unwrap();

            let (vx_str, vy_str) = velocity
                .trim_end()
                .split_once('=')
                .unwrap()
                .1
                .split_once(',')
                .unwrap();
            let vx = str::parse::<isize>(vx_str).unwrap();
            let vy = str::parse::<isize>(vy_str).unwrap();

            ((px, py), (vx, vy))
        })
        .collect::<Vectors>();

    let tick = |prev_vectors: Vectors| -> Vectors {
        prev_vectors
            .iter()
            .map(|((px, py), (vx, vy))| {
                (
                    (
                        (px + vx + width).rem_euclid(width),
                        (py + vy + height).rem_euclid(height),
                    ),
                    (*vx, *vy),
                )
            })
            .collect::<Vectors>()
    };

    let mut next = vectors;
    for i in (0..100000).into_iter() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        next = tick(next);
        println!("i={}", i);
        if is_christmas_tree(&next) {
            print_board(&input, &next);
            return i + 1;
        }
    }

    panic!("Could not find christmas tree");
}

pub fn day14(input: Day14Input) -> Day14 {
    Day14 {
        part1: part1(input.clone()),
        part2: part2(input.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part1() {
        let input = Day14Input {
            robots: r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
            width: 11,
            height: 7,
        };

        let result = part1(input);
        assert_eq!(result, 12);
    }
}
