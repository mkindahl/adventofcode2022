use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc1.dat");
    let mut vals: Vec<u32> = read_to_string(&path)?
        .split("\n\n")
        .map(|group| group.lines().collect::<Vec<_>>())
        .map(|para| {
            para.iter()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();

    vals.sort();
    vals.reverse();
    println!("Part #1: {:?}", vals[0]);
    println!("Part #2: {:?}", vals[0..3].into_iter().sum::<u32>());

    Ok(())
}
