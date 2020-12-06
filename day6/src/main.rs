#[path = "../../common/aoc2020.rs"]
mod aoc2020;

use std::collections::HashMap;

fn part1(filename: &str) -> u32 {
    let mut lines = aoc2020::lines_from_file(filename);
    lines.push("".to_string());

    let mut ans = HashMap::new();
    let mut nof_yes = 0;

    for l in lines {
        if l.len() == 0 {
            for (_key, val) in &ans {
                nof_yes += val;
            }
            ans.clear();
        } else {
            for c in l.chars() {
                let yes = ans.entry(c).or_insert(0u32);
                *yes = 1;
            }
        }
    }
    return nof_yes;
}

fn part2(filename: &str) -> u32 {
    let mut lines = aoc2020::lines_from_file(filename);
    lines.push("".to_string());

    let mut ans: HashMap<char, u32> = HashMap::new();
    let mut nof_yes = 0;
    let mut nof_persons = 0;

    for l in lines {
        if l.len() == 0 {
            for (_key, val) in &ans {
                if *val == nof_persons {
                    nof_yes += 1;
                }
            }
            nof_persons = 0;
            ans.clear();
        } else {
            for c in l.chars() {
                let yes = ans.entry(c).or_insert(0u32);
                *yes += 1;
            }
            nof_persons += 1;
        }
    }
    return nof_yes;
}

fn main() {
    assert_eq!(part1("src/example.txt"), 11);
    assert_eq!(part1("src/puzzle.txt"), 6809);
    assert_eq!(part2("src/example.txt"), 6);
    assert_eq!(part2("src/puzzle.txt"), 3394);
}
