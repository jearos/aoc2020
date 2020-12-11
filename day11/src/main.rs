#[path = "../../common/aoc2020.rs"]
mod aoc2020;

fn part1(filename: &str) -> i32 {
    let lines = aoc2020::lines_from_file(filename);
    let x_size = lines[0].len() as usize;
    let y_size = lines.len() as usize;
    println!("{}x{}", x_size, y_size);
    let mut m: Vec<Vec<char>> = vec![vec!['X'; y_size]; x_size];
    let mut t: Vec<Vec<char>> = vec![vec!['X'; y_size]; x_size];
    let mut cnt = 0;

    for y in 0..y_size {
        let mut x = 0;
        for c in lines[y].chars() {
            m[x][y] = c;
            x = x + 1;
        }
    }
    loop {
        for y in 0..y_size {
            for x in 0..x_size {
                let mut occupied = 0;
                for dy in -1..2 {
                    let y_curr = y as isize + dy as isize;
                    if y_curr < 0 || y_curr >= y_size as isize {
                        continue;
                    }
                    for dx in -1..2 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let x_curr = x as isize + dx;
                        if x_curr < 0 || x_curr >= x_size as isize {
                            continue;
                        }
                        if x_curr as usize == x && y_curr as usize == y {
                            continue;
                        }
                        if m[x_curr as usize][y_curr as usize] == '#' {
                            occupied += 1;
                        }
                    }
                }
                if m[x][y] == '.' {
                    t[x][y] = '.';
                } else if m[x][y] == 'L' && occupied == 0 {
                    t[x][y] = '#';
                } else if m[x][y] == '#' && occupied >= 4 {
                    t[x][y] = 'L';
                }
            }
        }

        // for y in 0..y_size {
        //     for x in 0..x_size {
        //         print!("{}", t[x][y]);
        //     }
        //     println!("");
        // }
        if m == t {
            for y in 0..y_size {
                for x in 0..x_size {
                    if t[x][y] == '#' {
                        cnt += 1;
                    }
                }
            }
            break;
        } else {
            m = t.clone();
        }
    }
    return cnt;
}

fn part2(filename: &str) -> i32 {
    let lines = aoc2020::lines_from_file(filename);
    let x_size = lines[0].len() as usize;
    let y_size = lines.len() as usize;
    println!("{}x{}", x_size, y_size);
    let mut m: Vec<Vec<char>> = vec![vec!['X'; y_size]; x_size];
    let mut t: Vec<Vec<char>> = vec![vec!['X'; y_size]; x_size];
    let mut cnt = 0;

    for y in 0..y_size {
        let mut x = 0;
        for c in lines[y].chars() {
            m[x][y] = c;
            x = x + 1;
        }
    }
    loop {
        for y in 0..y_size {
            for x in 0..x_size {
                let mut occupied = 0;
                for dy in -1..2 {
                    for dx in -1..2 {
                        let mut x_curr = x as isize;
                        let mut y_curr = y as isize;
                        if !(dx == 0 && dy == 0) {
                            loop {
                                x_curr = x_curr as isize + dx;
                                y_curr = y_curr as isize + dy as isize;
                                if x_curr < 0 || x_curr >= x_size as isize {
                                    break;
                                }
                                if y_curr < 0 || y_curr >= y_size as isize {
                                    break;
                                }
                                if m[x_curr as usize][y_curr as usize] == '#' {
                                    occupied += 1;
                                    break;
                                }
                                if m[x_curr as usize][y_curr as usize] == 'L' {
                                    break;
                                }
                            }
                        }
                    }
                }
                //                print!("{}", occupied);
                if m[x][y] == '.' {
                    t[x][y] = '.';
                } else if m[x][y] == 'L' && occupied == 0 {
                    t[x][y] = '#';
                } else if m[x][y] == '#' && occupied >= 5 {
                    t[x][y] = 'L';
                }
            }
            //            println!("");
        }
        println!("");

        for y in 0..y_size {
            for x in 0..x_size {
                print!("{}", t[x][y]);
            }
            println!("");
        }
        if m == t {
            for y in 0..y_size {
                for x in 0..x_size {
                    if t[x][y] == '#' {
                        cnt += 1;
                    }
                }
                println!("");
            }
            break;
        } else {
            m = t.clone();
        }
    }
    return cnt;
}

fn main() {
    assert_eq!(part1("src/example.txt"), 37);
    assert_eq!(part1("src/puzzle.txt"), 2281);
    assert_eq!(part2("src/example.txt"), 26);
    assert_eq!(part2("src/puzzle.txt"), 26);
}
