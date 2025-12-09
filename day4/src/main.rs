use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn parse(filename: &str) -> Result<Vec<Vec<bool>>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let grid = reader
        .lines()
        .map(|line| line.unwrap().chars().map(|c| c == '@').collect())
        .collect::<Vec<_>>();
    // println!("grid={grid:?}");
    Ok(grid)
}

fn pick_rolls(grid: &[Vec<bool>]) -> Vec<(usize,usize)> {
    let mut picked = Vec::new();
    let directions: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let (rows, cols) = (grid.len(), grid[0].len());
    for r in 0..rows {
        for c in 0..cols {
            if !grid[r][c] {
                // no rolls to remove
                continue;
            }
            let mut rolls = 0;
            for (dr, dc) in directions {
                let (adjr, adjc) = (r as isize + dr, c as isize + dc);
		if adjr < 0 || adjc < 0 { // lower bound 
		    continue;
		}
		let (adjr, adjc) = (adjr as usize, adjc as usize);
		if adjr >= rows || adjc >= cols { // uppper bound
		    continue;
		}
		if grid[adjr][adjc] {
                    rolls += 1;
		    if rolls >= 4 {
			break;
                    }
                };
            }
            if rolls < 4 {
                // println!("{r},{c}");
                picked.push((r, c));
            }
        }
    }
    picked
}

fn part1(grid: &[Vec<bool>]) -> usize {
    pick_rolls(grid).len()
}

fn part2(grid: &mut [Vec<bool>]) -> usize {
    let mut n = 0;
    loop {
	let picked = pick_rolls(grid);
        if picked.is_empty() {
            break;
        }
	n += picked.len();
        println!("removed={} total={n}", picked.len());
	for (r, c) in picked {
            grid[r][c] = false;
	}
    }
    n
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/input.txt".to_string());
    let mut grid = parse(&filename)?;
    let start1 = Instant::now();
    let n1 = part1(&grid);
    let duration1 = start1.elapsed();
    println!("part1: {n1}, time: {duration1:?}");
    let start2 = Instant::now();
    let n2 = part2(&mut grid);
    let duration2 = start2.elapsed();
    println!("part2: {n2}, time: {duration2:?}");
    Ok(())
}
