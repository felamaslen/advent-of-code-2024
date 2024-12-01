pub fn day1(input: String) -> i32 {
    let mut ordered_l: Vec<i32> = vec![];
    let mut ordered_r: Vec<i32> = vec![];

    input.lines().for_each(|line| {
        let (l, r) = line.split_once(" ").unwrap();
        ordered_l.push(str::parse::<i32>(l).unwrap());
        ordered_r.push(str::parse::<i32>(r.trim_start()).unwrap());
    });

    ordered_l.sort();
    ordered_r.sort();

    ordered_l.iter().enumerate().fold(0, |sum, (i, l)| {
        let diff = l - ordered_r[i];
        sum + (if diff > 0 { diff } else { -1 * diff })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pairs_example() {
        let input = r"3   4
4   3
2   5
1   3
3   9
3   3
";

        let result = day1(input.to_owned());

        assert_eq!(result, 11);
    }
}
