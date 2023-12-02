use std::error::Error;

use rayon::{iter::ParallelIterator, str::ParallelString};

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.txt")?;

    let result: usize = input
        .lines()
        .map(|line| {
            let line = line.split_once(": ").unwrap().1;
            let sets = line.split("; ");

            let mut blue = 0;
            let mut red = 0;
            let mut green = 0;

            for set in sets {
                let items = set.split(',');
                for item in items {
                    let (num, color) = item.trim().split_once(' ').unwrap();
                    let num: usize = num.parse().unwrap();
                    match color {
                        "blue" => blue = blue.max(num),
                        "red" => red = red.max(num),
                        "green" => green = green.max(num),
                        _ => unreachable!(),
                    }
                }
            }

            red * blue * green
        })
        .sum();

    println!("{result}");

    Ok(())
}
