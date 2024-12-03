use std::char;

pub struct Day3 {
    pub part1: i32,
    pub part2: i32,
}

pub fn day3(input: String) -> Day3 {
    let part1 = input
        .chars()
        .fold(
            (0, None, None, None),
            |(sum, last_char, left, right): (i32, Option<char>, Option<String>, Option<String>),
             c| {
                if c == 'm'
                    || last_char.is_some_and(|l| {
                        (c == 'u' && l == 'm') || (c == 'l' && l == 'u') || (c == '(' && l == 'l')
                    })
                {
                    return (sum, Some(c), None, None);
                }
                if last_char.is_some_and(|l| l == '(') {
                    if c == ',' && left.is_some() {
                        return (sum, Some(','), left, None);
                    }
                    if c.is_digit(10) {
                        return (
                            sum,
                            last_char,
                            Some(format!("{}{}", left.unwrap_or("".to_owned()), c).to_owned()),
                            None,
                        );
                    }
                    return (sum, None, None, None);
                }
                if last_char.is_some_and(|l| l == ',') {
                    if c == ')' {
                        return match (left, right) {
                            (Some(l), Some(r)) => {
                                let value_left = str::parse::<i32>(&l).unwrap();
                                let value_right = str::parse::<i32>(&r).unwrap();

                                return (sum + value_left * value_right, None, None, None);
                            }
                            _ => (sum, None, None, None),
                        };
                    }

                    if c.is_digit(10) {
                        return (
                            sum,
                            last_char,
                            left,
                            Some(format!("{}{}", right.unwrap_or("".to_owned()), c).to_owned()),
                        );
                    }
                    return (sum, None, None, None);
                }
                return (sum, None, None, None);
            },
        )
        .0;

    Day3 { part1, part2: 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part1() {
        let input = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let result = day3(input.to_owned());

        assert_eq!(result.part1, 161);
    }
}
