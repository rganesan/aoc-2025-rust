use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn parse(filename: &str) -> Result<(Vec<Vec<u64>>, Vec<char>), std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let (mut numbers, mut ops) = (Vec::new(), Vec::new());
    let mut max_num = 0;
    for line in reader.lines() {
        let line = line?;
        let cols = line.split_whitespace().collect::<Vec<_>>();
        if cols[0].as_bytes()[0].is_ascii_digit() {
            // println!("numbers {cols:?}");
            let cols = cols
                .iter()
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            let cur_max = cols.iter().max().unwrap();
            if *cur_max > max_num {
                max_num = *cur_max;
            }
            numbers.push(cols);
        } else {
            ops = cols.iter().map(|op| op.chars().next().unwrap()).collect();
            // println!("ops {cols:?} {ops:?}");
        }
    }

    let rows = numbers.len();
    let cols = numbers[0].len();

    let mut transposed_numbers: Vec<Vec<u64>> = Vec::with_capacity(cols);

    for j in 0..cols {
        let mut new_row: Vec<u64> = Vec::with_capacity(rows);
        for i in 0..rows {
            new_row.push(numbers[i][j]);
        }
        transposed_numbers.push(new_row);
    }
    println!("{max_num}");
    Ok((transposed_numbers, ops))
}

fn parse2(filename: &str) -> Result<(Vec<String>, Vec<char>), std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
    // println!("{lines:?}");

    let ops = lines
        .pop()
        .unwrap()
        .split_whitespace()
        .map(|op| op.chars().next().unwrap())
        .collect();
    let rows = lines.len();
    let cols = lines[0].len();
    let mut inverted = Vec::with_capacity(cols);
    for j in 0..cols {
        let mut new_row: Vec<u8> = Vec::with_capacity(rows);
        for i in 0..rows {
            new_row.push(lines[i].as_bytes()[j]);
        }
	// println!("new_row: {}", String::from_utf8(new_row.clone()).unwrap());
        inverted.push(String::from_utf8(new_row).unwrap());
    }
    println!("{inverted:?} {ops:?}");

    Ok((inverted, ops))
}

fn part1(numbers: &[Vec<u64>], ops: &[char]) -> u64 {
    let mut sum = 0;
    for (i, op) in ops.iter().enumerate() {
        match op {
            '*' => sum += numbers[i].iter().product::<u64>(),
            '+' => sum += numbers[i].iter().sum::<u64>(),
            _ => panic!("Unexpected op {}", op),
        }
    }
    sum
}

fn part2(numbers: &[String], ops: &[char]) -> u64 {
    let mut iter = numbers.iter();
    let mut grand_total = 0;
    for op in ops {
        match op {
            '*' => {
		let prod: u64 = iter.by_ref().take_while(|col| !col.trim().is_empty())
		    .inspect(|n| println!("processing '{n}'"))
		    .map(|n| n.trim().parse::<u64>().unwrap())
		    .inspect(|n| println!("processing {n}"))
		    .product();
		println!("prod={prod}");
		grand_total += prod;
	    },
	    '+' => {
		let sum: u64 = iter.by_ref().take_while(|col| !col.trim().is_empty())
		    .inspect(|n| println!("processing '{n}'"))
		    .map(|n| n.trim().parse::<u64>().unwrap_or(0))
		    .inspect(|n| println!("processing {n}"))
		    .sum();
		println!("sum={sum}");
		grand_total += sum
	    },
            _ => panic!("Unexpected op {}", op),
        }
    }
    grand_total
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/input.txt".to_string());
    let (numbers, ops) = parse(&filename)?;
    let start1 = Instant::now();
    let n1 = part1(&numbers, &ops);
    let duration1 = start1.elapsed();
    println!("part1: {n1}, time: {duration1:?}");
    let start2 = Instant::now();
    let (numbers, ops) = parse2(&filename)?;
    let n2 = part2(&numbers, &ops);
    let duration2 = start2.elapsed();
    println!("part2: {n2}, time: {duration2:?}");
    Ok(())
}
