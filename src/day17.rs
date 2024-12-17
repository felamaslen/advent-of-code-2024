pub struct Day17 {
    pub part1: String,
}

#[derive(Clone, Debug)]
struct Computer {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    program: Vec<u8>,
}

fn parse_input(input: String) -> Computer {
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

    Computer {
        reg_a,
        reg_b,
        reg_c,
        program,
    }
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
        4 => computer.reg_a,
        5 => computer.reg_b,
        6 => computer.reg_c,
        7 => panic!("Invalid program: 7 is a reserved combo operator"),
        _ => panic!("Operand out of range"),
    }
}

fn part1(input: String) -> String {
    let mut computer = parse_input(input);
    let mut i: usize = 0;

    let mut output = vec![];

    while i < computer.program.len() - 1 {
        let opcode = computer.program[i];
        let operand = computer.program[i + 1];

        let combo = operand_combo(&computer, operand);

        let mut jumped = false;

        match opcode {
            OP_ADV => {
                let numerator = computer.reg_a;
                let denominator = 2_u32.pow(combo as u32) as usize;
                computer.reg_a = numerator / denominator;
            }
            OP_BXL => {
                computer.reg_b = computer.reg_b ^ (operand as usize);
            }
            OP_BST => {
                computer.reg_b = combo.rem_euclid(8);
            }
            OP_JNZ => {
                if computer.reg_a != 0 {
                    i = operand as usize;
                    jumped = true;
                }
            }
            OP_BXC => {
                computer.reg_b = computer.reg_b ^ computer.reg_c;
            }
            OP_OUT => {
                output.push(format!("{}", combo.rem_euclid(8)));
            }
            OP_BDV => {
                let numerator = computer.reg_a;
                let denominator = 2_u32.pow(combo as u32) as usize;
                computer.reg_b = numerator / denominator;
            }
            OP_CDV => {
                let numerator = computer.reg_a;
                let denominator = 2_u32.pow(combo as u32) as usize;
                computer.reg_c = numerator / denominator;
            }
            _ => panic!("invalid opcode {}", opcode),
        }

        if !jumped {
            i += 2;
        }
    }

    output.join(",")
}

pub fn day17(input: String) -> Day17 {
    Day17 {
        part1: part1(input.clone()),
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
        let result = day17(input.to_owned());
        assert_eq!(result.part1, "4,6,3,5,6,3,5,2,1,0");
    }
}
