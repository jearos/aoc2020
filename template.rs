#[path = "../../common/aoc2020.rs"]
mod aoc2020;

fn part1(filename: &str) -> i32 {
    let lines = aoc2020::lines_from_file(filename);
    return lines.len() as i32;
}

fn main() {
    assert_eq!(part1("src/example.txt"), 2);
}
