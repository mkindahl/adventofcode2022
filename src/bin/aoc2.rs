use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

// A for Rock, B for Paper, and C for Scissors.
// X for Rock, Y for Paper, and Z for Scissors.
fn wscore1(line: Option<(&str, &str)>) -> u32 {
    match line.unwrap() {
        ("A", "X") => 3,
        ("B", "X") => 0,
        ("C", "X") => 6,
        ("A", "Y") => 6,
        ("B", "Y") => 3,
        ("C", "Y") => 0,
        ("A", "Z") => 0,
        ("B", "Z") => 6,
        ("C", "Z") => 3,
        (_, _) => panic!(),
    }
}

fn cscore1(line: Option<(&str, &str)>) -> u32 {
    match line.unwrap() {
        (_, "X") => 1,
        (_, "Y") => 2,
        (_, "Z") => 3,
        (_, _) => panic!(),
    }
}

// A for Rock, B for Paper, and C for Scissors.
// X for lose, Y for draw, and Z for win.
fn cscore2(line: Option<(&str, &str)>) -> u32 {
    match line.unwrap() {
        ("A", "X") => 3, // C
        ("B", "X") => 1, // A
        ("C", "X") => 2, // B
        ("A", "Y") => 1, // A
        ("B", "Y") => 2, // B
        ("C", "Y") => 3, // C
        ("A", "Z") => 2, // B
        ("B", "Z") => 3, // C
        ("C", "Z") => 1, // A
        (_, _) => panic!(),
    }
}

fn wscore2(line: Option<(&str, &str)>) -> u32 {
    match line.unwrap() {
        (_, "X") => 0,
        (_, "Y") => 3,
        (_, "Z") => 6,
        (_, _) => panic!(),
    }
}

fn compute(
    lines: &[Result<String, std::io::Error>],
    wscore: &dyn Fn(Option<(&str, &str)>) -> u32, // Whether you win or lose
    cscore: &dyn Fn(Option<(&str, &str)>) -> u32, // What you win or lose with
) -> u32 {
    let score: u32 = lines
        .iter()
        .map(|line| {
            let line = line.as_ref().unwrap();
            let x = line.split_once(' ');
            wscore(x) + cscore(x)
        })
        .sum();
    score
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc2.dat");
    let file = File::open(path)?;
    let input = BufReader::new(file);
    let lines: Vec<_> = input.lines().collect();
    println!("Part #1: {}", compute(&lines, &wscore1, &cscore1));
    println!("Part #2: {}", compute(&lines, &wscore2, &cscore2));
    Ok(())
}
