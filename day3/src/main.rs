use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn lines_from_file<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn part1(filename: &str, x_step: usize, y_step: usize) -> i64 {
    let lines = lines_from_file(filename);
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    while y < lines.len() {
        let row = &lines[y];
        if '#' == row.chars().nth(x).unwrap() {
            trees = trees + 1;
        }
        x = (x + x_step) % row.len();
        y = y + y_step;
    }
    return trees;
}

fn part2(filename: &str) -> i64 {
    let mut trees: i64 = 1;
    trees = trees * part1(filename, 1, 1);
    trees = trees * part1(filename, 3, 1);
    trees = trees * part1(filename, 5, 1);
    trees = trees * part1(filename, 7, 1);
    trees = trees * part1(filename, 1, 2);
    return trees;
}

fn main() {
    assert_eq!(part1("src/example.txt", 3, 1), 7);
    assert_eq!(part1("src/puzzle1.txt", 3, 1), 205);

    assert_eq!(part1("src/example.txt", 1, 1), 2);
    assert_eq!(part1("src/example.txt", 3, 1), 7);
    assert_eq!(part1("src/example.txt", 5, 1), 3);
    assert_eq!(part1("src/example.txt", 7, 1), 4);
    assert_eq!(part1("src/example.txt", 1, 2), 2);
    assert_eq!(part2("src/example.txt"), 336);
    assert_eq!(part2("src/puzzle1.txt"), 3952146825);
}
