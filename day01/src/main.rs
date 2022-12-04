fn main() {
    let input = include_str!("../input.txt");
    let mut elves = input
        .split("\n\n")
        .map(|x| x.lines().flat_map(str::parse::<usize>).sum())
        .collect::<Vec<usize>>();
    elves.sort_by(|a, b| b.cmp(a));

    println!("Q1: {:?}", elves[0]);
    println!("Q2: {:?}", elves.iter().take(3).sum::<usize>());
}
