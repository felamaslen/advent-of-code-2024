pub struct Day1 {
    pub diff_sum: i32,
    pub similarity_score: i32,
}

fn parse_input(input: String) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(" ").unwrap();
            (
                str::parse::<i32>(l).unwrap(),
                str::parse::<i32>(r.trim_start()).unwrap(),
            )
        })
        .collect::<Vec<(i32, i32)>>()
}

fn diff_sum(input: Vec<(i32, i32)>) -> i32 {
    let mut ordered_l: Vec<i32> = vec![];
    let mut ordered_r: Vec<i32> = vec![];

    input.iter().for_each(|(l, r)| {
        ordered_l.push(l.to_owned());
        ordered_r.push(r.to_owned());
    });

    ordered_l.sort();
    ordered_r.sort();

    ordered_l.iter().enumerate().fold(0, |sum, (i, l)| {
        let diff = l - ordered_r[i];
        sum + (if diff > 0 { diff } else { -1 * diff })
    })
}

fn similarity_score(input: Vec<(i32, i32)>) -> i32 {
    let mut ordered_r: Vec<i32> = vec![];
    input.iter().for_each(|(_l, r)| {
        ordered_r.push(r.to_owned());
    });
    ordered_r.sort();

    let right_count = |value: &i32| {
        ordered_r
            .iter()
            .skip_while(|r| r.to_owned() < value)
            .take_while(|r| r.to_owned() == value)
            .count()
    };

    input
        .iter()
        .fold(0, |sum, (l, _)| sum + l * (right_count(l) as i32))
}

pub fn day1(input: String) -> Day1 {
    let input_parsed = parse_input(input);
    Day1 {
        diff_sum: diff_sum(input_parsed.clone()),
        similarity_score: similarity_score(input_parsed.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_ordered_diff_sum() {
        let input = r"3   4
4   3
2   5
1   3
3   9
3   3
";

        let result = day1(input.to_owned());

        assert_eq!(result.diff_sum, 11);
    }

    #[test]
    fn calculates_similarity_score() {
        let input = r"3   4
4   3
2   5
1   3
3   9
3   3
";

        let result = day1(input.to_owned());

        assert_eq!(result.similarity_score, 31);
    }
}
