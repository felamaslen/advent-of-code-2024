use std::collections::HashSet;

pub struct Day9 {
    pub part1: usize,
    pub part2: usize,
}

fn part1(input: String) -> usize {
    let mut blocks = vec![];
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

fn part2(input: String) -> usize {
    let mut blocks = input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, ch)| {
            let n = ch.to_digit(10).unwrap() as usize;

            if i.rem_euclid(2) == 0 {
                let id = i / 2;
                (Some(id), n)
            } else {
                (None, n)
            }
        })
        .collect::<Vec<(Option<usize>, usize)>>();

    let mut i = blocks.len() - 1;
    let mut moved_blocks = HashSet::new();

    while i > 0 {
        let block = blocks[i];
        if let Some(id) = block.0 {
            if !moved_blocks.contains(&id) {
                if let Some((j, free_space)) = blocks
                    .clone()
                    .iter()
                    .enumerate()
                    .take(i)
                    .find(|(_i, f)| f.0.is_none() && f.1 >= block.1)
                {
                    blocks[i] = (None, block.1);
                    blocks[j] = block;
                    blocks.insert(j + 1, (None, free_space.1 - block.1));
                }
                moved_blocks.insert(id);
            }
        }

        i -= 1;
    }

    blocks
        .iter()
        .map(|block| {
            (0..block.1)
                .map(|_| block.0.unwrap_or(0))
                .collect::<Vec<usize>>()
        })
        .flatten()
        .enumerate()
        .fold(0, |sum, (i, block)| sum + i * block)
}

pub fn day9(input: String) -> Day9 {
    Day9 {
        part1: part1(input.clone()),
        part2: part2(input.clone()),
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

    #[test]
    fn gets_part2() {
        let input = r"2333133121414131402";

        let result = day9(input.to_owned());

        assert_eq!(result.part2, 2858);
    }
}
