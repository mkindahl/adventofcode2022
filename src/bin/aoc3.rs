use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

pub fn prio(ch: &char) -> u32 {
    match ch {
        'a'..='z' => *ch as u32 - 'a' as u32 + 1,
        'A'..='Z' => *ch as u32 - 'A' as u32 + 27,
        _ => panic!(),
    }
}

fn part1(lines: &[String]) -> u32 {
    lines
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(left, right)| {
            let l: HashSet<char> = left.chars().collect();
            let r: HashSet<char> = right.chars().collect();
            l.intersection(&r).map(prio).sum::<u32>()
        })
        .sum()
}

fn part2(lines: &[String]) -> u32 {
    lines
        .chunks(3)
        .map(|chunk| {
            chunk
                .iter()
                .map(|line| line.chars().collect::<HashSet<char>>())
                .reduce(|a, b| a.intersection(&b).cloned().collect())
        })
        .map(|set| set.unwrap().iter().map(prio).sum::<u32>())
        .sum::<u32>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc3.dat");
    let file = File::open(path)?;
    let input = BufReader::new(file);
    let lines: Result<Vec<_>, _> = input.lines().collect();
    println!("Part #1: {}", part1(lines.as_ref().unwrap()));
    println!("Part #2: {}", part2(lines.as_ref().unwrap()));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_prio() {
        assert_eq!(prio(&'a'), 1);
        assert_eq!(prio(&'A'), 27);
        assert_eq!(prio(&'z'), 26);
        assert_eq!(prio(&'Z'), 52);
    }
}
