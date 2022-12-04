fn main() {
    let input = include_str!("../input.txt");

    let result = part1(input);
    println!("Q1: {:?}", result);

    let result = part2(input);
    println!("Q2: {:?}", result);
}

fn part1(input: &str) -> usize {
    input.lines().map(parse_line).filter(overlaps_fully).count()
}

fn part2(input: &str) -> usize {
    input.lines().map(parse_line).filter(overlaps_any).count()
}

fn parse_line(line: &str) -> Vec<Vec<usize>> {
    line.split(",")
        .map(|x| x.split("-").flat_map(|x| x.parse()).collect())
        .collect()
}

fn overlaps_fully(x: &Vec<Vec<usize>>) -> bool {
    x[0][0] >= x[1][0] && x[0][1] <= x[1][1] || x[1][0] >= x[0][0] && x[1][1] <= x[0][1]
}

fn overlaps_any(x: &Vec<Vec<usize>>) -> bool {
    x[0][0] <= x[1][0] && x[0][1] >= x[1][0] || x[1][0] <= x[0][0] && x[1][1] >= x[0][0]
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 4);
    }
}
