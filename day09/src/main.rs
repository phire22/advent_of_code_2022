#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(input);
    println!("Q1: {:?}", result);

    let result = part2(input);
    println!("Q2: {:?}", result);
}

fn part1(input: &str) -> usize {
    let instructions = parse_input(input);
    let mut head: Coordinate = (0, 0);
    let mut tail: Coordinate = (0, 0);
    let mut visited: HashSet<Coordinate> = HashSet::new();
    for step in instructions {
        move_coord(&mut head, step);
        pull_tail(&mut tail, &head);
        visited.insert(tail);
    }
    visited.len()
}

fn part2(input: &str) -> usize {
    let instructions = parse_input(input);
    let mut head: Coordinate = (0, 0);
    let mut tails: [Coordinate; 9] = [(0, 0); 9];
    let mut visited: HashSet<Coordinate> = HashSet::new();
    for step in instructions {
        move_coord(&mut head, step);
        let mut knot_in_front = head;
        for (i, mut tail) in tails.iter_mut().enumerate() {
            pull_tail(tail, &knot_in_front);
            knot_in_front = *tail;
        }
        visited.insert(tails[8]);
    }
    visited.len()
}

fn parse_input(input: &str) -> Vec<Vector> {
    input
        .lines()
        .flat_map(|line| {
            let (step, num) = line.split_once(' ').unwrap();
            (0..num.parse().unwrap()).map(move |_| match step {
                "U" => (0, 1),
                "R" => (1, 0),
                "D" => (0, -1),
                "L" => (-1, 0),
                _ => unreachable!(),
            })
        })
        .collect()
}

type Vector = (i32, i32);

type Coordinate = (i32, i32);

fn move_coord(coord: &mut Coordinate, vector: Vector) {
    coord.0 += vector.0;
    coord.1 += vector.1;
}

fn pull_tail(tail: &mut Coordinate, head: &Coordinate) {
    let vector = match (head.0 - tail.0, head.1 - tail.1) {
        // straight
        (2, 0) => (1, 0),
        (0, 2) => (0, 1),
        (-2, 0) => (-1, 0),
        (0, -2) => (0, -1),
        // diagonal
        (1, 2) | (2, 1) | (2, 2) => (1, 1),
        (-1, 2) | (-2, 1) | (-2, 2) => (-1, 1),
        (1, -2) | (2, -1) | (2, -2) => (1, -1),
        (-1, -2) | (-2, -1) | (-2, -2) => (-1, -1),
        _ => (0, 0),
    };
    move_coord(tail, vector);
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 13);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 1);
    }
}
