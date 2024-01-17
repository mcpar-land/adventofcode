use super::*;

fn parse_day02(input: &str) -> Vec<(u128, u128, u128)> {
	input
		.lines()
		.map(|line| {
			let mut split = line.split('x');
			let l: u128 = split.next().unwrap().parse().unwrap();
			let w: u128 = split.next().unwrap().parse().unwrap();
			let h: u128 = split.next().unwrap().parse().unwrap();
			(l, w, h)
		})
		.collect()
}

submit!(Challenge {
	year: 2015,
	day: 02,
	part: 1,
	f: day02_1,
	unit_tests: &[("2x3x4", 58), ("1x1x10", 43)],
	skip: false
});

fn day02_1(input: &str) -> ChallengeResult {
	let mut total = 0;
	for (l, w, h) in parse_day02(input) {
		total += (2 * l * w) + (2 * w * h) + (2 * h * l);
		total += [l * w, w * h, h * l].into_iter().min().unwrap();
	}

	Ok(total)
}

submit!(Challenge {
	year: 2015,
	day: 02,
	part: 2,
	f: day02_2,
	unit_tests: &[("2x3x4", 34), ("1x1x10", 14)],
	skip: false
});

fn day02_2(input: &str) -> ChallengeResult {
	let mut total = 0;
	for (l, w, h) in parse_day02(input) {
		total += [l + w, w + h, h + l]
			.into_iter()
			.map(|v| 2 * v)
			.min()
			.unwrap();
		total += l * w * h;
	}
	Ok(total)
}
