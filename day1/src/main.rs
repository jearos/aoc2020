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

fn calc(filename: String) -> i32 {
    let lines = lines_from_file(filename);
    let mut numbers = Vec::new();

    for l in lines {
        let num = l.parse::<i32>().unwrap();
        numbers.push(num);
    }

    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            if numbers[i]+numbers[j] == 2020 {
                return numbers[i]*numbers[j];
            }
        }
    }
    return 0;
}

fn calc2(filename: String) -> i32 {
    let lines = lines_from_file(filename);
    let mut numbers = Vec::new();

    for l in lines {
        let num = l.parse::<i32>().unwrap();
        numbers.push(num);
    }

    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            for k in 0..numbers.len() {
                if (i != j) && (i != k) && (j != k) {
                    if numbers[i]+numbers[j]+numbers[k] == 2020 {
                        return numbers[i]*numbers[j]*numbers[k];
                    }
                }
            }
        }
    }
    return 0;
}

fn main() {
    assert_eq!(calc("src/example.txt".to_string()), 514579);
    assert_eq!(calc("src/puzzle1.txt".to_string()), 921504);
    assert_eq!(calc2("src/example.txt".to_string()), 241861950);
    assert_eq!(calc2("src/puzzle1.txt".to_string()), 195700142);
}
