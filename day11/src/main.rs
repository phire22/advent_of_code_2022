use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(input);
    println!("Q1: {:?}", result);

    let result = part2(input);
    println!("Q2: {:?}", result);
}

fn part1(input: &'static str) -> usize {
    let mut monkeys = parse_input(input);
    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..20 {
        for source in 0..monkeys.len() {
            let monkey = monkeys[source].clone();
            for item in &monkey.items {
                let worry = monkey.operation(*item) / 3;
                let target = monkey.decide(worry);
                monkeys[target].items.push(worry);
                monkeys[source].items.remove(0);
                inspections[source] += 1;
            }
        }
    }

    inspections.sort_by(|a, b| b.cmp(a));
    inspections[0] * inspections[1]
}

fn part2(input: &'static str) -> usize {
    let mut monkeys = parse_input(input);
    let mut inspections = vec![0; monkeys.len()];
    // common denominator
    let base = monkeys.iter().map(|m| m.test_divisor).product::<usize>();

    for _ in 0..10000 {
        for source in 0..monkeys.len() {
            let monkey = monkeys[source].clone();
            for item in &monkey.items {
                let worry = monkey.operation(*item) % base;
                let target = monkey.decide(worry);
                monkeys[target].items.push(worry);
                monkeys[source].items.remove(0);
                inspections[source] += 1;
            }
        }
    }

    inspections.sort_by(|a, b| b.cmp(a));
    inspections[0] * inspections[1]
}

fn parse_input(input: &'static str) -> Vec<Monkey> {
    input.split("\n\n").map(parse_monkey).collect()
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test_divisor: usize,
    true_monkey: usize,
    false_monkey: usize,
}

impl Monkey {
    fn decide(&self, number: usize) -> usize {
        if number % self.test_divisor == 0 {
            return self.true_monkey;
        }
        self.false_monkey
    }

    fn operation(&self, item: usize) -> usize {
        self.operation.apply(item)
    }

    fn _parser(input: &str, replace: &str) -> usize {
        input.replace(replace, "").parse().unwrap()
    }
}

fn parse_monkey(input: &str) -> Monkey {
    let re = Regex::new(r"Monkey (\d+):\n  Starting items: ([\d, ]+)\n  Operation: (.*)\n  Test: divisible by (\d+)\n    If true: throw to monkey (\d+)\n    If false: throw to monkey (\d+)").unwrap();
    let captures = re.captures(input).unwrap();

    let items = captures[2]
        .split(", ")
        .map(|s| s.parse().unwrap())
        .collect();

    let operation = captures[3].to_string();
    Monkey {
        items,
        operation: Operation::from_str(&operation),
        test_divisor: captures[4].parse().unwrap(),
        true_monkey: captures[5].parse().unwrap(),
        false_monkey: captures[6].parse().unwrap(),
    }
}

#[derive(Debug, Clone)]
struct Operation {
    operator: String,
    number: Option<usize>,
}

impl Operation {
    fn from_str(input: &str) -> Self {
        let (_, input) = input.split_once("new = old ").unwrap();
        let (operator, number) = if input.contains("* old") {
            ("pow", None)
        } else {
            (&input[0..1], Some(input[1..].trim().parse().unwrap()))
        };
        Self {
            operator: operator.to_string(),
            number,
        }
    }

    fn apply(&self, target: usize) -> usize {
        match self.operator.as_str() {
            "pow" => target * target,
            "*" => target * self.number.unwrap(),
            "+" => target + self.number.unwrap(),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 10605);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 2713310158);
    }
}
