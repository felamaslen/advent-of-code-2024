pub struct Day11 {
    pub part1: usize,
}

fn part1(input: String) -> usize {
    (0..25)
        .into_iter()
        .fold(
            input
                .trim()
                .split(' ')
                .map(|ch| str::parse::<i64>(ch).unwrap())
                .collect::<Vec<i64>>(),
            |stones, _| {
                stones
                    .iter()
                    .flat_map(|stone| match stone {
                        0 => vec![1],
                        i => {
                            let number = format!("{}", i);
                            if number.len().rem_euclid(2) == 0 {
                                let (a, b) = (
                                    str::parse::<i64>(&number[..number.len() / 2]).unwrap(),
                                    str::parse::<i64>(&number[number.len() / 2..]).unwrap(),
                                );
                                vec![a, b]
                            } else {
                                vec![i * 2024]
                            }
                        }
                    })
                    .collect::<Vec<i64>>()
            },
        )
        .len()
}

pub fn day11(input: String) -> Day11 {
    Day11 {
        part1: part1(input.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part1() {
        let input = r"125 17";
        let result = day11(input.to_owned());
        assert_eq!(result.part1, 55312);
    }
}
