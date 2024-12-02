pub struct Day2 {
    pub num_safe: i32,
    pub num_safe_with_dampener: i32,
}

fn get_num_safe(input: String, with_dampener: bool) -> i32 {
    input.lines().fold(0, |sum, line| {
        let is_safe = |values: Vec<i8>| -> bool {
            let first = values[0];
            let direction = if values[values.len() - 1] > first {
                1
            } else {
                -1
            };

            let is_safe = values
                .iter()
                .skip(1)
                .fold((true, first), |prev, next| {
                    if !prev.0 {
                        return (false, 0);
                    }
                    let diff = (next - prev.1) * direction;
                    if diff < 1 || diff > 3 {
                        return (false, 0);
                    }
                    (true, *next)
                })
                .0;

            is_safe
        };

        let values = line
            .split(" ")
            .map(|c| str::parse::<i8>(c).unwrap())
            .collect::<Vec<i8>>();

        if is_safe(values.clone()) {
            return sum + 1;
        }

        if !with_dampener {
            return sum;
        }

        let is_safe_with_idx_removed =
            values.iter().enumerate().fold((false, 0), |prev, (i, _)| {
                if prev.0 {
                    return prev;
                }

                let values_with_idx_removed = values
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, v)| *v)
                    .collect::<Vec<i8>>();

                if is_safe(values_with_idx_removed) {
                    (true, i)
                } else {
                    prev
                }
            });

        if is_safe_with_idx_removed.0 {
            sum + 1
        } else {
            sum
        }
    })
}

pub fn day2(input: String) -> Day2 {
    let num_safe = get_num_safe(input.to_owned(), false);
    let num_safe_with_dampener = get_num_safe(input.to_owned(), true);

    Day2 {
        num_safe,
        num_safe_with_dampener,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_safe_reports() {
        let example = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

        assert_eq!(day2(example.to_owned()).num_safe, 2);
    }

    #[test]
    fn gets_safe_reports_with_dampener() {
        let example = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

        assert_eq!(day2(example.to_owned()).num_safe_with_dampener, 4);
    }
}
