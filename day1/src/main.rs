use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn part1(filename: &str) -> Result<u32, > {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut dial: i32 = 50;
    let mut zeros: u32 = 0;
    for line in reader.lines() {
        let line = line?;
        let n = line[1..].parse::<i32>()?;
	if line.chars().next() == Some('L') {
	    dial -= n;
	} else {
	    dial += n;
	}
	dial %= 100;
	if dial == 0 {
	    zeros += 1;
	} else if dial < 0 {
	    dial += 100;
	}
    }
    Ok(zeros)
}

fn part2(filename: &str) -> Result<u32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut dial: i32 = 50;
    let mut zeros: u32 = 0;
    for line in reader.lines() {
        let line = line?;
        let mut n = line[1..].parse::<i32>()?;
	if n >= 100 {
	    zeros += n as u32 / 100;
	}
	n %= 100;
	if line.chars().next() == Some('L') {
	    if n >= dial {
		if dial != 0 {
		    zeros += 1;
		    println!("touched 0: dial={dial} {line} n={n} zeros={zeros}");
		}
		if n > dial {
		    dial += 100;
		}
	    }
	    dial -= n;
	} else {
	    if dial + n >= 100 {
		zeros += 1;
		println!("touched 0: dial={dial} {line} n={n} dial={dial} zeros={zeros}");
	    }
	    dial = (dial + n) % 100;
	}
    }
    Ok(zeros)
}

fn main() -> Result<()> {
    let filename = env::args().nth(1).unwrap_or_else(|| "inputs/input.txt".to_string());
    let start1 = Instant::now();
    let sum1 = part1(&filename).expect("failed");
    let duration1 = start1.elapsed();
    println!("part1: {sum1}, time: {duration1:?}");
    let start2 = Instant::now();
    let sum2 = part2(&filename).expect("failed");
    let duration2 = start2.elapsed();
    println!("part2: {sum2}, time: {duration2:?}");
    Ok(())
}
