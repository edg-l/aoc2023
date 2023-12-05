use std::{collections::HashMap, error::Error};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<_> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|x| x.chars().collect_vec())
        .collect_vec();

    let mut result: i64 = 0;
    let mut current_num = String::with_capacity(16);

    let height = input.len();
    let width = input[0].len();

    let mut gear_map: HashMap<(usize, usize), Vec<i64>> = HashMap::new();

    for y in 0..height {
        let mut is_part_num = None;
        current_num.clear();

        for x in 0..width {
            let c = input[y][x];

            if c.is_ascii_digit() {
                current_num.push(c);
            }

            for dy in -1i64..2 {
                for dx in -1i64..2 {
                    // check bounds
                    if (y == 0 && dy < 0)
                        || (y == (height - 1) && dy > 0)
                        || (x == 0 && dx < 0)
                        || (x == (width - 1) && dx > 0)
                    {
                        continue;
                    }
                    let coordy = ((y as i64) + dy) as usize;
                    let coordx = ((x as i64) + dx) as usize;
                    let v = input.get(coordy).and_then(|a| a.get(coordx)).unwrap();

                    if !v.is_ascii_digit() && *v == '*' && !current_num.is_empty() {
                        is_part_num = Some((coordx, coordy));
                    }
                }
            }

            let cc = input[y].get(x + 1);
            if !cc.map(|a| a.is_ascii_digit()).unwrap_or(false) {
                if let Some(is_part_num) = is_part_num {
                    if !current_num.is_empty() {
                        let gear = gear_map.entry(is_part_num).or_default();
                        gear.push(current_num.parse::<i64>().unwrap());
                    }
                }
                current_num.clear();
                is_part_num = None;
            }
        }
    }

    for gear in gear_map.values() {
        if gear.len() == 2 {
            result += gear[0] * gear[1];
        }
    }

    println!("{result}");

    Ok(())
}
