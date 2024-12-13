pub struct Day13 {
    pub part1: usize,
    pub part2: usize,
}

fn part1(input: String) -> usize {
    let (mut machines, last_machine) = input.lines().fold(
        (vec![], ((0.0, 0.0), (0.0, 0.0), (0.0, 0.0))),
        |(mut machines, machine), line| {
            let get_button = || {
                let (x_diff, y_diff) = line.split_once(": ").unwrap().1.split_once(", ").unwrap();
                let x_diff_n = str::parse::<f32>(x_diff.split_once('+').unwrap().1).unwrap();
                let y_diff_n = str::parse::<f32>(y_diff.split_once('+').unwrap().1).unwrap();
                (x_diff_n, y_diff_n)
            };

            if line.starts_with("Button A") {
                let button_a = get_button();
                (machines, (button_a, (0.0, 0.0), (0.0, 0.0)))
            } else if line.starts_with("Button B") {
                let button_b = get_button();
                (machines, (machine.0, button_b, (0.0, 0.0)))
            } else if line.starts_with("Prize") {
                let (x, y) = line.split_once(": ").unwrap().1.split_once(", ").unwrap();
                let x_n = str::parse::<f32>(x.split_once("=").unwrap().1).unwrap();
                let y_n = str::parse::<f32>(y.split_once("=").unwrap().1).unwrap();

                (machines, (machine.0, machine.1, (x_n, y_n)))
            } else {
                machines.push(machine);
                (machines, ((0.0, 0.0), (0.0, 0.0), (0.0, 0.0)))
            }
        },
    );

    if last_machine.0 .0 > 0.0 {
        machines.push(last_machine);
    }

    machines
        .iter()
        .fold(0, |score, ((ax, ay), (bx, by), (px, py))| {
            // (ax bx)(Na) = (px)
            // (ay by)(Nb)   (py)
            //
            // =>
            //
            // (Na) =       1       (by  -bx)(px)
            // (Nb)   |ax*by-bx*ay| (-ay  ax)(py)
            //
            // =>
            //
            // Na = (by*px - bx*py) / (ax*by - bx*ay)
            // Nb = (ax*py - ay*px) / (ax*by - bx*ay)

            let na = (by * px - bx * py) / (ax * by - bx * ay);
            let nb = (ax * py - ay * px) / (ax * by - bx * ay);

            if na < 0.0 || nb < 0.0 || na.fract() != 0.0 || nb.fract() != 0.0 {
                return score;
            }

            let cost = 3 * (na as usize) + (nb as usize);

            score + cost
        })
}

pub fn day13(input: String) -> Day13 {
    Day13 {
        part1: part1(input.clone()),
        part2: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part1() {
        let input = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let result = day13(input.to_owned());

        assert_eq!(result.part1, 480);
    }
}
