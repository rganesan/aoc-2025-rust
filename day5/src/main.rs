use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn parse(filename: &str) -> Result<(Vec<(u64,u64)>, Vec<u64>), std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let (mut ranges, mut ids) = (Vec::new(), Vec::new());
    for line in reader.lines() {
	let line = line?;
	if line.is_empty() {
	    // this just separates valid ranges from the ids
	    continue;
	}
				
        if let Some((start, end)) = line.split_once('-') {
            ranges.push((start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()));
        } else {
	    ids.push(line.parse::<u64>().unwrap());
        }
    }
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
	
    // println!("ranges={ranges:?}");
    Ok((ranges, ids))
}

fn part1(ranges: &[(u64,u64)], ids: &[u64]) -> u64 {
    let mut n = 0;
    for id in ids {
	for (start, end) in ranges {
	    if id >= start && id <= end {
		n += 1;
		break;
	    }
	}
    }
    n
}

fn part2(ranges: &[(u64,u64)]) -> u64 {
    let mut n = 0;
    let mut i = 1;
    let (mut pstart, mut pend) = ranges[0];
    while i < ranges.len() {
	match ranges[i] {
	    (start, end) if start > pend => {
		// no overlap
		n += pend - pstart + 1;
		println!("counted {pstart}-{pend} running total={n}");
		(pstart, pend) = (start, end);
	    },
	    (start, end) if start >= pstart && end <= pend => {
		// full overlap, skip
		println!("{start}-{end} fully covered by {pstart}-{pend}");
	    },
	    (start, end) if start >= pstart && end >= pend => {
		// range extended
		println!("{start}-{end} extends range {pstart}-{pend}");
		pend = end;
	    },
	    (start, end) => {
		println!("Unprocessed range {start}-{end}, previous {pstart}-{pend}");
		todo!();
	    }
	}
	i += 1;
    }
    // count remaining range
    n += pend - pstart + 1;
    println!("counted {pstart}-{pend} running total={n}");
    n
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/input.txt".to_string());
    let (ranges, ids) = parse(&filename)?;
    let start1 = Instant::now();
    let n1 = part1(&ranges, &ids);
    let duration1 = start1.elapsed();
    println!("part1: {n1}, time: {duration1:?}");
    let start2 = Instant::now();
    let n2 = part2(&ranges);
    let duration2 = start2.elapsed();
    println!("part2: {n2}, time: {duration2:?}");
    Ok(())
}
