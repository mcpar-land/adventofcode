use std::collections::HashSet;

use super::*;

pub enum Dir {
	Up,
	Down,
	Left,
	Right,
}

impl Dir {
	pub fn shift(&self, x: i32, y: i32) -> (i32, i32) {
		match self {
			Dir::Up => (x, y + 1),
			Dir::Down => (x, y - 1),
			Dir::Left => (x - 1, y),
			Dir::Right => (x + 1, y),
		}
	}
}

fn parse(input: &str) -> Result<Vec<Dir>, anyhow::Error> {
	input
		.chars()
		.map(|c| match c {
			'^' => Ok(Dir::Up),
			'v' => Ok(Dir::Down),
			'<' => Ok(Dir::Left),
			'>' => Ok(Dir::Right),
			c => Err(anyhow::anyhow!("bad input {}", c)),
		})
		.collect()
}

submit!(Challenge {
	year: 2015,
	day: 03,
	part: 1,
	f: day03_1,
	unit_tests: &[(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)],
	skip: false
});

pub fn day03_1(input: &str) -> ChallengeResult {
	let mut houses = HashSet::<(i32, i32)>::new();
	houses.insert((0, 0));
	let (mut x, mut y) = (0, 0);
	for dir in parse(input)? {
		(x, y) = dir.shift(x, y);
		houses.insert((x, y));
	}
	Ok(houses.len() as u128)
}

submit!(Challenge {
	year: 2015,
	day: 03,
	part: 2,
	f: day03_2,
	unit_tests: &[("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)],
	skip: false
});

pub fn day03_2(input: &str) -> ChallengeResult {
	let mut houses = HashSet::<(i32, i32)>::new();
	houses.insert((0, 0));
	let (mut x1, mut y1) = (0, 0);
	let (mut x2, mut y2) = (0, 0);
	for (i, dir) in parse(input)?.into_iter().enumerate() {
		let (x, y) = if (i + 1) % 2 == 0 {
			(&mut x1, &mut y1)
		} else {
			(&mut x2, &mut y2)
		};
		(*x, *y) = dir.shift(*x, *y);
		houses.insert((*x, *y));
	}
	Ok(houses.len() as u128)
}
