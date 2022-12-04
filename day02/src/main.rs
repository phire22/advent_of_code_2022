fn main() {
    let input = include_str!("../input.txt");

    let result = part1(input);
    println!("Q1: {:?}", result);

    let result = part2(input);
    println!("Q2: {:?}", result);
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| line.split_once(' '))
        .map(|x| score(x.1, x.0))
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| line.split_once(' '))
        .map(|x| score(player_from_outcome(x.0, x.1), x.0))
        .sum()
}

fn score(player: &str, opponent: &str) -> usize {
    match (player, opponent) {
        ("X", "A") => 1 + 3,
        ("X", "B") => 1 + 0,
        ("X", "C") => 1 + 6,
        ("Y", "A") => 2 + 6,
        ("Y", "B") => 2 + 3,
        ("Y", "C") => 2 + 0,
        ("Z", "A") => 3 + 0,
        ("Z", "B") => 3 + 6,
        ("Z", "C") => 3 + 3,
        _ => unreachable!(),
    }
}

fn player_from_outcome<'a>(opponent: &str, outcome: &str) -> &'a str {
    match (opponent, outcome) {
        ("B", "X") | ("A", "Y") | ("C", "Z") => "X",
        ("C", "X") | ("B", "Y") | ("A", "Z") => "Y",
        ("A", "X") | ("C", "Y") | ("B", "Z") => "Z",
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    const INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 15);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 12);
    }
}
