use std::char;

pub struct Day4 {
    pub part1: usize,
    pub part2: usize,
}

const WORD: &str = "XMAS";
const WORD_LEN: usize = WORD.len();

fn word_search(input: String) -> usize {
    let line_len = input.lines().next().unwrap().len();
    let num_lines = input.lines().count();
    let word_vec = WORD.chars().collect::<Vec<char>>();

    input
        .lines()
        .enumerate()
        .fold(vec![], |acc, (i, line)| {
            let mut next = acc.clone();

            for (j, _) in line.chars().enumerate().filter(|(j, ch)| {
                *ch == word_vec[0] && !acc.iter().any(|(y, x, _)| *y == i && x == j)
            }) {
                let match_0 = j <= line_len - WORD_LEN
                    && WORD
                        .chars()
                        .enumerate()
                        .skip(1)
                        .all(|(w, wch)| line.chars().nth(j + w).unwrap() == wch);

                let match_45 = j <= line_len - WORD_LEN
                    && i >= WORD_LEN - 1
                    && WORD.chars().enumerate().skip(1).all(|(w, wch)| {
                        input
                            .lines()
                            .nth(i - w)
                            .unwrap()
                            .chars()
                            .nth(j + w)
                            .unwrap()
                            == wch
                    });

                let match_90 = i >= WORD_LEN - 1
                    && WORD.chars().enumerate().skip(1).all(|(w, wch)| {
                        input.lines().nth(i - w).unwrap().chars().nth(j).unwrap() == wch
                    });

                let match_135 = j >= WORD_LEN - 1
                    && i >= WORD_LEN - 1
                    && WORD.chars().enumerate().skip(1).all(|(w, wch)| {
                        input
                            .lines()
                            .nth(i - w)
                            .unwrap()
                            .chars()
                            .nth(j - w)
                            .unwrap()
                            == wch
                    });

                let match_180 = j >= WORD_LEN - 1
                    && WORD
                        .chars()
                        .enumerate()
                        .skip(1)
                        .all(|(w, wch)| line.chars().nth(j - w).unwrap() == wch);

                let match_225 = j >= WORD_LEN - 1
                    && i <= num_lines - WORD_LEN
                    && WORD.chars().enumerate().skip(1).all(|(w, wch)| {
                        input
                            .lines()
                            .nth(i + w)
                            .unwrap()
                            .chars()
                            .nth(j - w)
                            .unwrap()
                            == wch
                    });

                let match_270 = i <= num_lines - WORD_LEN
                    && WORD.chars().enumerate().skip(1).all(|(w, wch)| {
                        input.lines().nth(i + w).unwrap().chars().nth(j).unwrap() == wch
                    });

                let match_315 = j <= line_len - WORD_LEN
                    && i <= num_lines - WORD_LEN
                    && WORD.chars().enumerate().skip(1).all(|(w, wch)| {
                        input
                            .lines()
                            .nth(i + w)
                            .unwrap()
                            .chars()
                            .nth(j + w)
                            .unwrap()
                            == wch
                    });

                let num_matches = (if match_0 { 1 } else { 0 })
                    + (if match_45 { 1 } else { 0 })
                    + (if match_90 { 1 } else { 0 })
                    + (if match_135 { 1 } else { 0 })
                    + (if match_180 { 1 } else { 0 })
                    + (if match_225 { 1 } else { 0 })
                    + (if match_270 { 1 } else { 0 })
                    + (if match_315 { 1 } else { 0 });

                if num_matches > 0 {
                    next.push((i, j, num_matches));
                }
            }

            next
        })
        .iter()
        .fold(0, |acc, (_, _, count)| acc + count)
}

fn xmas(input: String) -> usize {
    let line_len = input.lines().next().unwrap().len();
    let num_lines = input.lines().count();

    input
        .lines()
        .enumerate()
        .filter(|(i, _)| *i > 0 && *i < num_lines - 1)
        .fold(0, |acc, (i, line)| {
            acc + line
                .chars()
                .enumerate()
                .filter(|(j, ch)| {
                    *ch == 'A'
                        && *j > 0
                        && *j < line_len - 1
                        && ((input
                            .lines()
                            .nth(i - 1)
                            .unwrap()
                            .chars()
                            .nth(j - 1)
                            .unwrap()
                            == 'M'
                            && input
                                .lines()
                                .nth(i + 1)
                                .unwrap()
                                .chars()
                                .nth(j + 1)
                                .unwrap()
                                == 'S')
                            || (input
                                .lines()
                                .nth(i - 1)
                                .unwrap()
                                .chars()
                                .nth(j - 1)
                                .unwrap()
                                == 'S'
                                && input
                                    .lines()
                                    .nth(i + 1)
                                    .unwrap()
                                    .chars()
                                    .nth(j + 1)
                                    .unwrap()
                                    == 'M'))
                        && ((input
                            .lines()
                            .nth(i - 1)
                            .unwrap()
                            .chars()
                            .nth(j + 1)
                            .unwrap()
                            == 'M'
                            && input
                                .lines()
                                .nth(i + 1)
                                .unwrap()
                                .chars()
                                .nth(j - 1)
                                .unwrap()
                                == 'S')
                            || (input
                                .lines()
                                .nth(i - 1)
                                .unwrap()
                                .chars()
                                .nth(j + 1)
                                .unwrap()
                                == 'S'
                                && input
                                    .lines()
                                    .nth(i + 1)
                                    .unwrap()
                                    .chars()
                                    .nth(j - 1)
                                    .unwrap()
                                    == 'M'))
                })
                .count()
        })
}

pub fn day4(input: String) -> Day4 {
    let part1 = word_search(input.to_owned());
    let part2 = xmas(input.to_owned());

    Day4 { part1, part2 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part1() {
        let input = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let result = day4(input.to_owned());

        assert_eq!(result.part1, 18);
    }

    #[test]
    fn gets_part2() {
        let input = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let result = day4(input.to_owned());

        assert_eq!(result.part2, 9);
    }
}
