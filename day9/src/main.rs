#[path = "../../common/aoc2020.rs"]
mod aoc2020;

fn part1(filename: &str, preamble_len: usize) -> i64 {
    let lines = aoc2020::lines_from_file(filename);
    let mut xmas: Vec<i64> = Vec::new();
    let mut preamble_sum = Vec::new();
    for l in lines {
        xmas.push(l.parse::<i64>().unwrap());
    }
    for k in 0..(xmas.len() - preamble_len) {
        preamble_sum.clear();
        for i in k..preamble_len + k {
            for j in k..preamble_len + k {
                if i != j {
                    preamble_sum.push(xmas[i] + xmas[j]);
                }
            }
        }
        let mut found = false;
        let mut num = 0;
        for s in &preamble_sum {
            num = xmas[preamble_len + k];
            if num == *s {
                found = true;
                break;
            }
        }
        if !found {
            return num;
        }
    }
    return 0;
}

fn part2(filename: &str, num: i64) -> i64 {
    let lines = aoc2020::lines_from_file(filename);
    let mut xmas: Vec<i64> = Vec::new();
    for l in lines {
        xmas.push(l.parse::<i64>().unwrap());
    }
    for i in 0..xmas.len() {
        let mut sum = 0;
        let mut found = false;
        let mut max = 0;
        let mut min = i64::MAX;
        for j in i..xmas.len() - i {
            let x = xmas[j];
            if x > max {
                max = x;
            }
            if x < min {
                min = x;
            }
            sum += xmas[j];
            if sum == num {
                found = true;
                break;
            }
            if sum > num {
                break;
            }
        }
        if found {
            return max + min;
        }
    }
    return 0;
}

fn main() {
    assert_eq!(part1("src/example.txt", 5), 127);
    assert_eq!(part1("src/puzzle.txt", 25), 88311122);
    assert_eq!(part2("src/example.txt", 127), 62);
    assert_eq!(part2("src/puzzle.txt", 88311122), 13549369);
}
