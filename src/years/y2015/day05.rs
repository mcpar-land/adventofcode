use std::collections::HashSet;

use super::*;

fn total_nice(input: &str, is_nice: fn(&str) -> bool) -> ChallengeResult {
	let mut total_nice = 0;
	for line in input.lines() {
		if (is_nice)(line) {
			total_nice += 1;
		}
	}
	Ok(total_nice)
}

submit!(Challenge {
	year: 2015,
	day: 05,
	part: 1,
	f: day05_1,
	unit_tests: &[
		("ugknbfddgicrmopn", 1),
		("aaa", 1),
		("jchzalrnumimnmhp", 0),
		("haegwjzuvuyypxyu", 0),
		("dvszwmarrgswjxmb", 0),
	],
	skip: false,
});

fn is_nice_01(input: &str) -> bool {
	const VOWELS: &'static str = "aeiou";

	// check for 3 vowels
	let mut n_vowels = 0;
	for c in input.chars() {
		if VOWELS.contains(c) {
			n_vowels += 1;
		}
	}
	if n_vowels < 3 {
		return false;
	}

	// check for double letters
	let mut has_double_letter = false;
	let chars = input.chars().collect::<Vec<char>>();
	for w in chars.windows(2) {
		let (a, b) = (w[0], w[1]);
		if a == b {
			has_double_letter = true;
			break;
		}
	}
	if !has_double_letter {
		return false;
	}

	// check for naughty pairs
	let naughty_pairs = &[('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];
	for w in chars.windows(2) {
		let (a, b) = (w[0], w[1]);
		for (na, nb) in naughty_pairs {
			if a == *na && b == *nb {
				return false;
			}
		}
	}

	true
}

pub fn day05_1(input: &str) -> ChallengeResult {
	total_nice(input, is_nice_01)
}

submit!(Challenge {
	year: 2015,
	day: 05,
	part: 2,
	f: day05_2,
	unit_tests: &[
		("qjhvhtzxzqqjkmpb", 1),
		("xxyxx", 1),
		("uurcxstgmygtbstg", 0),
		("ieodomkazucvgmuy", 0),
	],
	skip: true
});

fn is_nice_02(input: &str) -> bool {
	let chars = input.chars().collect::<Vec<char>>();

	// check pairs
	let mut pairs = HashSet::<(char, char)>::new();
	let mut has_double = false;
	let mut i = 0;
	while i < chars.len() - 1 {
		let pair = (chars[i], chars[i + 1]);
		if !pairs.insert(pair) {
			has_double = true;
			break;
		}
		i += 1;
		if pair.0 == pair.1 && i < chars.len() - 2 && pair.1 == chars[i + 2] {
			i += 1;
		}
	}
	if !has_double {
		return false;
	}

	// check swandwich
	for w in chars.windows(3) {
		if w[0] == w[2] {
			return true;
		}
	}

	false
}

pub fn day05_2(input: &str) -> ChallengeResult {
	total_nice(input, is_nice_02)
}
