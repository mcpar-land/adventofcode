use itertools::Itertools;

use crate::common::Pos;

use super::*;

#[derive(Debug)]
struct GalaxyMap(Vec<Vec<bool>>);

impl GalaxyMap {
	pub fn parse(input: &str) -> Self {
		let mut values: Vec<Vec<bool>> = input
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
		let size = values.len();
		let empty_rows: Vec<usize> = values
			.iter()
			.enumerate()
			.filter(|(_, row)| row.iter().all(|is_galaxy| !is_galaxy))
			.map(|(i, _)| i)
			.collect();
		let empty_cols: Vec<usize> = (0..values[0].len())
			.into_iter()
			.filter(|i| values.iter().map(|row| row[*i]).all(|is_galaxy| !is_galaxy))
			.collect();
		let mut offset = 0;
		for i_row in empty_rows {
			values.insert(i_row + offset, vec![false; size]);
			offset += 1;
		}
		let mut offset = 0;
		for i_col in empty_cols {
			for row in values.iter_mut() {
				row.insert(i_col + offset, false);
			}
			offset += 1;
		}

		Self(values)
	}
	fn print(&self) {
		let res = self
			.0
			.iter()
			.map(|row| {
				row
					.iter()
					.map(|is_galaxy| if *is_galaxy { '#' } else { '.' })
					.collect::<String>()
			})
			.collect::<Vec<String>>()
			.join("\n");
		println!("{}", res);
	}
	fn get(&self, pos: Pos) -> bool {
		self.0[pos.y as usize][pos.x as usize]
	}
	fn size(&self) -> Pos {
		Pos::new(self.0[0].len() as i32, self.0.len() as i32)
	}
	fn galaxy_positions(&self) -> Vec<Pos> {
		let size = self.size();
		let mut res = Vec::new();
		for y in 0..size.y {
			for x in 0..size.x {
				if self.get(Pos::new(x, y)) {
					res.push(Pos::new(x, y))
				}
			}
		}
		res
	}
	fn sum_distances(&self) -> i32 {
		self
			.galaxy_positions()
			.into_iter()
			.tuple_combinations()
			.map(|(a, b)| (a.x - b.x).abs() + (a.y - b.y).abs())
			.sum()
	}
}

fn day11_1(input: &str) -> ChallengeResult {
	let map = GalaxyMap::parse(input);

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
