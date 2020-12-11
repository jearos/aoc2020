#[path = "../../common/aoc2020.rs"]
mod aoc2020;

use cached::proc_macro::cached;
use std::convert::TryInto;

fn part1(filename: &str) -> i32 {
    let lines = aoc2020::lines_from_file(filename);
    let mut v: Vec<i32> = Vec::new();
    for l in lines {
        v.push(l.parse::<i32>().unwrap())
    }
    let rated = match v.iter().max() {
        None => 0,
        Some(min) => *min + 3,
    };
    v.push(0);
    v.push(rated);
    v.sort();

    let mut num_1 = 0;
    let mut num_3 = 0;
    for i in 0..v.len() - 1 {
        let diff = v[i + 1] - v[i];
        if diff == 1 {
            num_1 += 1;
        }
        if diff == 3 {
            num_3 += 1;
        }
        print!("{} ", diff);
    }
    return num_1 * num_3;
}

#[cached]
fn calc_paths(v: [u64; 104], i: usize, l: usize) -> u64 {
    let mut res = 0;
    if l - 1 == i {
        return 1;
    }
    for j in i + 1..l {
        if v[i] - v[j] <= 3 {
            //println!("{} {}", v[j], v[i]);
            res += calc_paths(v, j, l);
        } else {
            return res;
        }
    }
    return res;
}

fn part2(filename: &str) -> u64 {
    let lines = aoc2020::lines_from_file(filename);
    let mut v: Vec<u64> = Vec::new();
    for l in lines {
        v.push(l.parse::<u64>().unwrap())
    }
    let rated = match v.iter().max() {
        None => 0,
        Some(min) => *min + 3,
    };
    v.push(0);
    v.push(rated);
    v.sort();
    v.reverse();
    println!("{:?}", v);

    println!("{}", v.len());
    let a: [u64; 104] = v.try_into().unwrap_or_else(|v: Vec<u64>| {
        panic!("Expected a Vec of length {} but it was {}", 104, v.len())
    });

    return calc_paths(a, 0, a.len());
}

fn main() {
    assert_eq!(part1("src/example.txt"), 7 * 5);
    assert_eq!(part1("src/example2.txt"), 22 * 10);
    assert_eq!(part1("src/puzzle.txt"), 2232);
    //assert_eq!(part2("src/example.txt"), 8);
    //assert_eq!(part2("src/example2.txt"), 19208);
    assert_eq!(part2("src/puzzle.txt"), 173625106649344);
}
