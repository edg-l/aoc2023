use std::{collections::VecDeque, error::Error, ops::Range};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut lines = input.lines();

    let numbers = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec();

    lines.nth(1);

    let mut numbers: VecDeque<Range<u64>> = {
        let mut it = numbers.iter();
        let mut new: VecDeque<Range<u64>> = Default::default();

        while let Some(x) = it.next() {
            let len = it.next().unwrap();
            new.push_back(*x..(x + len));
        }

        new
    };
    let mut new_numbers = VecDeque::with_capacity(numbers.len());

    while let Some(line) = lines.next() {
        if line.is_empty() {
            numbers.extend(new_numbers.clone());
            new_numbers.clear();
            lines.next();
        } else {
            let (dst, src, len): (u64, u64, u64) = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();

            let mut i = numbers.len();

            while i > 0 {
                i -= 1;
                let src_range = src..(src + len);
                let dst_range = dst..(dst + len);

                if let Some(n) = numbers.pop_front() {
                    if n.start >= src_range.start && n.start < src_range.end {
                        // intersects at end

                        let end = n.end.min(src_range.end);
                        let diff_start = n.start - src_range.start;
                        let diff_end = end - n.start;

                        new_numbers.push_back(
                            (dst_range.start + diff_start)
                                ..(dst_range.start + diff_start + diff_end),
                        );

                        if n.end > src_range.end {
                            let end_range = src_range.end..(n.end);
                            numbers.push_front(end_range);
                            i += 1;
                        }
                    } else if n.end > src_range.start && n.end <= src_range.end {
                        // intersects at start

                        let start = n.start.max(src_range.start);
                        let diff_start = start - src_range.start;
                        let diff = n.end - start;
                        new_numbers.push_back(
                            (dst_range.start + diff_start)..(dst_range.start + diff_start + diff),
                        );

                        if n.start < src_range.start {
                            let start_range = n.start..(src_range.start);
                            numbers.push_front(start_range);
                            i += 1;
                        }
                    } else if n.start < src_range.start && n.end > src_range.end {
                        let first_range = n.start..(src_range.start);
                        let last_range = src_range.end..(n.end);
                        numbers.push_front(first_range);
                        numbers.push_front(last_range);
                        i += 2;
                        new_numbers.push_back(dst_range);
                    } else {
                        numbers.push_back(n);
                    }
                }
            }
        }
    }

    let x = new_numbers
        .iter()
        .chain(numbers.iter())
        .map(|x| x.start)
        .min()
        .unwrap();
    println!("{x}");

    Ok(())
}
