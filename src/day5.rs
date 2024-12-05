pub struct Day5 {
    pub part1: i16,
}

fn sum_correctly_ordered(input: String) -> i16 {
    let ordering = input
        .lines()
        .take_while(|line| line.trim().len() > 0)
        .map(|line| {
            let order: Vec<i16> = line
                .splitn(2, '|')
                .map(|c| str::parse::<i16>(c).unwrap())
                .collect();

            (order[0], order[1])
        })
        .collect::<Vec<(i16, i16)>>();

    input
        .lines()
        .skip(ordering.len() + 1)
        .map(|line| {
            line.split(',')
                .map(|c| str::parse::<i16>(c).unwrap())
                .collect::<Vec<i16>>()
        })
        .filter(|pages| {
            let is_in_order = pages.iter().enumerate().all(|(i, page)| {
                let mut pages_before = pages.iter().take(i);
                let mut pages_after = pages.iter().skip(i + 1);

                let mut ordering_before = ordering.iter().filter(|(p, _)| p == page);
                let mut ordering_after = ordering.iter().filter(|(_, p)| p == page);

                let incorrect_order = pages_before.any(|p| ordering_before.any(|(_, b)| b == p))
                    || pages_after.any(|p| ordering_after.any(|(a, _)| a == p));

                !incorrect_order
            });

            is_in_order
        })
        .fold(0, |sum, pages| sum + pages[pages.len() / 2])
}

pub fn day5(input: String) -> Day5 {
    let part1 = sum_correctly_ordered(input);

    Day5 { part1 }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part1() {
        let input = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let result = day5(input.to_owned());

        assert_eq!(result.part1, 143);
    }
}
