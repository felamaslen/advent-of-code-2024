use std::collections::HashMap;

pub struct Day11 {
    pub part1: usize,
    pub part2: usize,
}

fn num_digits(n: i64) -> u32 {
    let mut i = 0;
    while 10_i64.pow(i) <= n {
        i += 1;
    }
    i
}

fn split_stone(n: i64) -> Option<(i64, i64)> {
    if n < 10 {
        return None;
    }
    let digits = num_digits(n);
    if digits.rem_euclid(2) == 1 {
        return None;
    }
    let mut a = n;
    let mut b = 0;
    let mut i = 0;
    let h = digits / 2;
    while i < h {
        let next = a.rem_euclid(10);
        b += next * 10_i64.pow(i);
        a -= next;
        a /= 10;

        i += 1;
    }
    Some((a, b))
}

fn upsert_stone(stones: &mut HashMap<i64, usize>, stone: i64, count: usize) {
    if let Some(v) = stones.get_mut(&stone) {
        *v += count;
    } else {
        stones.insert(stone, count);
    }
}

fn blinkn(stones: Vec<i64>, n: usize) -> usize {
    let mut stones_count = HashMap::new();
    stones.iter().for_each(|stone| {
        upsert_stone(&mut stones_count, *stone, 1);
    });

    let blink = |prev_count: HashMap<i64, usize>| {
        let mut next_count = HashMap::new();
        prev_count.iter().for_each(|(stone, count)| match stone {
            0 => {
                upsert_stone(&mut next_count, 1, *count);
            }
            s => match split_stone(*s) {
                Some((a, b)) => {
                    upsert_stone(&mut next_count, a, *count);
                    upsert_stone(&mut next_count, b, *count);
                }
                None => {
                    upsert_stone(&mut next_count, s * 2024, *count);
                }
            },
        });
        next_count
    };

    (0..n)
        .into_iter()
        .fold(stones_count, |prev_count, _| blink(prev_count))
        .iter()
        .fold(0, |sum, (_, count)| sum + count)
}

pub fn day11(input: String) -> Day11 {
    let stones = input
        .trim()
        .split(' ')
        .map(|ch| str::parse::<i64>(ch).unwrap())
        .collect::<Vec<i64>>();

    Day11 {
        part1: blinkn(stones.clone(), 25),
        part2: blinkn(stones.clone(), 75),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part1() {
        let input = r"125 17";
        let result = day11(input.to_owned());
        assert_eq!(result.part1, 55312);
    }
}
