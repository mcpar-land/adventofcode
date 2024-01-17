use std::collections::HashMap;

use super::*;
use anyhow::Error;
use owo_colors::*;

#[derive(Debug)]
struct CircuitState(pub HashMap<String, WireState>);

impl CircuitState {
	pub fn get(&mut self, id: &str) -> Result<u16, Error> {
		if let Some(mut wire) = self.0.get(id).cloned() {
			if let Some(cached_value) = wire.cached_value {
				Ok(cached_value)
			} else {
				let res = wire.gate.run(self)?;
				wire.cached_value = Some(res);
				// println!("Cached wire {}: {}", id, res);
				self.0.insert(id.to_string(), wire);
				Ok(res)
			}
		} else {
			Err(anyhow!("wire {} not found", id))
		}
	}
}

#[derive(Debug, Clone)]
struct WireState {
	pub gate: Gate,
	pub cached_value: Option<u16>,
}

#[derive(Debug)]
struct Command {
	pub gate: Gate,
	pub assignment: String,
}

impl Command {
	pub fn parse(s: &str) -> Result<Self, Error> {
		let (gate, assignment) = s
			.split_once(" -> ")
			.ok_or_else(|| anyhow!("Missing -> in command \"{}\"", s))?;
		Ok(Command {
			gate: Gate::parse(gate)?,
			assignment: assignment.to_string(),
		})
	}
	pub fn apply(&self, state: &mut CircuitState) -> Result<(), Error> {
		if state.0.contains_key(&self.assignment) {
			return Err(anyhow!(
				"input already assigned for wire {}",
				self.assignment
			));
		}
		state.0.insert(
			self.assignment.clone(),
			WireState {
				gate: self.gate.clone(),
				cached_value: None,
			},
		);
		Ok(())
	}
}

#[derive(Debug, Clone)]
enum Gate {
	Assignment(Value),
	And(Value, Value),
	Or(Value, Value),
	LShift(Value, Value),
	RShift(Value, Value),
	Not(Value),
}

impl Gate {
	pub fn parse(s: &str) -> Result<Self, Error> {
		let words = s.split_ascii_whitespace().collect::<Vec<&str>>();
		if words.len() == 1 {
			return Ok(Self::Assignment(Value::parse(words[0])?));
		}
		if words.len() == 2 && words[0] == "NOT" {
			return Ok(Self::Not(Value::parse(words[1])?));
		}
		if words.len() == 3 {
			let a = Value::parse(words[0])?;
			let b = Value::parse(words[2])?;
			return match words[1] {
				"AND" => Ok(Self::And(a, b)),
				"OR" => Ok(Self::Or(a, b)),
				"LSHIFT" => Ok(Self::LShift(a, b)),
				"RSHIFT" => Ok(Self::RShift(a, b)),
				e => Err(anyhow!("Invalid gate {}", e)),
			};
		}
		return Err(anyhow!("Invalid gate: \"{}\"", s));
	}
	pub fn run(&self, state: &mut CircuitState) -> Result<u16, Error> {
		Ok(match self {
			Gate::Assignment(a) => a.get(state)?,
			Gate::And(a, b) => a.get(state)? & b.get(state)?,
			Gate::Or(a, b) => a.get(state)? | b.get(state)?,
			Gate::LShift(a, b) => a.get(state)? >> b.get(state)?,
			Gate::RShift(a, b) => a.get(state)? << b.get(state)?,
			Gate::Not(a) => !a.get(state)?,
		})
	}
}

#[derive(Debug, Clone)]
enum Value {
	Literal(u16),
	Variable(String),
}

impl Value {
	pub fn parse(s: &str) -> Result<Self, Error> {
		if s.contains(' ') {
			return Err(anyhow!("invalid value \"{}\"", s));
		}
		if let Ok(lit) = s.parse() {
			Ok(Self::Literal(lit))
		} else {
			Ok(Self::Variable(s.to_string()))
		}
	}
	pub fn get(&self, state: &mut CircuitState) -> Result<u16, Error> {
		match self {
			Value::Literal(lit) => Ok(*lit),
			Value::Variable(var_name) => {
				// println!("Running {}", var_name);
				state.get(var_name)
			}
		}
	}
}

submit!(Challenge {
	year: 2015,
	day: 07,
	part: 1,
	f: day07_1,
	unit_tests: &[(
		"123 -> ll\nll -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> a",
		65079,
	)],
	skip: true,
});

fn day07_1(input: &str) -> ChallengeResult {
	let commands = input
		.lines()
		.map(|line| Command::parse(line))
		.collect::<Result<Vec<Command>, Error>>()?;

	let mut state = CircuitState(HashMap::new());

	for command in commands {
		command.apply(&mut state)?;
	}

	let res = state.get("a").map(|v| v as u128);
	// println!("{:#?}", state);
	res
}
