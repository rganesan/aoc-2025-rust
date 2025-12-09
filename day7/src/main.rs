use std::env;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn parse(filename: &str) -> Result<Vec<Vec<u8>>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let grid = reader
        .lines()
	.map(|line| line.expect("io error").as_bytes().to_vec())
        .collect::<Vec<_>>();
    // println!("grid={grid:?}");
    Ok(grid)
}


fn part1(grid: &[Vec<u8>]) -> usize {
    let start_col = grid[0].iter().position(|&c| c == b'S').expect("Start not found!");
    let mut beams = grid[0].clone(); // beams tracks all current beams
    beams[start_col] = b'|';
    let mut splits = 0;
    for r in 1..grid.len() {
	let ncols = grid[r].len();
	let row = &grid[r];
	println!("row {r} {}", String::from_utf8(row.to_vec()).unwrap());
	let mut new_beams = vec![b'.'; ncols];
	for c in 0..ncols {
	    match (beams[c], row[c]) {
		(b'|', b'.')  => { // beam can continue
		    // println!("Beam continues at {r},{c}");
		    new_beams[c] = b'|';
		},
		(b'|', b'^') => {	// beam splits
		    // println!("Beam splits at {r},{c}");
		    splits += 1;
		    if c != 0 {	// split left
			new_beams[c-1] = b'|';
		    }
		    if c < ncols - 1 { // split right
			new_beams[c+1] = b'|';
		    }
		},
		(_, _) => (),
	    }
	}
	beams = new_beams;
    }
    splits
}

fn tachyon_timelines(seen: &mut HashMap<(usize, usize), usize>, row: usize, grid: &[Vec<u8>], col: usize) -> usize {
    if row == grid.len() - 1 {
	// timeline ends
	return 1;
    }
    if let Some(timelines) = seen.get(&(row, col)) {
	println!("Seen row {row},{col} = {timelines}");
	return *timelines;
    } 
    println!("processing row {row},{col}");
    let timelines = match grid[row][col] {
	b'.' => {
	    // println!("Beam continues at {row},{col}");
	    tachyon_timelines(seen, row + 1, grid, col)
	},
	b'^' => {
	    // println!("Beam split at {row},{col}");
	    // this assumes no beam splitter at corners
	    tachyon_timelines(seen, row + 1, grid, col - 1) +
		tachyon_timelines(seen, row + 1, grid, col + 1)
	}
	_ => {
	    panic!("Shouldn't reach here!");
	}
    };
    seen.insert((row, col), timelines);
    timelines
}

fn part2(grid: &[Vec<u8>]) -> usize {
    let col = grid[0].iter().position(|&c| c == b'S').expect("Start not found!");
    let mut n = 0;
    let mut seen = HashMap::new();
    n = tachyon_timelines(&mut seen, 1, grid, col);
    n
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/test1.txt".to_string());
    let grid = parse(&filename)?;
    let start1 = Instant::now();
    let n1 = part1(&grid);
    let duration1 = start1.elapsed();
    println!("part1: {n1}, time: {duration1:?}");
    let start2 = Instant::now();
    let n2 = part2(&grid);
    let duration2 = start2.elapsed();
    println!("part2: {n2}, time: {duration2:?}");
    Ok(())
}
