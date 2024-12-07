pub struct Day7 {
    pub part1: i64,
    pub part2: i64,
}

const OP_PLUS: i64 = 0;
const OP_TIMES: i64 = 1;

const OP_CONCAT: i64 = 2;

fn part1(input: String) -> i64 {
    input.lines().fold(0, |sum, line| {
        let (result_str, inputs_str) = line.split_once(':').unwrap();
        let expected_result = str::parse::<i64>(result_str).unwrap();

        let inputs_int = inputs_str
            .trim()
            .split(' ')
            .map(|c| str::parse::<i64>(c).unwrap());

        let num_ops = (inputs_int.clone().count() - 1) as u32;

        let num_permutations = 2_i64.pow(num_ops);

        let valid_permutation = (0..num_permutations).into_iter().find(|i| {
            let mut inputs = inputs_int.clone();
            let result_with_op_sequence = (0..num_ops)
                .fold((*i, inputs.next().unwrap().clone()), |(n, result), _j| {
                    let op = n.rem_euclid(2);
                    let n_next = (n - op) / 2;

                    let input = inputs.next().unwrap();
                    let result_next = match op {
                        OP_PLUS => result + input,
                        OP_TIMES => result * input,
                        _ => panic!("Invalid op {}", op),
                    };

                    (n_next, result_next)
                })
                .1;

            result_with_op_sequence == expected_result
        });

        if valid_permutation.is_some() {
            sum + expected_result
        } else {
            sum
        }
    })
}

fn part2(input: String) -> i64 {
    input.lines().fold(0, |sum, line| {
        let (result_str, inputs_str) = line.split_once(':').unwrap();
        let expected_result = str::parse::<i64>(result_str).unwrap();

        let inputs_int = inputs_str
            .trim()
            .split(' ')
            .map(|c| str::parse::<i64>(c).unwrap());

        let num_ops = (inputs_int.clone().count() - 1) as u32;

        let num_permutations = 3_i64.pow(num_ops);

        let valid_permutation = (0..num_permutations).into_iter().find(|i| {
            let mut inputs = inputs_int.clone();
            let result_with_op_sequence = (0..num_ops)
                .fold((*i, inputs.next().unwrap().clone()), |(n, result), _j| {
                    let op = n.rem_euclid(3);
                    let n_next = (n - op) / 3;

                    let input = inputs.next().unwrap();
                    let result_next = match op {
                        OP_PLUS => result + input,
                        OP_TIMES => result * input,
                        OP_CONCAT => {
                            str::parse::<i64>(format!("{}{}", result, input).as_str()).unwrap()
                        }
                        _ => panic!("Invalid op {}", op),
                    };

                    (n_next, result_next)
                })
                .1;

            result_with_op_sequence == expected_result
        });

        if valid_permutation.is_some() {
            sum + expected_result
        } else {
            sum
        }
    })
}

pub fn day7(input: String) -> Day7 {
    Day7 {
        part1: part1(input.clone()),
        part2: part2(input.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part1() {
        let input = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let result = day7(input.to_owned());

        assert_eq!(result.part1, 3749);
    }

    #[test]
    fn gets_part2() {
        let input = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let result = day7(input.to_owned());

        assert_eq!(result.part2, 11387);
    }
}
