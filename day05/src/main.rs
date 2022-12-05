use std::str::from_utf8;

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(input);
    println!("Q1: {:?}", result);

    let result = part2(input);
    println!("Q2: {:?}", result);
}

fn part1(input: &str) -> String {
    let (mut containers, instructions) = parse_input(input);
    for instr in instructions {
        let from = instr[1] - 1;
        let to = instr[2] - 1;
        let n = containers[from].len() - instr[0];
        let mut moved = containers[from].split_off(n);
        moved.reverse();
        containers[to].append(&mut moved);
    }
    get_last_elements(containers)
}

fn part2(input: &str) -> String {
    let (mut containers, instructions) = parse_input(input);
    for instr in instructions {
        let from = instr[1] - 1;
        let to = instr[2] - 1;
        let n = containers[from].len() - instr[0];
        let mut moved = containers[from].split_off(n);
        containers[to].append(&mut moved);
    }
    get_last_elements(containers)
}

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<usize>>) {
    let input_split = input.split_once("\n\n").unwrap();

    let mut containers = vec![];
    for line in input_split.0.lines() {
        line.as_bytes().chunks(4).enumerate().for_each(|(i, x)| {
            if containers.len() <= i {
                containers.push(vec![]);
            }
            if x[1] != b' ' {
                containers[i].insert(0, x[1]);
            }
        });
    }

    let instructions: Vec<Vec<usize>> = input_split
        .1
        .lines()
        .map(|line| line.split(&[' ']).filter_map(|x| x.parse().ok()).collect())
        .collect();

    (containers, instructions)
}

fn get_last_elements(data: Vec<Vec<u8>>) -> String {
    let last_elems: Vec<u8> = data.iter().map(|x| *x.last().unwrap()).collect();
    return from_utf8(&last_elems).unwrap().to_string();
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "MCD");
    }
}
