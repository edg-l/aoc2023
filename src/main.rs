use std::{collections::HashSet, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.txt")?;

    let mut result: i64 = 0;

    for line in input.lines() {
        let (_, line) = line.split_once(": ").unwrap();
        let (winning, own) = line.split_once(" | ").unwrap();
        let winning: HashSet<i64> = winning
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let own: HashSet<i64> = own
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        let count = own.intersection(&winning).count() as u32;

        if count > 0 {
            result += 2i64.pow(count - 1);
        }
    }

    println!("{result}");

    Ok(())
}

// not 546029
// not 412998
