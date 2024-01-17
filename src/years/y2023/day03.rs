use std::collections::{HashMap, HashSet};

use super::*;

#[derive(Debug)]
struct PartNumber {
	pub pos: (usize, usize),
	pub value: u128,
	pub len: usize,
}

impl PartNumber {
	pub fn new(x: usize, y: usize, val: &str) -> Result<Self, anyhow::Error> {
		Ok(Self {
			pos: (x, y),
			value: val.parse()?,
			len: val.len(),
		})
	}
	pub fn adjacent_indices(&self) -> Vec<(usize, usize)> {
		/*
			abbbbc
			a1234c
			abbbbc
		*/
		let mut res = vec![
			(self.pos.0.checked_sub(1), self.pos.1.checked_sub(1)), // a
			(self.pos.0.checked_sub(1), Some(self.pos.1 + 0)),      // a
			(self.pos.0.checked_sub(1), Some(self.pos.1 + 1)),      // a
			(
				Some(self.pos.0 + self.len),
				Some(self.pos.1.saturating_sub(1)),
			), // c
			(Some(self.pos.0 + self.len), Some(self.pos.1 + 0)),    // c
			(Some(self.pos.0 + self.len), Some(self.pos.1 + 1)),    // c
		];

		for x in self.pos.0..(self.pos.0 + self.len) {
			res.push((Some(x), self.pos.1.checked_sub(1)));
			res.push((Some(x), Some(self.pos.1 + 1)));
		}

		res
			.into_iter()
			.filter_map(|coord| match coord {
				(Some(x), Some(y)) => Some((x, y)),
				_ => None,
			})
			.collect()
	}
}

struct Schematic {
	pub symbols: HashMap<(usize, usize), char>,
	pub part_numbers: Vec<PartNumber>,
}

impl Schematic {
	pub fn parse(input: &str) -> Result<Self, anyhow::Error> {
		let schematic = input.lines().collect::<Vec<&str>>();

		let mut symbols = HashMap::<(usize, usize), char>::new();

		for (y, row) in schematic.iter().enumerate() {
			for (x, c) in row.chars().enumerate() {
				match c {
					'.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
						continue
					}
					c => {
						symbols.insert((x, y), c);
					}
				}
			}
		}

		let mut part_numbers = Vec::<PartNumber>::new();

		for (y, row) in schematic.iter().enumerate() {
			let mut chars = row.chars().enumerate();
			let mut current_num: Option<String> = None;
			while let Some((x, c)) = chars.next() {
				match c {
					'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
						if let Some(current_num) = current_num.as_mut() {
							current_num.push(c);
						} else {
							current_num = Some(format!("{}", c));
						}
					}
					_ => {
						if let Some(n) = current_num.clone() {
							part_numbers.push(PartNumber::new(x - n.len(), y, &n)?);
							current_num = None;
						}
					}
				}
			}
			if let Some(n) = current_num {
				part_numbers.push(PartNumber::new(row.len() - n.len(), y, &n)?);
			}
		}

		Ok(Self {
			symbols,
			part_numbers,
		})
	}
}

const TEST_1: &'static str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

submit!(Challenge {
	year: 2023,
	day: 03,
	part: 1,
	f: day03_1,
	unit_tests: &[(TEST_1, 4361)],
	skip: false,
});

pub fn day03_1(input: &str) -> ChallengeResult {
	let schematic = Schematic::parse(input)?;

	let res = schematic
		.part_numbers
		.iter()
		.filter(|part| {
			for check in part.adjacent_indices() {
				if schematic.symbols.contains_key(&check) {
					return true;
				}
			}
			return false;
		})
		.fold(0, |acc, part| acc + part.value);

	Ok(res)
}

submit!(Challenge {
	year: 2023,
	day: 03,
	part: 2,
	f: day03_2,
	unit_tests: &[(TEST_1, 467835)],
	skip: false,
});

pub fn day03_2(input: &str) -> ChallengeResult {
	let schematic = Schematic::parse(input)?;

	let gears = schematic
		.symbols
		.iter()
		.filter(|(_, c)| **c == '*')
		.map(|(pos, _)| *pos);

	let mut ratio_sum = 0;
	for gear_position in gears {
		let parts = schematic
			.part_numbers
			.iter()
			.filter(|part| part.adjacent_indices().contains(&gear_position))
			.collect::<Vec<&PartNumber>>();
		if parts.len() != 2 {
			continue;
		}
		ratio_sum += parts[0].value * parts[1].value;
	}

	Ok(ratio_sum)
}
