use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn max_joltage(bank: &str, digits: u32) -> u64 {
    let mut idx = 0;
    let mut max_digit = 0;
    let bank_digits = bank.as_bytes();
    // find max digit that can still form a number with "digits" digits
    for i in 0..bank_digits.len() - (digits as usize - 1) {
	if bank_digits[i] > max_digit {
	    max_digit = bank_digits[i];
	    idx = i;
	}
    }
    let max_digit = (max_digit - b'0') as u64;
    let joltage = if digits > 1 {
	// println!("{digits} {} {max_digit} ", &bank[idx+1..]);
	max_digit * (10 as u64).pow(digits - 1) + max_joltage(&bank[idx+1..], digits - 1)
    } else {
	// println!("{digits} {max_digit}");
	max_digit
    };
    joltage
}

fn parse(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let batteries = reader
        .lines()
	.collect::<Result<Vec<_>, _>>();
    // println!("{batteries:?}");
    batteries
}

fn part1(batteries: &[String]) -> u64 {
    let mut sum: u64 = 0;
    for bank in batteries {
        sum += max_joltage(bank, 2);
    }
    sum
}

fn part2(batteries: &[String]) -> u64 {
    let mut sum = 0;
    for bank in batteries {
	let joltage = max_joltage(bank, 12);
	println!("{bank} joltage={joltage}");
	sum += joltage;

    }
    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/input.txt".to_string());
    let batteries = parse(&filename)?;
    let start1 = Instant::now();
    let sum1 = part1(&batteries);
    let duration1 = start1.elapsed();
    println!("part1: {sum1}, time: {duration1:?}");
    let start2 = Instant::now();
    let sum2 = part2(&batteries);
    let duration2 = start2.elapsed();
    println!("part2: {sum2}, time: {duration2:?}");
    Ok(())
}
