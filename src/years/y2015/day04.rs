use super::*;
use md5::compute;

pub fn find_prefix(input: &str, prefix: &str) -> ChallengeResult {
	const MAX_LOOPS: u128 = 9999999u128;
	for i in 0..MAX_LOOPS {
		let code = format!("{}{}", input, i);
		let res = format!("{:x}", compute(code));
		if res.starts_with(prefix) {
			return Ok(i);
		}
	}
	Err(anyhow!("Exceeded max hash loops of {}", MAX_LOOPS))
}

submit!(Challenge {
	year: 2015,
	day: 04,
	part: 1,
	f: day04_1,
	unit_tests: &[("abcdef", 609043), ("pqrstuv", 1048970)],
	skip: false
});

pub fn day04_1(input: &str) -> ChallengeResult {
	find_prefix(input, "00000")
}

submit!(Challenge {
	year: 2015,
	day: 04,
	part: 2,
	f: day04_2,
	unit_tests: &[],
	skip: false
});

pub fn day04_2(input: &str) -> ChallengeResult {
	find_prefix(input, "000000")
}
