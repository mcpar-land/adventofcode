use std::collections::HashSet;

use itertools::Itertools;

use crate::common::Pos;

use super::*;

#[derive(Debug)]
struct GalaxyMap(HashSet<Pos<u128>>);

impl GalaxyMap {
	pub fn parse(input: &str, expansion: u128) -> Self {
		let values: Vec<Vec<bool>> = input
			.lines()
			.map(|line| {
				line
					.chars()
					.map(|c| match c {
						'.' => false,
						'#' => true,
						c => panic!("Unrecognized symbol {}", c),
					})
					.collect()
			})
			.collect();

		let mut galaxies = Vec::new();
		for y in 0..values.len() {
			for x in 0..values[0].len() {
				if values[y][x] {
					galaxies.push(Pos::new(x as u128, y as u128));
				}
			}
		}

		let empty_rows: Vec<usize> = values
			.iter()
			.enumerate()
			.filter(|(_, row)| row.iter().all(|is_galaxy| !is_galaxy))
			.map(|(i, _)| i)
			.collect();

		let empty_cols: Vec<usize> = (0..values[0].len())
			.into_iter()
			.filter(|i| values.iter().map(|row| row[*i]).all(|is_galaxy| !is_galaxy))
			.unique()
			.collect();

		//TODO
		// almost entirely convinced this is wher the problem is.
		// why does this work when expansion is 1, but fails when it's other values?

		for (i, y) in empty_rows.into_iter().enumerate() {
			let offset = expansion * i as u128;
			for galaxy in galaxies.iter_mut().filter(|g| g.y > y as u128 + offset) {
				galaxy.y += expansion;
			}
		}

		for (i, x) in empty_cols.into_iter().enumerate() {
			let offset = expansion * i as u128;
			for galaxy in galaxies.iter_mut().filter(|g| g.x > x as u128 + offset) {
				galaxy.x += expansion;
			}
		}

		Self(galaxies.into_iter().collect())
	}

	fn sum_distances(&self) -> u128 {
		self
			.0
			.iter()
			.tuple_combinations()
			.map(|(a, b)| a.x.abs_diff(b.x) + a.y.abs_diff(b.y))
			.sum()
	}
}

fn day11_1(input: &str) -> ChallengeResult {
	let map = GalaxyMap::parse(input, 1);

	Ok(map.sum_distances() as u128)
}

const TEST1: &'static str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

submit!(Challenge {
	year: 2023,
	day: 11,
	part: 1,
	f: day11_1,
	unit_tests: &[(TEST1, 374)],
	skip: false
});

fn day11_2(input: &str) -> ChallengeResult {
	let map = GalaxyMap::parse(input, 100);

	Ok(map.sum_distances() as u128)
}

submit!(Challenge {
	year: 2023,
	day: 11,
	part: 2,
	f: day11_2,
	unit_tests: &[(TEST1, 8410)],
	skip: true
});
