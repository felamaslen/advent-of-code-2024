pub fn day2(input: String) -> i32 {
    input.lines().fold(0, |sum, line| {
        let mut values = line.split(" ").map(|c| str::parse::<i8>(c).unwrap());

        let first = values.next().unwrap();
        let second = values.next().unwrap();

        let direction = if second > first {
            1
        } else if second < first {
            -1
        } else {
            0
        };

        let diff = (second - first) * direction;
        if diff < 1 || diff > 3 {
            return sum;
        }

        if direction == 0 {
            sum
        } else if values
            .fold((true, second), |prev, next| {
                if !prev.0 {
                    return (false, 0);
                }
                let diff = (next - prev.1) * direction;
                if diff < 1 || diff > 3 {
                    return (false, 0);
                }
                (true, next)
            })
            .0
        {
            sum + 1
        } else {
            sum
        }
    })
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

        assert_eq!(day2(example.to_owned()), 2);
    }
}
