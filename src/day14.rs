pub struct Day14 {
    pub part1: usize,
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
    type Vectors = Vec<((isize, isize), (isize, isize))>;

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

    let vectors_final = (0..100).into_iter().fold(vectors, |prev, _| tick(prev));

    let (q0, q1, q2, q3) =
        vectors_final
            .iter()
            .fold((0, 0, 0, 0), |(q0, q1, q2, q3), ((px, py), _)| {
                let left = *px < width / 2;
                let top = *py < height / 2;

                if (*px + 1) == (width + 1) / 2 || (*py + 1) == (height + 1) / 2 {
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

pub fn day14(input: Day14Input) -> Day14 {
    Day14 {
        part1: part1(input.clone()),
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

        let result = day14(input);
        assert_eq!(result.part1, 12);
    }
}
