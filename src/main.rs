use std::error::Error;

use aho_corasick::AhoCorasick;
use rayon::{str::ParallelString, iter::ParallelIterator};

#[inline]
fn match_str(str: &str) -> u32 {
    match str {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => unreachable!(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.txt")?;

    let number_keys = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    let ac = AhoCorasick::new(number_keys).unwrap();

    let result: u32 = input
        .lines()
        .map(|line| {
            let mut it = ac.find_overlapping_iter(line);
            let x = it.next().unwrap();
            let y = it.last().unwrap_or(x);
            let first = match_str(&line[x.range()]);
            let last = match_str(&line[y.range()]);
            first * 10 + last
        })
        .sum();

    println!("{result}");

    Ok(())
}
