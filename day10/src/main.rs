fn main() {
    let input = include_str!("../input.txt");

    let result = part1(input);
    println!("Q1: {:?}", result);

    let result = part2(input);
    println!("Q2:");
    result
        .as_bytes()
        .chunks(40)
        .flat_map(std::str::from_utf8)
        .for_each(|x| println!("{:?}", x));
}

fn part1(input: &str) -> i32 {
    parse_input(input)
        .iter()
        .scan(1, |state, &x| {
            *state += x;
            Some(*state)
        })
        .enumerate()
        .map(|(cycle, x)| (cycle + 2, x))
        .filter(|(cycle, _)| *cycle == 20 || (cycle + 20) % 40 == 0)
        .map(|(cycle, x)| x * (cycle as i32))
        .sum()
}

fn part2(input: &str) -> String {
    parse_input(input)
        .iter()
        .scan(1, |state, &x| {
            let prev_state = *state;
            *state += x;
            Some(prev_state)
        })
        .enumerate()
        .map(|(cycle, x)| {
            let pixel_pos = (cycle % 40) as i32;
            if x - 1 <= pixel_pos && x + 1 >= pixel_pos {
                return "#";
            }
            "."
        })
        .collect()
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .flat_map(|instruction| match &instruction[..4] {
            "addx" => vec![0, instruction[5..].parse().unwrap()],
            _ => vec![0],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 13140);
    }
}
