use std::char;

pub struct Day3 {
    pub part1: i32,
    pub part2: i32,
}

fn scan_program(input: String, with_do_dont: bool) -> i32 {
    let mut sum = 0;
    let mut last_char: Option<char> = None;
    let mut is_enabled = true;
    let mut left: Option<String> = None;
    let mut right: Option<String> = None;

    let mut skip = 0;

    for (i, c) in input.chars().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }

        if c == 'd' && with_do_dont {
            if input[i..i + 4].to_owned() == "do()" {
                is_enabled = true;
                skip = 3;
            } else if input[i..i + 7].to_owned() == "don't()" {
                is_enabled = false;
                skip = 6;
            }
        }
        if !is_enabled {
            continue;
        }
        if c == 'm' && input[i..i + 4].to_owned() == "mul(" {
            last_char = Some('(');
            left = None;
            right = None;
            skip = 3;
        } else if let Some(l) = last_char {
            if l == '(' {
                if c == ',' && left.is_some() {
                    last_char = Some(',');
                } else if c.is_digit(10) {
                    left = Some(format!("{}{}", left.unwrap_or("".to_owned()), c).to_owned());
                } else {
                    last_char = None;
                }
            } else if l == ',' {
                if c == ')' {
                    match (left.clone(), right.clone()) {
                        (Some(lv), Some(rv)) => {
                            let value_left = str::parse::<i32>(&lv).unwrap();
                            let value_right = str::parse::<i32>(&rv).unwrap();

                            sum += value_left * value_right;
                        }
                        _ => {}
                    };
                    last_char = None;
                } else if c.is_digit(10) {
                    right = Some(format!("{}{}", right.unwrap_or("".to_owned()), c).to_owned());
                } else {
                    last_char = None;
                }
            }
        }
    }

    sum
}

pub fn day3(input: String) -> Day3 {
    let part1 = scan_program(input.to_owned(), false);
    let part2 = scan_program(input.to_owned(), true);

    Day3 { part1, part2 }
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

    #[test]
    fn gets_part2() {
        let input = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let result = day3(input.to_owned());

        assert_eq!(result.part2, 48);
    }
}
