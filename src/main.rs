use std::time::Duration;

use clap::Parser;
use owo_colors::OwoColorize;
use rayon::prelude::*;

use crate::common::Challenge;

pub mod years {
	pub mod y2015;
	pub mod y2016;
	pub mod y2017;
	pub mod y2018;
	pub mod y2019;
	pub mod y2020;
	pub mod y2021;
	pub mod y2022;
	pub mod y2023;
}
pub mod common;
pub mod util;

fn main() -> Result<(), anyhow::Error> {
	let args = common::Args::parse();

	let challenges = common::all_challenges()?
		.into_iter()
		.filter(|c| {
			if let Some(year) = args.year {
				if year != c.year {
					return false;
				}
			}
			if let Some(day) = args.day {
				if day != c.day {
					return false;
				}
			}
			return true;
		})
		.collect::<Vec<&'static Challenge>>();

	if challenges.len() == 0 {
		println!("{}", "No challenges found!".yellow());
		return Ok(());
	}

	println!("\nRunning unit tests....\n");

	let test_results: Vec<common::TestResults> =
		challenges.par_iter().map(|item| item.unit_test()).collect();

	for result in test_results {
		println!("{}", result);
	}

	if args.test_only {
		println!("test-only specified, skipping actual tests.");
		return Ok(());
	}

	println!("\nRunning actual tests...\n");

	let real_results = challenges
		.into_par_iter()
		.map(|challenge| {
			challenge
				.run_on_file()
				.map(|(duration, result)| (challenge, duration, result))
		})
		.collect::<Result<
			Vec<(&Challenge, Duration, Result<u128, anyhow::Error>)>,
			anyhow::Error,
		>>()?;

	for (challenge, duration, result) in real_results {
		match result {
			Ok(res) => println!(
				"{} - {} {}",
				challenge.label().black(),
				res,
				format!("({}s)", duration.as_secs_f64()).black(),
			),
			Err(err) => println!(
				"{} - {} {}",
				challenge.label().black(),
				format!("Error - {}", err).red(),
				format!("({}s)", duration.as_secs_f64()).black(),
			),
		}
	}

	// years
	// 	.run_on_files()
	// 	.expect("Got an error while running on files");

	Ok(())
}
