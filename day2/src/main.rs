use anyhow::Result;
use std::cmp::max;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn pattern_sum(from: &str, to: &str, repeats: usize) -> Vec<u64> {
    // repeats is number of times pattern repeats
    let mut invalid_ids = Vec::new();
    // println!("given range: {from}-{to} repeats={repeats}");
    if from.len() % repeats != 0 && to.len() % repeats != 0 {
        // println!("no invalid IDs possible in range: {from}-{to} for repeats={repeats}");
        return invalid_ids;
    }
    let (mut from, mut to) = (from.to_string(), to.to_string());
    if from.len() < to.len() {
        // println!("tricky range: {from}-{to}");
        if from.len() % repeats != 0 {
            // need to round "from" up
            let len: u32 = to.len() as u32 - 1;
            from = 10u64.pow(len).to_string();
        } else {
            assert!(to.len() % repeats != 0);
            // need to round "to" down
            let len: u32 = from.len() as u32;
            to = (10u64.pow(len) - 1).to_string();
        }
    }
    // println!("adjusted range: {from}-{to} repeats={repeats}");
    assert!(from.len() == to.len());
    let (from_num, to_num) = (from.parse::<u64>().unwrap(), to.parse::<u64>().unwrap());
    let range = from_num..=to_num;
    let pat_len = from.len() / repeats;
    let pat_from = from[0..pat_len].parse::<u64>().unwrap();
    let pat_to = to[0..pat_len].parse::<u64>().unwrap();
    // println!("Trying: {pat_from}-{pat_to} repeat={repeats}");
    for i in pat_from..=pat_to {
        let id = i.to_string().repeat(repeats).parse::<u64>().unwrap();
        // println!("id={id}");
        if range.contains(&id) {
            invalid_ids.push(id);
            // println!("{id} falls in range, repeats={repeats}!");
        }
        if id > to_num {
            // println!("{id} exceeded range!");
            break;
        }
    }
    // println!(
    //     "{from}-{to} total={} invalid={} repeats={repeats}",
    //     to_num - from_num + 1,
    //     invalid_ids.len()
    // );
    invalid_ids
}

fn parse(filename: &str) -> Result<Vec<(String, String)>> {
    let line = fs::read_to_string(filename)?;
    let line = line.trim();
    let ranges = line
        .split(',')
        .map(|s| match s.split_once('-') {
            Some((from, to)) => (from.to_string(), to.to_string()),
            _ => {
                panic!("Invalid range");
            }
        })
        .collect::<Vec<_>>();
    Ok(ranges)
}

fn part1(ranges: &[(String, String)]) -> u64 {
    let mut sum = 0;
    for (from, to) in ranges {
        sum += pattern_sum(from, to, 2).into_iter().sum::<u64>();
    }
    sum
}

fn part2(ranges: &[(String, String)]) -> u64 {
    let mut sum = 0;
    for (from, to) in ranges {
        let mut invalid_ids = HashSet::new();
        let max_repeats = max(from.len(), to.len());
        for repeats in 2..=max_repeats {
            invalid_ids.extend(pattern_sum(from, to, repeats).into_iter());
        }
        sum += invalid_ids.into_iter().sum::<u64>();
    }
    sum
}

fn main() -> Result<()> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/input.txt".to_string());
    let ranges = parse(&filename)?;
    let start1 = Instant::now();
    let sum1 = part1(&ranges);
    let duration1 = start1.elapsed();
    println!("part1: {sum1}, time: {duration1:?}");
    let start2 = Instant::now();
    let sum2 = part2(&ranges);
    let duration2 = start2.elapsed();
    println!("part2: {sum2}, time: {duration2:?}");
    Ok(())
}
