use super::*;
use once_cell::sync::Lazy;
use regex::Regex;

submit!(Challenge {
	year: 2023,
	day: 01,
	part: 1,
	f: day01_1,
	unit_tests: &[
		("1abc2", 12),
		("pqr3stu8vwx", 38),
		("a1b2c3d4e5f", 15),
		("treb7uchet", 77),
		("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet", 142),
	],
	skip: false,
});

fn is_digit(c: &char) -> bool {
	"1234567890".contains(*c)
}

fn parse_w_digits(line: &str) -> ChallengeResult {
	let first = line.chars().find(is_digit);
	let last = line.chars().rev().find(is_digit);

	let (first, last) = match (first, last) {
		(Some(first), Some(last)) => (first, last),
		_ => return Err(anyhow!("{} has no digits", line)),
	};

	let digit: u128 = format!("{}{}", first, last).parse()?;
	Ok(digit)
}

fn day01_1(input: &str) -> ChallengeResult {
	let mut sum = 0;
	for line in input.lines() {
		sum += parse_w_digits(line)?;
	}
	Ok(sum)
}

submit!(Challenge {
	year: 2023,
	day: 01,
	part: 2,
	f: day01_2,
	unit_tests: &[
		("two1nine", 29),
		("eightwothree", 83),
		("abcone2threexyz", 13),
		("xtwone3four", 24),
		("4nineeightseven2", 42),
		("zoneight234", 14),
		("7pqrstsixteen", 76),
	],
	skip: false,
});

fn spelled_to_char(spelled: &str) -> Result<char, anyhow::Error> {
	let res = match spelled {
		"one" => '1',
		"two" => '2',
		"three" => '3',
		"four" => '4',
		"five" => '5',
		"six" => '6',
		"seven" => '7',
		"eight" => '8',
		"nine" => '9',
		"1" => '1',
		"2" => '2',
		"3" => '3',
		"4" => '4',
		"5" => '5',
		"6" => '6',
		"7" => '7',
		"8" => '8',
		"9" => '9',
		"0" => '0',
		_ => return Err(anyhow!("{} not a valid number", spelled)),
	};
	Ok(res)
}

fn find_first_spelled(line: &str, end: bool) -> Result<char, anyhow::Error> {
	let re = Lazy::new(|| {
		Regex::new(if end {
			r".*(one|two|three|four|five|six|seven|eight|nine|\d)"
		} else {
			r"(one|two|three|four|five|six|seven|eight|nine|\d)"
		})
		.unwrap()
	});
	let first = re
		.captures(line)
		.ok_or(anyhow!("No number found"))?
		.get(1)
		.ok_or(anyhow!("No number found"))?;
	spelled_to_char(first.as_str())
}

fn parse_w_spelled(line: &str) -> ChallengeResult {
	let first = find_first_spelled(line, false)?;
	let last = find_first_spelled(line, true)?;
	let digit: u128 = format!("{}{}", first, last).parse()?;
	Ok(digit)
}

fn day01_2(input: &str) -> ChallengeResult {
	let mut sum = 0;
	for line in input.lines() {
		sum += parse_w_spelled(line)?;
	}
	Ok(sum)
}
