pub struct Day17 {
    pub part1: String,
    pub part2: usize,
}

#[derive(Clone, Debug)]
struct Computer(usize, usize, usize);

fn parse_input(input: String) -> (Computer, Vec<u8>) {
    let reg_a = str::parse::<usize>(
        input
            .lines()
            .find(|line| line.starts_with("Register A:"))
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .trim(),
    )
    .unwrap();
    let reg_b = str::parse::<usize>(
        input
            .lines()
            .find(|line| line.starts_with("Register B:"))
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .trim(),
    )
    .unwrap();
    let reg_c = str::parse::<usize>(
        input
            .lines()
            .find(|line| line.starts_with("Register C:"))
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .trim(),
    )
    .unwrap();

    let program = input
        .lines()
        .find(|line| line.starts_with("Program:"))
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim_start()
        .split(",")
        .map(|ch| str::parse::<u8>(ch).unwrap())
        .collect::<Vec<u8>>();

    let computer = Computer(reg_a, reg_b, reg_c);
    (computer, program)
}

const OP_ADV: u8 = 0;
const OP_BXL: u8 = 1;
const OP_BST: u8 = 2;
const OP_JNZ: u8 = 3;
const OP_BXC: u8 = 4;
const OP_OUT: u8 = 5;
const OP_BDV: u8 = 6;
const OP_CDV: u8 = 7;

fn operand_combo(computer: &Computer, operand: u8) -> usize {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => computer.0,
        5 => computer.1,
        6 => computer.2,
        7 => panic!("Invalid program: 7 is a reserved combo operator"),
        _ => panic!("Operand out of range"),
    }
}

fn run_step(computer: &mut Computer, program: &Vec<u8>, i: usize) -> (usize, Option<usize>) {
    let opcode = program[i];
    let operand = program[i + 1];

    let combo = operand_combo(&computer, operand);

    match opcode {
        OP_ADV => {
            let numerator = computer.0;
            let denominator = 2_u32.pow(combo as u32) as usize;
            computer.0 = numerator / denominator;
            (i + 2, None)
        }
        OP_BXL => {
            computer.1 = computer.1 ^ (operand as usize);
            (i + 2, None)
        }
        OP_BST => {
            computer.1 = combo.rem_euclid(8);
            (i + 2, None)
        }
        OP_JNZ => {
            if computer.0 != 0 {
                (operand as usize, None)
            } else {
                (i + 2, None)
            }
        }
        OP_BXC => {
            computer.1 = computer.1 ^ computer.2;
            (i + 2, None)
        }
        OP_OUT => (i + 2, Some(combo.rem_euclid(8))),
        OP_BDV => {
            let numerator = computer.0;
            let denominator = 2_u32.pow(combo as u32) as usize;
            computer.1 = numerator / denominator;
            (i + 2, None)
        }
        OP_CDV => {
            let numerator = computer.0;
            let denominator = 2_u32.pow(combo as u32) as usize;
            computer.2 = numerator / denominator;
            (i + 2, None)
        }
        _ => panic!("invalid opcode {}", opcode),
    }
}

fn run_program(computer: &mut Computer, program: &Vec<u8>) -> Vec<usize> {
    let mut i: usize = 0;
    let mut output = vec![];

    while i < program.len() - 1 {
        let (i_next, out) = run_step(computer, program, i);
        i = i_next;
        if let Some(o) = out {
            output.push(o);
        }
    }

    output
}

fn part1(input: String) -> String {
    let (mut computer, program) = parse_input(input);
    run_program(&mut computer, &program)
        .iter()
        .map(|o| format!("{}", o))
        .collect::<Vec<String>>()
        .join(",")
}

fn part2(input: String) -> usize {
    let (computer, program) = parse_input(input);
    let prog_len = program.len();

    let mut stack = vec![(0, prog_len - 1)];

    let mut solution = None;

    while stack.len() != 0 {
        let (reg_a_init, i) = stack.pop().unwrap();
        for j in 0..8 {
            let reg_a = reg_a_init + j;
            let mut computer_mod = computer.clone();
            computer_mod.0 = reg_a;
            let output = run_program(&mut computer_mod, &program);
            let matches = (i..prog_len)
                .into_iter()
                .all(|k| output[k - i] == program[k] as usize);

            if matches {
                if i == 0 {
                    solution = match solution {
                        Some(s) => {
                            if s < reg_a {
                                Some(s)
                            } else {
                                Some(reg_a)
                            }
                        }
                        None => Some(reg_a),
                    };
                } else {
                    stack.push((reg_a * 8, i - 1));
                }
            }
        }
    }

    solution.expect("Did not find solution")
}

pub fn day17(input: String) -> Day17 {
    Day17 {
        part1: part1(input.clone()),
        part2: part2(input.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part1() {
        let input = r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        let result = part1(input.to_owned());
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn gets_part2() {
        let input = r"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        let result = part2(input.to_owned());
        assert_eq!(result, 117440);
    }
}
