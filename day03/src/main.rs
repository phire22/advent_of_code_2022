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
        .filter_map(|line| find_common(split_rucksack(line)))
        .map(item_priority)
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .filter_map(find_common_in_group)
        .map(item_priority)
        .sum()
}

fn split_rucksack(rucksack: &str) -> (&str, &str) {
    rucksack.split_at(rucksack.len() / 2)
}

fn find_common(splits: (&str, &str)) -> Option<char> {
    splits.0.chars().find(|c| splits.1.contains(*c))
}

fn find_common_in_group(group: &[&str]) -> Option<char> {
    group
        .first()?
        .chars()
        .find(|c| group[1..].iter().all(|r| r.contains(*c)))
}

fn item_priority(item: char) -> usize {
    if item.is_ascii_lowercase() {
        ((item as u8) - b'a') as usize + 1
    } else if item.is_ascii_uppercase() {
        ((item as u8) - b'A') as usize + 27
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 157);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 70);
    }
}
