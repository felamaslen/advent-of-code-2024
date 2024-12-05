pub struct Day5 {
    pub part1: i16,
    pub part2: i16,
}

pub fn day5(input: String) -> Day5 {
    let mut part1 = 0;
    let mut part2 = 0;

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

    let bubble_sort_pages_pass = |pages: &mut Vec<i16>| {
        let mut has_changed = false;
        for i in 0..pages.len() - 1 {
            if ordering
                .iter()
                .any(|(a, b)| (*a == pages[i + 1] && *b == pages[i]))
            {
                pages[i..i + 2].rotate_left(1);
                has_changed = true;
            }
        }
        has_changed
    };

    let bubble_sort_pages = |pages: &mut Vec<i16>| {
        let mut has_sorted = false;
        let mut num_passes = 0;
        while !has_sorted {
            has_sorted = !bubble_sort_pages_pass(pages);
            num_passes += 1;
        }

        num_passes > 1
    };

    input
        .lines()
        .skip(ordering.len() + 1)
        .map(|line| {
            line.split(',')
                .map(|c| str::parse::<i16>(c).unwrap())
                .collect::<Vec<i16>>()
        })
        .for_each(|pages| {
            let mut ordered_pages = pages.clone();
            let sorted = bubble_sort_pages(&mut ordered_pages);

            if sorted {
                part2 += ordered_pages[pages.len() / 2];
            } else {
                part1 += pages[pages.len() / 2];
            }
        });

    Day5 { part1, part2 }
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

    #[test]
    fn gets_part2() {
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

        assert_eq!(result.part2, 123);
    }
}
