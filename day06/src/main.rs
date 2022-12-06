use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(input);
    println!("Q1: {:?}", result);

    let result = part2(input);
    println!("Q2: {:?}", result);
}

fn part1(input: &str) -> usize {
    first_unique_set(input, 4)
}

fn part2(input: &str) -> usize {
    first_unique_set(input, 14)
}

fn first_unique_set(input: &str, size: usize) -> usize {
    input
        .as_bytes()
        .windows(size)
        .take_while(|x| !all_uniqe(x))
        .count()
        + size
}

fn all_uniqe(data: &[u8]) -> bool {
    let mut uniqe = HashSet::new();
    data.iter().all(move |x| uniqe.insert(x))
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    const INPUT: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 5);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 23);
    }
}
