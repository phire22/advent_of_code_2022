use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(input);
    println!("Q1: {:?}", result);

    let result = part2(input);
    println!("Q2: {:?}", result);
}

fn part1(input: &str) -> usize {
    let tree = process_tree(input);

    tree.nodes.into_values().filter(|&x| x <= 100000).sum()
}

fn part2(input: &str) -> usize {
    let tree = process_tree(input);

    let total_size: usize = tree.nodes["/"];
    tree.nodes
        .into_iter()
        .filter(|x| x.1 >= total_size - 40000000)
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .1
}

fn process_tree(input: &str) -> Tree {
    let mut tree = Tree::new();
    for cmd in input.split('$').skip(1) {
        match &cmd[..3] {
            " cd" => {
                let mut folder = cmd[4..].to_owned();
                folder.pop(); // remove trailing new line
                tree.insert(&folder);
                tree.cd(&folder);
            }
            " ls" => {
                for file in cmd.lines().skip(1) {
                    if !file.starts_with("dir") {
                        let file_size = file.split_once(' ').unwrap().0.parse::<usize>().unwrap();
                        let file_loc = tree.curr_dir.to_string();
                        tree.root();
                        if file_loc != "/" {
                            tree.add_size(file_size); // not nice...
                        }
                        for folder in file_loc.split('/').skip(1) {
                            tree.cd(folder);
                            tree.add_size(file_size);
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    tree
}

type Key = String;

#[derive(Debug)]
struct Tree {
    curr_dir: Key,
    nodes: HashMap<Key, usize>,
}

impl Tree {
    fn new() -> Self {
        Self {
            curr_dir: "".to_string(),
            nodes: HashMap::new(),
        }
    }

    fn cd(&mut self, folder: &str) {
        self.curr_dir = self.get_dirname(folder);
    }

    fn root(&mut self) {
        self.curr_dir = "/".to_string();
    }

    fn insert(&mut self, folder: &str) {
        let new_dir = self.get_dirname(folder);
        self.nodes.entry(new_dir).or_insert(0);
    }

    fn get_dirname(&self, subfolder: &str) -> String {
        match subfolder {
            ".." => {
                let parent = self.curr_dir.rsplit_once('/').unwrap().0.to_string();
                if parent.is_empty() {
                    return '/'.to_string();
                }
                parent
            }
            "/" => "/".to_string(),
            _ => {
                if self.curr_dir.ends_with('/') {
                    return format!("{}{}", self.curr_dir, subfolder);
                }
                format!("{}/{}", self.curr_dir, subfolder)
            }
        }
    }

    fn add_size(&mut self, size: usize) {
        let folder = self.curr_dir.to_string();
        self.nodes
            .insert(folder.to_string(), self.nodes[&folder] + size);
    }
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 95437);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 24933642);
    }
}
