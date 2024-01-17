use super::*;

const TEST_01: &'static str = r#"Time:      7  15   30
Distance:  9  40  200"#;

struct RaceList {
	pub races: Vec<Race>,
}

impl RaceList {
	pub fn parse(input: &str) -> Self {
		let (line_time, line_distance) = input.split_once("\n").unwrap();

		let iter_time = line_time
			.strip_prefix("Time:")
			.unwrap()
			.trim()
			.split_ascii_whitespace()
			.map(|v| {
				let time: u64 = v.parse().unwrap();
				time
			});

		let iter_distance = line_distance
			.strip_prefix("Distance:")
			.unwrap()
			.trim()
			.split_ascii_whitespace()
			.map(|v| {
				let time: u64 = v.parse().unwrap();
				time
			});

		Self {
			races: iter_time
				.zip(iter_distance)
				.map(|(time, distance)| Race { time, distance })
				.collect(),
		}
	}

	pub fn parse_2(input: &str) -> Self {
		let (line_time, line_distance) = input.split_once("\n").unwrap();

		let time: u64 = line_time
			.strip_prefix("Time:")
			.unwrap()
			.to_string()
			.replace(" ", "")
			.parse()
			.unwrap();

		let distance: u64 = line_distance
			.strip_prefix("Distance:")
			.unwrap()
			.to_string()
			.replace(" ", "")
			.parse()
			.unwrap();

		Self {
			races: vec![Race { time, distance }],
		}
	}

	pub fn ways_to_win_multiplied(&self) -> u64 {
		use rayon::prelude::*;

		self
			.races
			.par_iter()
			.fold(|| 1, |acc, race| acc * race.ways_to_win())
			.reduce(|| 1, |acc, v| acc * v)
	}
}

struct Race {
	pub time: u64,
	pub distance: u64,
}

impl Race {
	pub fn ways_to_win(&self) -> u64 {
		(0..self.time)
			.into_iter()
			.filter(|hold_time| (hold_time * (self.time - hold_time)) > self.distance)
			.count() as u64
	}
}

submit!(Challenge {
	year: 2023,
	day: 06,
	part: 1,
	f: day06_1,
	unit_tests: &[
		("Time: 7\nDistance: 9", 4),
		("Time: 15\nDistance: 40", 8),
		("Time: 30\nDistance: 200", 9),
		(TEST_01, 288)
	],
	skip: false
});

fn day06_1(input: &str) -> ChallengeResult {
	let race_list = RaceList::parse(input);
	Ok(race_list.ways_to_win_multiplied() as u128)
}

submit!(Challenge {
	year: 2023,
	day: 06,
	part: 2,
	f: day06_2,
	unit_tests: &[(TEST_01, 71503)],
	skip: false
});

fn day06_2(input: &str) -> ChallengeResult {
	let race_list = RaceList::parse_2(input);
	Ok(race_list.ways_to_win_multiplied() as u128)
}
