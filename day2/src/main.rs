use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
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

fn calc(filename: &str) -> i32 {
    let lines = lines_from_file(filename);
    let mut valid = 0;
    for l in lines {
        let v: Vec<&str> = l.split(|c| c == '-' || c == ' ' || c == ':').collect();
    //        println!("{:?}", v);
        let min_num = v[0].parse::<i32>().unwrap();
        let max_num = v[1].parse::<i32>().unwrap();
        let character = v[2].parse::<char>().unwrap();
        let mut cnt = 0;
        for c in v[4].chars() {
            if c == character
            {
                cnt = cnt + 1;
            }
        }
        if cnt >= min_num {
            if cnt <= max_num {
                valid = valid +1;
            }
        }
    }
    return valid;
}

fn part2(filename: &str) -> i32 {
    let lines = lines_from_file(filename);
    let mut valid = 0;
    for l in lines {
        let v: Vec<&str> = l.split(|c| c == '-' || c == ' ' || c == ':').collect();
        let min_num = v[0].parse::<usize>().unwrap() - 1;
        let max_num = v[1].parse::<usize>().unwrap() - 1;
        let character = v[2].parse::<char>().unwrap();
        let pw = v[4];

        if (pw.chars().nth(min_num).unwrap() == character) ^
           (pw.chars().nth(max_num).unwrap() == character) {
            valid = valid + 1;
        }
    }
    return valid;
}

fn main() {
    assert_eq!(calc("src/example.txt"), 2);
    assert_eq!(calc("src/puzzle1.txt"), 483);
    assert_eq!(part2("src/example.txt"), 1);
    assert_eq!(part2("src/puzzle1.txt"), 482);
}
