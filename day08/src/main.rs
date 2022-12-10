fn main() {
    let input = include_str!("../input.txt");

    let result = part1(input);
    println!("Q1: {:?}", result);

    let result = part2(input);
    println!("Q2: {:?}", result);
}

fn part1(input: &str) -> usize {
    let forest = parse_input(input);

    forest
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| (y, x)))
        .filter(|(y, x)| is_visible(&forest, *x, *y))
        .count()
}

fn part2(input: &str) -> usize {
    let forest = parse_input(input);

    forest
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, _)| (x, y))
                .map(|(x, y)| scenic_score(&forest, x, y))
        })
        .max_by(|a, b| a.cmp(b))
        .unwrap()
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().flat_map(|c| c.to_digit(10)).collect())
        .collect()
}

fn is_visible(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let (w, h) = (forest.len(), forest[0].len());
    let height = forest[x][y];
    x == 0
        || x == w - 1
        || y == 0
        || y == h - 1
        || (0..x).all(|i| forest[i][y] < height)
        || (x + 1..w).all(|i| forest[i][y] < height)
        || (0..y).all(|i| forest[x][i] < height)
        || (y + 1..h).all(|i| forest[x][i] < height)
}

fn scenic_score(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    let (w, h) = (forest.len() as i32, forest[0].len() as i32 as i32);
    let height = forest[y][x];
    let directions = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    let mut counts = [0; 4];
    for (i, (dx, dy)) in directions.into_iter().enumerate() {
        let (mut x, mut y) = (x as i32, y as i32);
        while x != 0 && x != w - 1 && y != 0 && y != h - 1 {
            x += dx;
            y += dy;
            counts[i] += 1;
            if forest[y as usize][x as usize] >= height {
                break;
            }
        }
    }
    counts.iter().product::<usize>()
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 21);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 8);
    }
}
