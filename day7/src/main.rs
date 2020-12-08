#[path = "../../common/aoc2020.rs"]
mod aoc2020;

use std::collections::HashMap;

fn dig(map: &HashMap<String, Vec<String>>, name: String) -> bool {
    let mut retval: bool = false;
    let v = &map[&name];
    for e in v {
        if e == "shiny gold" {
            retval = true;
            break;
        }
        if e != "" {
            if dig(map, e.to_string()) {
                retval = true;
            }
        }
    }
    return retval;
}

fn dig2(map: &HashMap<String, Vec<(String, i32)>>, name: String) -> u32 {
    let mut retval: u32 = 0;
    let v = &map[&name];
    for e in v {
        if e.1 != 0 {
            retval += dig2(&map, e.0.to_string()) * e.1 as u32;
        } else {
            retval = 0;
        }
    }
    return retval + 1;
}

fn part1(filename: &str) -> u32 {
    let lines = aoc2020::lines_from_file(filename);
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut cnt = 0;
    for l in lines {
        let v: Vec<&str> = l.trim().split(|c| c == ' ').collect();
        let key = format!("{} {}", v[0], v[1]);
        let mut c: Vec<String> = Vec::new();
        for i in 0..(v.len() - 4) / 4 {
            let s = 4 + i * 4;
            let child = format!("{} {}", v[s + 1], v[s + 2]);
            c.push(child);
        }
        map.insert(key, c);
    }
    for e in &map {
        if dig(&map, e.0.to_string()) {
            cnt += 1;
        }
    }
    return cnt;
}

fn part2(filename: &str) -> u32 {
    let lines = aoc2020::lines_from_file(filename);
    let mut map: HashMap<String, Vec<(String, i32)>> = HashMap::new();
    for l in lines {
        let v: Vec<&str> = l.trim().split(|c| c == ' ').collect();
        let key = format!("{} {}", v[0], v[1]);
        let mut c: Vec<(String, i32)> = Vec::new();
        if v.len() == 7 {
            let child = (format!("-"), 0);
            //println!("{:?}", &child);
            c.push(child);
        } else {
            for i in 0..(v.len() - 4) / 4 {
                let s = 4 + i * 4;
                let child = (
                    format!("{} {}", v[s + 1], v[s + 2]),
                    v[s].parse::<i32>().unwrap(),
                );
                c.push(child);
            }
        }
        map.insert(key, c);
    }
    return dig2(&map, "shiny gold".to_string()) - 1;
}

fn main() {
    assert_eq!(part1("src/example.txt"), 4);
    assert_eq!(part1("src/puzzle.txt"), 169);
    assert_eq!(part2("src/example.txt"), 32);
    assert_eq!(part2("src/example2.txt"), 126);
    assert_eq!(part2("src/puzzle.txt"), 82372);
}
