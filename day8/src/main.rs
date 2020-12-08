use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
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

fn run_program(program: &Vec<Vec<&str>>) -> (i32, bool) {
    let mut pc = 0;
    let mut acc = 0;
    let mut ran: Vec<bool> = vec![false; program.len()];
    loop {
        let v = &program[pc as usize];
        if ran[pc as usize] == true {
            return (acc, true);
        }
        ran[pc as usize] = true;
        match v[0] {
            "nop" => pc += 1,
            "acc" => {
                acc += v[1].parse::<i32>().unwrap();
                pc += 1
            }
            "jmp" => {
                pc += v[1].parse::<i32>().unwrap();
            }
            _ => {
                pc += 1;
                println!("Illegal instruction")
            }
        }
        if pc == program.len() as i32 {
            break;
        }
    }
    return (acc, false);
}

fn part1(filename: &str) -> (i32, bool) {
    let lines = lines_from_file(filename);
    let mut program: Vec<Vec<&str>> = Vec::new();
    for l in &lines {
        let v = l.trim().split(|c| c == ' ').collect();
        program.push(v);
    }
    return run_program(&program);
}

fn part2(filename: &str) -> (i32, bool) {
    let lines = lines_from_file(filename);
    let mut program: Vec<Vec<&str>> = Vec::new();
    for l in &lines {
        let v = l.trim().split(|c| c == ' ').collect();
        program.push(v);
    }

    for i in 0..program.len() {
        if program[i][0] == "nop" {
            program[i][0] = "jmp";
            let res = run_program(&program);
            if res.1 == false {
                return res;
            }
            program[i][0] = "nop";
        } else if program[i][0] == "jmp" {
            program[i][0] = "nop";
            let res = run_program(&program);
            if res.1 == false {
                return res;
            }
            program[i][0] = "jmp";
        }
    }
    return run_program(&program);
}

fn main() {
    assert_eq!(part1("src/example.txt"), (5, true));
    assert_eq!(part1("src/puzzle.txt"), (1262, true));
    assert_eq!(part2("src/example.txt"), (8, false));
    assert_eq!(part2("src/puzzle.txt"), (1643, false));
}
