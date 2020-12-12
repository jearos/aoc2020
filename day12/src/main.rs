#[path = "../../common/aoc2020.rs"]
mod aoc2020;

fn calc_dist(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    let x_dist = p1.0 - p2.0;
    let y_dist = p1.1 - p2.1;
    x_dist.abs() + y_dist.abs()
}

fn part1(filename: &str) -> i32 {
    let lines = aoc2020::lines_from_file(filename);
    let mut heading = num_complex::Complex32::new(1.0, 0.0);
    let mut pos = num_complex::Complex32::new(0.0, 0.0);
    for l in lines {
        let cmd = l.chars().nth(0).unwrap();
        let val = &l[1..l.len()].parse::<f32>().unwrap();

        match cmd {
            'N' => pos.im += val,
            'S' => pos.im -= val,
            'E' => pos.re += val,
            'W' => pos.re -= val,
            'L' => {
                for _ in 0..*val as usize / 90 {
                    heading *= num_complex::Complex32::i()
                }
            }
            'R' => {
                for _ in 0..*val as usize / 90 {
                    heading /= num_complex::Complex32::i()
                }
            }
            'F' => pos += heading * val,
            _ => println!("Error"),
        }
    }

    return calc_dist((0, 0), (pos.re as i32, pos.im as i32));
}

fn part2(filename: &str) -> i32 {
    let lines = aoc2020::lines_from_file(filename);
    let mut pos = num_complex::Complex32::new(0.0, 0.0);
    let mut wp = num_complex::Complex32::new(10.0, 1.0);
    for l in lines {
        let cmd = l.chars().nth(0).unwrap();
        let val = &l[1..l.len()].parse::<f32>().unwrap();

        match cmd {
            'N' => wp.im += val,
            'S' => wp.im -= val,
            'E' => wp.re += val,
            'W' => wp.re -= val,
            'L' => {
                for _ in 0..*val as usize / 90 {
                    wp = wp * num_complex::Complex32::i()
                }
            }
            'R' => {
                for _ in 0..*val as usize / 90 {
                    wp = wp / num_complex::Complex32::i()
                }
            }
            'F' => pos = pos + wp * val,
            _ => println!("Error"),
        }
    }

    return calc_dist((0, 0), (pos.re as i32, pos.im as i32));
}

fn main() {
    assert_eq!(part1("src/example.txt"), 25);
    assert_eq!(part1("src/puzzle.txt"), 445);
    assert_eq!(part2("src/example.txt"), 286);
    assert_eq!(part2("src/puzzle.txt"), 42495);
}
