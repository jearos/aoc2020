#[path = "../../common/aoc2020.rs"]
mod aoc2020;

use std::collections::HashMap;

// light red bags contain 1 bright white bag, 2 muted yellow bags.
// dark orange bags contain 3 bright white bags, 4 muted yellow bags.
// bright white bags contain 1 shiny gold bag.
// muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
// shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
// dark olive bags contain 3 faded blue bags, 4 dotted black bags.
// vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
// faded blue bags contain no other bags.
// dotted black bags contain no other bags.

fn dig(map: &HashMap<String, Vec<String>>, name: String) -> bool {
    let mut retval: bool = false;
    let v = &map[&name];
    for e in v {
        //print!("{} ", &e);
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

fn dig2(map: &HashMap<String, Vec<String>>, name: String) -> u32 {
    let mut retval: u32 = 0;
    let v = &map[&name];
    for e in v {
        if e.len() == 0 {
            retval += dig2(map, e.to_string());
        } else {
            println!("kals ");
            retval += 1;
        }
    }
    return retval;
}

fn part1(filename: &str) -> u32 {
    let mut lines = aoc2020::lines_from_file(filename);
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut cnt = 0;
    for l in lines {
        let v: Vec<&str> = l.trim().split(|c| c == ' ').collect();
        let key = format!("{} {}", v[0], v[1]);
        let mut c: Vec<String> = Vec::new();
        for i in 0..(v.len() - 4) / 4 {
            let s = 4 + i * 4;
            let child;
            if v[s] != "no" {
                //let child = format!("{} {} {}", v[s], v[s + 1], v[s + 2]);
                child = format!("{} {}", v[s + 1], v[s + 2]);
                println!("{:?}", &child);
                c.push(child);
            }
        }
        map.insert(key, c);
    }
    println!("{:?}", &map);
    for e in &map {
        if dig(&map, e.0.to_string()) {
            cnt += 1;
        }
    }
    return cnt;
}

fn part2(filename: &str) -> u32 {
    let mut lines = aoc2020::lines_from_file(filename);
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut cnt = 0;
    for l in lines {
        let v: Vec<&str> = l.trim().split(|c| c == ' ').collect();
        let key = format!("{} {}", v[0], v[1]);
        let mut c: Vec<String> = Vec::new();
        for i in 0..(v.len() - 4) / 4 {
            let s = 4 + i * 4;
            let child;
            if v[s] != "no" {
                //let child = format!("{} {} {}", v[s], v[s + 1], v[s + 2]);
                child = format!("{} {}", v[s + 1], v[s + 2]);
                println!("{:?}", &child);
                c.push(child);
            }
        }
        map.insert(key, c);
    }
    println!("{:?}", &map);
    return dig2(&map, "shiny gold".to_string());
}

// let mut ans: HashMap<char, u32> = HashMap::new();
// let mut nof_yes = 0;
// let mut nof_persons = 0;

// for l in lines {
//     if l.len() == 0 {
//         for (_key, val) in &ans {
//             if *val == nof_persons {
//                 nof_yes += 1;
//             }
//         }
//         nof_persons = 0;
//         ans.clear();
//     } else {
//         for c in l.chars() {
//             let yes = ans.entry(c).or_insert(0u32);
//             *yes += 1;
//         }
//         nof_persons += 1;
//     }
// }
// return nof_yes;

fn main() {
    assert_eq!(part1("src/example.txt"), 4);
    assert_eq!(part1("src/puzzle.txt"), 169);
    assert_eq!(part2("src/example.txt"), 32);
}
