use super::*;

submit!(Challenge {
	year: 2015,
	day: 01,
	part: 1,
	f: day01_1,
	unit_tests: &[
		("(())", 0),
		("()()", 0),
		("(((", 3),
		("(()(()(", 3),
		("))(((((", 3),
		// ("())", -1),
		// ("))(", -1),
		// (")))", -3),
		// (")())())", -3),
	],
	skip: false,
});

fn day01_1(input: &str) -> ChallengeResult {
	let mut floor: i32 = 0;
	for c in input.chars() {
		match c {
			'(' => {
				floor += 1;
			}
			')' => {
				floor -= 1;
			}
			c => {
				return Err(anyhow!("unrecognized command {}", c));
			}
		}
	}
	Ok(floor as u128)
}

submit!(Challenge {
	year: 2015,
	day: 01,
	part: 2,
	f: day01_2,
	unit_tests: &[(")", 1), ("()())", 5)],
	skip: false,
});

fn day01_2(input: &str) -> ChallengeResult {
	let mut floor = 0;
	for (i, c) in input.chars().enumerate() {
		match c {
			'(' => {
				floor += 1;
			}
			')' => {
				floor -= 1;
			}
			c => {
				return Err(anyhow!("unrecognized command {}", c));
			}
		}
		if floor == -1 {
			return Ok(i as u128 + 1);
		}
	}
	Err(anyhow!("Never got to floor -1"))
}
