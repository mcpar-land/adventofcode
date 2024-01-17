use super::*;

struct Game {
	id: u128,
	turns: Vec<Turn>,
}

impl Game {
	pub fn parse(line: &str) -> Result<Self, anyhow::Error> {
		let (id, turns) = line
			.split_once(": ")
			.ok_or(anyhow!("malformed line: {}", line))?;
		let id: u128 = id
			.strip_prefix("Game ")
			.ok_or(anyhow!("malformed line: {}", line))?
			.parse()?;

		let turns = turns
			.split("; ")
			.map(Turn::parse)
			.collect::<Result<Vec<Turn>, anyhow::Error>>()?;

		Ok(Game { id, turns })
	}

	pub fn is_possible(&self) -> bool {
		for turn in &self.turns {
			if !turn.is_possible() {
				return false;
			}
		}
		true
	}

	pub fn min_cubes(&self) -> Turn {
		let mut min_cubes = Turn {
			red: 0,
			green: 0,
			blue: 0,
		};

		for turn in &self.turns {
			min_cubes.red = min_cubes.red.max(turn.red);
			min_cubes.green = min_cubes.green.max(turn.green);
			min_cubes.blue = min_cubes.blue.max(turn.blue);
		}
		min_cubes
	}
}

struct Turn {
	pub red: u128,
	pub green: u128,
	pub blue: u128,
}

impl Turn {
	pub fn parse(input: &str) -> Result<Self, anyhow::Error> {
		let mut r: Option<u128> = None;
		let mut g: Option<u128> = None;
		let mut b: Option<u128> = None;

		for pull in input.split(", ") {
			let (number, color) = pull
				.split_once(' ')
				.ok_or_else(|| anyhow!("malformed turn: {}", input))?;
			let number: u128 = number.parse()?;
			*match color {
				"red" => &mut r,
				"green" => &mut g,
				"blue" => &mut b,
				_ => return Err(anyhow!("malformed turn: {}", input)),
			} = Some(number);
		}

		Ok(Turn {
			red: r.unwrap_or_default(),
			green: g.unwrap_or_default(),
			blue: b.unwrap_or_default(),
		})
	}

	fn is_possible(&self) -> bool {
		self.red <= 12 && self.green <= 13 && self.blue <= 14
	}

	fn power(&self) -> u128 {
		self.red * self.green * self.blue
	}
}

const TEST_ALL: &'static str =
	"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

const GAME_1: &'static str =
	"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
const GAME_2: &'static str =
	"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
const GAME_3: &'static str =
	"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
const GAME_4: &'static str =
	"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
const GAME_5: &'static str =
	"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

submit!(Challenge {
	year: 2023,
	day: 02,
	part: 1,
	f: day02_1,
	unit_tests: &[
		(GAME_1, 1),
		(GAME_2, 2),
		(GAME_3, 0),
		(GAME_4, 0),
		(GAME_5, 5),
		(TEST_ALL, 8)
	],
	skip: false,
});

fn day02_1(input: &str) -> ChallengeResult {
	let mut id_sum = 0;

	for line in input.lines() {
		let game = Game::parse(line)?;
		if game.is_possible() {
			id_sum += game.id;
		}
	}

	Ok(id_sum)
}

submit!(Challenge {
	year: 2023,
	day: 02,
	part: 2,
	f: day02_2,
	unit_tests: &[
		(GAME_1, 48),
		(GAME_2, 12),
		(GAME_3, 1560),
		(GAME_4, 630),
		(GAME_5, 36),
		(TEST_ALL, 2286)
	],
	skip: false,
});

fn day02_2(input: &str) -> ChallengeResult {
	let mut power_sum = 0;
	for line in input.lines() {
		let game = Game::parse(line)?;
		power_sum += game.min_cubes().power();
	}
	Ok(power_sum)
}
