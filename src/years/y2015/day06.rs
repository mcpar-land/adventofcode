use std::ops::RangeInclusive;

use super::*;

trait Light {
	fn handle_command(&self, command: &CommandAction) -> Self;
	fn brightness(&self) -> i32;
}

#[derive(Clone, Copy, Default)]
struct ToggleLight(bool);

impl Light for ToggleLight {
	fn handle_command(&self, command: &CommandAction) -> Self {
		match command {
			CommandAction::TurnOn => Self(true),
			CommandAction::TurnOff => Self(false),
			CommandAction::Toggle => Self(!self.0),
		}
	}
	fn brightness(&self) -> i32 {
		if self.0 {
			1
		} else {
			0
		}
	}
}

#[derive(Clone, Copy, Default)]
struct FaderLight(i32);

impl Light for FaderLight {
	fn handle_command(&self, command: &CommandAction) -> Self {
		match command {
			CommandAction::TurnOn => Self(self.0 + 1),
			CommandAction::TurnOff => Self((self.0 - 1).max(0)),
			CommandAction::Toggle => Self(self.0 + 2),
		}
	}

	fn brightness(&self) -> i32 {
		self.0
	}
}

struct LightGrid<L: Light + Copy + Default>(Vec<Vec<L>>);

impl<L: Light + Copy + Default> LightGrid<L> {
	pub fn new() -> Self {
		Self(vec![vec![L::default(); 1000]; 1000])
	}

	pub fn change(&mut self, cmd: &Command) -> Result<(), anyhow::Error> {
		for y in cmd.from.1..=cmd.to.1 {
			let row = self
				.0
				.get_mut(y)
				.ok_or_else(|| anyhow!("invalid y coord {}", y))?;
			for x in cmd.from.0..=cmd.to.0 {
				let cell = row
					.get_mut(x)
					.ok_or_else(|| anyhow!("Invalid x coord {}", x))?;
				*cell = cell.handle_command(&cmd.action);
			}
		}

		Ok(())
	}

	pub fn brightness(&self) -> i32 {
		let mut count = 0;
		for row in &self.0 {
			for cell in row {
				count += cell.brightness();
			}
		}
		count
	}
}

fn parse_range(range: &str) -> Result<(usize, usize), anyhow::Error> {
	let (x, y) = range
		.split_once(",")
		.ok_or_else(|| anyhow!("Invalid range, no comma"))?;
	let x: usize = x
		.parse()
		.map_err(|e| anyhow!("Error parsing {}: {}", x, e))?;
	let y: usize = y
		.parse()
		.map_err(|e| anyhow!("Error parsing {}: {}", y, e))?;
	Ok((x, y))
}

#[derive(Debug)]
struct Command {
	pub action: CommandAction,
	pub from: (usize, usize),
	pub to: (usize, usize),
}

impl Command {
	pub fn parse(line: &str) -> Result<Self, anyhow::Error> {
		let mut s = line.to_string();
		let options = [
			("turn on", CommandAction::TurnOn),
			("turn off", CommandAction::TurnOff),
			("toggle", CommandAction::Toggle),
		];
		let mut action: Option<CommandAction> = None;
		for (prefix, a) in options {
			if s.starts_with(prefix) {
				action = Some(a);
				s = s.trim_start_matches(prefix).to_string();
				break;
			}
			s = s.trim_start_matches(prefix).to_string();
		}
		let action =
			action.ok_or_else(|| anyhow!("invalid prefix on command {}", line))?;
		s = s.trim_start().to_string();

		let (r1, r2) = s
			.split_once(" through ")
			.ok_or_else(|| anyhow!("invalid command {}", line))?;

		Ok(Command {
			action,
			from: parse_range(r1)?,
			to: parse_range(r2)?,
		})
	}
}

#[derive(Debug)]
enum CommandAction {
	TurnOn,
	TurnOff,
	Toggle,
}

submit!(Challenge {
	year: 2015,
	day: 06,
	part: 1,
	f: day06_1,
	unit_tests: &[
		("turn on 0,0 through 999,999", 1000 * 1000),
		("toggle 0,0 through 999,0", 1000),
		("turn off 499,499 through 500,500", 0),
	],
	skip: false,
});

fn day06_1(input: &str) -> ChallengeResult {
	let commands = input
		.lines()
		.map(|line| Command::parse(line))
		.collect::<Result<Vec<Command>, anyhow::Error>>()?;
	// println!("{:#?}", commands);

	let mut grid = LightGrid::<ToggleLight>::new();

	for command in &commands {
		grid.change(command)?;
	}

	Ok(grid.brightness() as u128)
}

submit!(Challenge {
	year: 2015,
	day: 06,
	part: 2,
	f: day06_2,
	unit_tests: &[
		("turn on 0,0 through 0,0", 1),
		("toggle 0,0 through 999,999", 2000000),
	],
	skip: false,
});

fn day06_2(input: &str) -> ChallengeResult {
	let commands = input
		.lines()
		.map(|line| Command::parse(line))
		.collect::<Result<Vec<Command>, anyhow::Error>>()?;
	// println!("{:#?}", commands);

	let mut grid = LightGrid::<FaderLight>::new();

	for command in &commands {
		grid.change(command)?;
	}

	Ok(grid.brightness() as u128)
}
