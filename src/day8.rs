use std::collections::HashSet;

pub struct Day8 {
    pub part1: usize,
}

fn get_unique_antinodes(input: String) -> usize {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    input
        .lines()
        .enumerate()
        .fold(HashSet::new(), |sum0, (i, line)| {
            let y0 = i as isize;
            line.chars()
                .enumerate()
                .fold(sum0, |mut sum1, (j, ch)| match ch {
                    '.' => sum1,
                    a => {
                        let x0 = j as isize;
                        input.lines().enumerate().for_each(|(k, line1)| {
                            let y1 = k as isize;

                            line1
                                .chars()
                                .enumerate()
                                .filter(|(l, b)| *b == a && !(k == i && *l == j))
                                .for_each(|(l, _)| {
                                    let x1 = l as isize;

                                    let dy = y0 - y1;
                                    let dx = x0 - x1;

                                    let antinode_a = (x0 + dx, y0 + dy);
                                    let antinode_b = (x1 - dx, y1 - dy);

                                    if antinode_a.0 >= 0
                                        && antinode_a.0 < width as isize
                                        && antinode_a.1 >= 0
                                        && antinode_a.1 < height as isize
                                    {
                                        sum1.insert(antinode_a);
                                    }
                                    if antinode_b.0 >= 0
                                        && antinode_b.0 < width as isize
                                        && antinode_b.1 >= 0
                                        && antinode_b.1 < height as isize
                                    {
                                        sum1.insert(antinode_b);
                                    }
                                });
                        });

                        sum1
                    }
                })
        })
        .len()
}

pub fn day8(input: String) -> Day8 {
    let part1 = get_unique_antinodes(input);
    Day8 { part1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part1() {
        let input = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let result = day8(input.to_owned());

        assert_eq!(result.part1, 14);
    }
}
