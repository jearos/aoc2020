#[path = "../../common/aoc2020.rs"]
mod aoc2020;

fn part1(filename: &str) -> i32 {
    let lines = aoc2020::lines_from_file(filename);
    //let mut entry : Vec<&str> = Vec::new();
    let mut valid = 0;
    let mut valid_entries = 0;
    for l in lines {
        if l.len() == 0 {
            if valid == 7 {
                valid_entries += 1;
            }
            valid = 0;
        }
        let v: Vec<&str> = l.split(|c| c == ' ' || c == ':').collect();
        for w in v {
            if w == "byr"
                || w == "iyr"
                || w == "eyr"
                || w == "hgt"
                || w == "hcl"
                || w == "ecl"
                || w == "pid"
            {
                valid = valid + 1
            }
        }
    }
    if valid == 7 {
        valid_entries += 1;
    }
    return valid_entries;
}

fn part2(filename: &str) -> i32 {
    let lines = aoc2020::lines_from_file(filename);
    //let mut entry : Vec<&str> = Vec::new();
    let mut valid = 0;
    let mut valid_entries = 0;
    for l in lines {
        if l.len() == 0 {
            if valid == 7 {
                valid_entries += 1;
            }
            valid = 0;
        }
        let v: Vec<&str> = l.split(|c| c == ' ').collect();
        for w in v {
            let kv: Vec<&str> = w.split(|c| c == ':').collect();
            if kv[0] == "byr" {
                let byr = kv[1].parse::<i32>().unwrap();
                if byr >= 1920 && byr <= 2002 {
                    valid = valid + 1
                }
            } else if kv[0] == "iyr" {
                let iyr = kv[1].parse::<i32>().unwrap();
                if iyr >= 2010 && iyr <= 2020 {
                    valid = valid + 1
                }
            } else if kv[0] == "eyr" {
                let eyr = kv[1].parse::<i32>().unwrap();
                if eyr >= 2020 && eyr <= 2030 {
                    valid = valid + 1
                }
            } else if kv[0] == "hgt" {
                if kv[1].contains("cm") {
                    let len = kv[1].replace("cm", "").parse::<i32>().unwrap();
                    if len >= 150 && len <= 193 {
                        valid = valid + 1
                    }
                } else if kv[1].contains("in") {
                    let len = kv[1].replace("in", "").parse::<i32>().unwrap();
                    if len >= 59 && len <= 76 {
                        valid = valid + 1
                    }
                }
            } else if kv[0] == "hcl" {
                if kv[1].len() == 7 {
                    if kv[1].chars().nth(0).unwrap() == '#' {
                        // for i in 1..6 {
                        //   print!("{}", kv[1].chars().nth(i).unwrap());
                        // }
                        valid = valid + 1
                    }
                }
            } else if kv[0] == "ecl" {
                if kv[1] == "amb"
                    || kv[1] == "blu"
                    || kv[1] == "brn"
                    || kv[1] == "gry"
                    || kv[1] == "grn"
                    || kv[1] == "hzl"
                    || kv[1] == "oth"
                {
                    valid = valid + 1
                }
            } else if kv[0] == "pid" {
                if kv[1].len() == 9 {
                    // for i in 1..9 {
                    //     print!("{}", kv[1].chars().nth(i).unwrap());
                    // }
                    // println!("");
                    valid = valid + 1
                }
            }
        }
    }
    if valid == 7 {
        valid_entries += 1;
    }
    return valid_entries;
}

fn main() {
    assert_eq!(part1("src/example.txt"), 2);
    assert_eq!(part1("src/puzzle1.txt"), 247);
    assert_eq!(part2("src/example2.txt"), 0);
    assert_eq!(part2("src/example3.txt"), 4);
    assert_eq!(part2("src/puzzle1.txt"), 145);
}
