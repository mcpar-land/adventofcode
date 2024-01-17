use super::*;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug)]
struct SensorValues(Vec<History>);

impl SensorValues {
	fn parse(input: &str) -> Result<Self, anyhow::Error> {
		Ok(Self(
			input
				.lines()
				.map(History::parse)
				.collect::<Result<_, _>>()?,
		))
	}
	fn next_value_sums(&self) -> i32 {
		self.0.par_iter().map(History::predict_next).sum()
	}
	fn previous_value_sums(&self) -> i32 {
		self.0.par_iter().map(History::predict_previous).sum()
	}
}

#[derive(Debug)]
struct History(Vec<i32>);

impl History {
	fn parse(line: &str) -> Result<Self, anyhow::Error> {
		Ok(Self(
			line
				.split_whitespace()
				.map(|v| v.parse::<i32>())
				.collect::<Result<Vec<i32>, _>>()?,
		))
	}

	fn predict_next(&self) -> i32 {
		if self.0.iter().all(|v| *v == 0) {
			return 0;
		}
		let current_value = *self.0.last().unwrap();

		let next_list = self
			.0
			.iter()
			.tuple_windows()
			.map(|(a, b)| b - a)
			.collect::<Vec<i32>>();
		let next_value = Self(next_list).predict_next();

		current_value + next_value
	}

	fn predict_previous(&self) -> i32 {
		if self.0.iter().all(|v| *v == 0) {
			return 0;
		}
		let current_value = self.0[0];

		let next_list = self
			.0
			.iter()
			.tuple_windows()
			.map(|(a, b)| a - b) // this is reversed!
			.collect::<Vec<i32>>();
		let next_value = Self(next_list).predict_previous();

		current_value + next_value
	}
}

static TEST_1: &'static str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

pub fn day09_1(input: &str) -> ChallengeResult {
	let values = SensorValues::parse(input)?;

	Ok(values.next_value_sums() as u128)
}

submit!(Challenge {
	year: 2023,
	day: 09,
	part: 1,
	f: day09_1,
	unit_tests: &[
		("0 3 6 9 12 15", 18),
		("1 3 6 10 15 21", 28),
		("10 13 16 21 30 45", 68),
		(TEST_1, 114)
	],
	skip: false
});

pub fn day09_2(input: &str) -> ChallengeResult {
	let values = SensorValues::parse(input)?;

	Ok(values.previous_value_sums() as u128)
}

submit!(Challenge {
	year: 2023,
	day: 09,
	part: 2,
	f: day09_2,
	unit_tests: &[(TEST_1, 2)],
	skip: false
});
