use std::env;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct Point3D(i64, i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PairwiseDistance(u64, usize, usize);

fn parse(filename: &str) -> Result<Vec<Point3D>, Box<dyn Error + 'static>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let points = reader
        .lines()
	.map(|line| {
	    let line = line?;
	    match line.split(',').collect::<Vec<_>>()[..] {
		[x, y, z] => Ok(Point3D(x.parse::<i64>()?, y.parse::<i64>()?, z.parse::<i64>()?)),
		_ => panic!("Unexpected line {line}")
	    }
	}).collect::<Result<Vec<_>, _>>();
    // println!("{:?}", points);
    points
}

fn get_dist_index(i: usize, j: usize, n_points: usize) -> usize {
    // condensed index for distance between points
    n_points * i - i * (i + 1) / 2 + j - 1 - i
}

fn get_distances_sq(points: &[Point3D]) -> Vec<PairwiseDistance> {
    let n = points.len();
    let mut distances_sq = Vec::with_capacity(n * (n - 1) / 2);

    for i in 0..n {
	for j in (i + 1)..n {
	    let (p1, p2) = (points[i], points[j]);
	    let (dx, dy, dz) = (p1.0 - p2.0, p1.1 - p2.1, p1.2 - p2.2);
	    let dist = (dx * dx + dy * dy + dz * dz) as u64;
	    let idx = get_dist_index(i, j, n);
	    assert!(idx == distances_sq.len());
	    distances_sq.push(PairwiseDistance(dist, i, j));
	}
    }
    distances_sq
}
    

fn part1(points: &[Point3D]) -> usize {
    let mut distances_sq = get_distances_sq(points);
    distances_sq.sort();
    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    for dist in &distances_sq[..10] {
	// println!("Processing {dist:?} {:?}-{:?})", points[dist.1], points[dist.2]);
	let mut join_idx = None;
	let mut other_point = None;
	for (i, c) in circuits.iter_mut().enumerate() {
	    // println!("Checking circuit: {c:?}");
	    if c.contains(&dist.1) {
		other_point = Some(dist.2);
	    } else if c.contains(&dist.2) {
		other_point = Some(dist.1);
	    }
	    if let Some(p) = other_point {
		// println!("Found circuit: {c:?} with {p:?}");
		c.insert(p);
		join_idx = Some(i);
		break;
	    }
	}
	if let Some(p) = other_point {
	    let join_idx = join_idx.unwrap();
	    for j in (join_idx + 1)..circuits.len() {
		if circuits[j].contains(&p) {
		    let otherc = circuits.remove(j);
		    // println!("Found duplicate circuit at {j}: {:?}", otherc);
		    circuits[join_idx].extend(otherc);
		    break;
		}
	    }
	} else {
	    // new circuit
	    let mut new_circuit = HashSet::new();
	    new_circuit.insert(dist.1);
	    new_circuit.insert(dist.2);
	    // println!("New circuit: {new_circuit:?} {:?}-{:?})", points[dist.1], points[dist.2]);
	    circuits.push(new_circuit);
	}
    }
    circuits.sort_by(|a,b| b.len().cmp(&a.len()));
    println!("{:?}", &circuits[..3]);
    circuits[..3].iter().map(|c| c.len()).product()
}


fn part2(points: &[Point3D]) -> u64 {
    let mut distances_sq = get_distances_sq(points);
    distances_sq.sort();
    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    let mut processed_points = HashSet::new();
    let mut answer = 0;
    for dist in &distances_sq {
	processed_points.insert(dist.1);
	processed_points.insert(dist.2);
	let mut join_idx = None;
	let mut other_point = None;
	for (i, c) in circuits.iter_mut().enumerate() {
	    // println!("Checking circuit: {c:?}");
	    if c.contains(&dist.1) {
		other_point = Some(dist.2);
	    } else if c.contains(&dist.2) {
		other_point = Some(dist.1);
	    }
	    if let Some(p) = other_point {
		c.insert(p);
		// println!("Added {p:?} to circuit {i}: {c:?}");
		join_idx = Some(i);
		break;
	    }
	}
	if let Some(p) = other_point {
	    let join_idx = join_idx.unwrap();
	    for j in (join_idx + 1)..circuits.len() {
		if circuits[j].contains(&p) {
		    let otherc = circuits.remove(j);
		    // println!("Found duplicate circuit at {j}: {:?}", otherc);
		    circuits[join_idx].extend(otherc);
		    // println!("Extended circuit at {join_idx}: {:?}", circuits[join_idx]);
		    break;
		}
	    }
	} else {
	    // new circuit
	    let mut new_circuit = HashSet::new();
	    new_circuit.insert(dist.1);
	    new_circuit.insert(dist.2);
	    // println!("New circuit {}: {new_circuit:?} {:?}-{:?})", circuits.len(), points[dist.1], points[dist.2]);
	    circuits.push(new_circuit);
	}
	// println!("Processing {dist:?} {:?}-{:?}, circuits: {})", points[dist.1], points[dist.2], circuits.len());
	if processed_points.len() == points.len() && circuits.len() == 1 {
	    answer = points[dist.1].0 * points[dist.2].0;
	    break;
	}
    }
    answer as u64
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/test1.txt".to_string());
    let points = parse(&filename)?;
    let start1 = Instant::now();
    let n1 = part1(&points);
    let duration1 = start1.elapsed();
    println!("part1: {n1}, time: {duration1:?}");
    let start2 = Instant::now();
    let n2 = part2(&points);
    let duration2 = start2.elapsed();
    println!("part2: {n2}, time: {duration2:?}");
    Ok(())
}
