use crate::common::ChallengeFn;

use super::*;

submit!(Challenge {
	year: 2015,
	day: 08,
	part: 1,
	f: day08_1,
	unit_tests: &[
		(r#""""#, 2 - 0),
		(r#""abc""#, 5 - 3),
		(r#""aaa\"aaa""#, 10 - 7),
		(r#""\x27""#, 6 - 1),
		(
			r#"""
"abc"
"aaa\"aaa"
"\x27""#,
			23 - 11
		)
	],
	skip: false,
});

submit!(Challenge {
	year: 2015,
	day: 08,
	part: 2,
	f: day08_2,
	unit_tests: &[
		(r#""""#, 6 - 2),
		(r#""abc""#, 9 - 5),
		(r#""aaa\"aaa""#, 16 - 10),
		(r#""\x27""#, 11 - 6),
		(
			r#"""
"abc"
"aaa\"aaa"
"\x27""#,
			42 - 23
		)
	],
	skip: false,
});

fn escaped_str_len(s: &str) -> usize {
	let mut l = 0;
	let mut i = 1;
	while i < (s.len() - 1) {
		let c = s.chars().nth(i).unwrap();
		if c == '\\' {
			let c2 = s.chars().nth(i + 1).unwrap();
			if c2 == 'x' {
				i += 3;
			} else {
				i += 1;
			}
		}
		i += 1;
		l += 1;
	}
	l
}

fn day08_1(input: &str) -> ChallengeResult {
	let (l1, l2) = input
		.lines()
		.map(|line| (line.len(), escaped_str_len(line)))
		.fold((0, 0), |(l1, l2), (a1, a2)| (l1 + a1, l2 + a2));

	Ok(l1 as u128 - l2 as u128)
}

fn encode_str(input: &str) -> String {
	let mut res = input.to_string();
	res = res.replace(r#"\"#, r#"\\"#);
	res = res.replace(r#"""#, r#"\""#);

	format!("\"{}\"", res)
}

fn day08_2(input: &str) -> ChallengeResult {
	let (l1, l2) = input
		.lines()
		.map(|line| (line.len(), encode_str(line).len()))
		.fold((0, 0), |(l1, l2), (a1, a2)| (l1 + a1, l2 + a2));

	Ok(l2 as u128 - l1 as u128)
}
