pub struct Day9 {
    pub part1: usize,
}

fn part1(input: String) -> usize {
    let mut blocks: Vec<Option<usize>> = vec![];

    input.trim().chars().enumerate().for_each(|(i, ch)| {
        let n = ch.to_digit(10).unwrap() as usize;
        if i.rem_euclid(2) == 0 {
            let block = i / 2;
            (0..n).into_iter().for_each(|_| {
                blocks.push(Some(block));
            });
        } else {
            (0..n).into_iter().for_each(|_| {
                blocks.push(None);
            });
        }
    });

    let mut i = 0;
    while i < blocks.len() - 1 {
        if blocks[i].is_none() {
            let mut next = blocks.pop().unwrap();
            while next.is_none() {
                next = blocks.pop().unwrap();
            }
            blocks[i] = next;
        }
        i += 1;
    }

    let sum = blocks
        .iter()
        .enumerate()
        .fold(0, |prev, (i, block)| prev + i * block.unwrap_or(0));

    sum
}

pub fn day9(input: String) -> Day9 {
    Day9 {
        part1: part1(input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part1() {
        let input = r"2333133121414131402";

        let result = day9(input.to_owned());

        assert_eq!(result.part1, 1928);
    }
}
