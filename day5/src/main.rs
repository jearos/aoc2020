#[path = "../../common/aoc2020.rs"]
mod aoc2020;

fn get_row(bp: &str) -> u32 {
    let mut min = 0;
    let mut max = 127;
    for i in 0..7 {
        let range = (max + 1 - min) / 2;
        if bp.chars().nth(i).unwrap() == 'F' {
            max = max - range;
        } else {
            min = min + range;
        }
    }
    return min;
}

fn get_column(bp: &str) -> u32 {
    let mut min = 0;
    let mut max = 7;
    for i in 7..10 {
        let range = (max + 1 - min) / 2;
        if bp.chars().nth(i).unwrap() == 'L' {
            max = max - range;
        } else {
            min = min + range;
        }
    }
    return min;
}

fn scan_line(bp: &str) -> u32 {
    return get_row(bp) * 8 + get_column(bp);
}

fn scan_line2(bp: &str) -> i32 {
    let row = get_row(bp);
    if row == 0 || row == 127 {
        return -1;
    }

    let column = get_column(bp);

    return (row * 8 + column) as i32;
}

fn part1(filename: &str) -> u32 {
    let lines = aoc2020::lines_from_file(filename);
    let mut max = 0;
    for l in lines {
        let id = scan_line(&l.to_string());
        if id > max {
            max = id;
        }
    }
    return max;
}

fn part2(filename: &str) -> i32 {
    let lines = aoc2020::lines_from_file(filename);
    let mut v: Vec<i32> = Vec::new();
    for l in lines {
        let id = scan_line2(&l.to_string());
        v.push(id);
    }
    v.sort();
    for i in 0..v.len() - 1 {
        if v[i + 1] - v[i] == 2 {
            return v[i] + 1;
        }
    }
    return 0;
}

fn main() {
    assert_eq!(scan_line("FBFBBFFRLR"), 357);
    assert_eq!(scan_line("BFFFBBFRRR"), 567);
    assert_eq!(scan_line("FFFBBBFRRR"), 119);
    assert_eq!(scan_line("BBFFBBFRLL"), 820);
    assert_eq!(part1("src/puzzle.txt"), 998);
    assert_eq!(part2("src/puzzle.txt"), 676);
}
