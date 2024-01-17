use anyhow::anyhow;
use clap::Parser;
use core::cmp::Ordering;
use owo_colors::OwoColorize;
use std::{
	collections::HashSet,
	fmt::Display,
	time::{Duration, Instant},
};

#[derive(Parser, Debug)]
#[command()]
pub struct Args {
	#[arg(short, long)]
	pub test_only: bool,
	#[arg(short, long)]
	pub year: Option<usize>,
	#[arg(short, long)]
	pub day: Option<usize>,
}

pub fn all_challenges() -> Result<Vec<&'static Challenge>, anyhow::Error> {
	let mut set = HashSet::new();

	for challenge in inventory::iter::<Challenge> {
		let challenge: &Challenge = challenge;
		if !set.insert(challenge) {
			return Err(anyhow!("Duplicate definition of: {}", challenge.label()));
		}
	}

	let mut res = set.into_iter().collect::<Vec<&Challenge>>();
	res.sort();

	Ok(res)
}

#[derive(Hash)]
pub struct Challenge {
	pub year: usize,
	pub day: usize,
	pub part: usize,
	pub f: ChallengeFn,
	pub unit_tests: &'static [(&'static str, u128)],
	pub skip: bool,
}

inventory::collect!(Challenge);

impl Challenge {
	pub fn label(&self) -> String {
		format!(
			"{} :: Day {:0>2} :: Part {}",
			self.year, self.day, self.part
		)
	}

	pub fn unit_test(&'static self) -> TestResults {
		TestResults {
			challenge: &self,
			results: self
				.unit_tests
				.iter()
				.map(|(input, expected)| match (self.f)(input) {
					Ok(got) => {
						if *expected == got {
							UnitTestResult::Equal
						} else {
							UnitTestResult::NotEqual {
								input: input.to_string(),
								expected: *expected,
								got,
							}
						}
					}
					Err(err) => UnitTestResult::Other(err),
				})
				.collect::<Vec<UnitTestResult>>(),
		}
	}

	pub fn run_on_file(
		&self,
	) -> Result<(Duration, Result<u128, anyhow::Error>), anyhow::Error> {
		let path = format!("./inputs/{}/{:0>2}.txt", self.year, self.day);
		let input = std::fs::read_to_string(path)?;

		let start = Instant::now();
		let result = (self.f)(&input);
		let duration = start.elapsed();

		Ok((duration, result))
	}
}

impl PartialEq for Challenge {
	fn eq(&self, other: &Self) -> bool {
		self.year == other.year && self.day == other.day && self.part == other.part
	}
}

impl Eq for Challenge {}

impl PartialOrd for Challenge {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		match self.year.partial_cmp(&other.year) {
			Some(Ordering::Equal) => {}
			ord => return ord,
		}
		match self.day.partial_cmp(&other.day) {
			Some(Ordering::Equal) => {}
			ord => return ord,
		}
		match self.part.partial_cmp(&other.part) {
			Some(Ordering::Equal) => {}
			ord => return ord,
		}
		Some(Ordering::Equal)
	}
}

impl Ord for Challenge {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

pub struct TestResults {
	challenge: &'static Challenge,
	results: Vec<UnitTestResult>,
}

impl Display for TestResults {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} - ", self.challenge.label().black())?;

		if self.challenge.skip {
			return write!(f, "{}", "(skipped)".black());
		}

		let n_passed = self.results.iter().filter(|r| r.equal()).count();
		let all_passed = n_passed == self.results.len();

		let mut counts = format!("{}/{}", n_passed, self.results.len());
		if all_passed {
			counts.push_str(" ✔️");
			counts = counts.green().to_string();
		} else {
			counts.push_str(" ❌");
			counts = counts.red().to_string();
		};

		write!(f, "{}", counts)?;

		if !all_passed {
			for (i, res) in
				self.results.iter().enumerate().filter(|(_, r)| !r.equal())
			{
				let output = match res {
					UnitTestResult::Equal => unreachable!(),
					UnitTestResult::NotEqual {
						input,
						expected,
						got,
					} => {
						format!(
							"for input: \n\n{}\n\n---expected {}, got {}",
							input, expected, got
						)
					}
					UnitTestResult::Other(err) => {
						format!("Error: {}", err)
					}
				};
				let red = format!(" Test {} ", i);
				write!(f, "\n  {} {}", red.on_red().black(), output.red())?;
			}
		}

		write!(f, "")
	}
}

#[derive(Debug)]
pub enum UnitTestResult {
	Equal,
	NotEqual {
		input: String,
		expected: u128,
		got: u128,
	},
	Other(anyhow::Error),
}

impl UnitTestResult {
	pub fn equal(&self) -> bool {
		match self {
			Self::Equal => true,
			_ => false,
		}
	}
}

pub type ChallengeFn = fn(&str) -> ChallengeResult;
pub type ChallengeResult = Result<u128, anyhow::Error>;

fn idt(level: usize) -> String {
	let mut s = String::new();
	for _ in 0..level {
		s += "  ";
	}
	s
}
