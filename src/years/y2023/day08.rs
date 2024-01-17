use std::collections::HashMap;

use super::*;

struct DesertMap {
	turns: Vec<Turn>,
	nodes: HashMap<NodeId, Node>,
}

impl DesertMap {
	fn parse(input: &str) -> Result<Self, anyhow::Error> {
		let err = || anyhow!("invalid input {}", input);
		let (turns, nodes) = input.split_once("\n\n").ok_or_else(err)?;
		let turns = turns
			.chars()
			.map(|c| match c {
				'L' => Turn::Left,
				'R' => Turn::Right,
				_ => unreachable!(),
			})
			.collect::<Vec<Turn>>();
		let nodes = nodes
			.lines()
			.map(|input| Node::parse(input).map(|node| (node.id, node)))
			.collect::<Result<HashMap<NodeId, Node>, anyhow::Error>>()?;
		Ok(DesertMap { turns, nodes })
	}

	fn goto(&self, from: &NodeId, dir: &Turn) -> NodeId {
		let node = self.nodes.get(from).unwrap();
		match dir {
			Turn::Left => node.left,
			Turn::Right => node.right,
		}
	}

	fn n_steps<F: Fn(&NodeId) -> bool>(
		&self,
		start_id: &NodeId,
		is_dest: F,
	) -> usize {
		let mut current_loc = *start_id;
		let mut steps = 0;
		for t in self.turns.iter().cycle() {
			steps += 1;
			current_loc = self.goto(&current_loc, t);
			if is_dest(&current_loc) {
				return steps;
			}
		}
		unreachable!();
	}
}

enum Turn {
	Left,
	Right,
}

struct Node {
	id: NodeId,
	left: NodeId,
	right: NodeId,
}

impl Node {
	fn parse(input: &str) -> Result<Self, anyhow::Error> {
		let err = || anyhow!("invalid node {}", input);
		let (id, lr) = input.split_once(" = ").ok_or_else(err)?;
		let (l, r) = lr
			.trim_matches(|c| c == ')' || c == '(')
			.split_once(", ")
			.ok_or_else(err)?;

		Ok(Node {
			id: NodeId::parse(id)?,
			left: NodeId::parse(l)?,
			right: NodeId::parse(r)?,
		})
	}
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
struct NodeId([char; 3]);

impl NodeId {
	fn parse(input: &str) -> Result<Self, anyhow::Error> {
		if input.len() != 3 {
			return Err(anyhow!("invalid node id {}", input));
		}
		Ok(Self(
			input.chars().collect::<Vec<char>>().try_into().unwrap(),
		))
	}
	fn is_starting_node(&self) -> bool {
		self.0[2] == 'A'
	}
	fn is_ending_node(&self) -> bool {
		self.0[2] == 'Z'
	}
}

static TEST_01_1: &'static str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

static TEST_01_2: &'static str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

fn day08_1(input: &str) -> ChallengeResult {
	let map = DesertMap::parse(input)?;
	let dest = NodeId::parse("ZZZ")?;
	let res = map.n_steps(&NodeId::parse("AAA")?, |n| n == &dest);
	Ok(res as u128)
}

submit!(Challenge {
	year: 2023,
	day: 08,
	part: 1,
	f: day08_1,
	unit_tests: &[(TEST_01_1, 2), (TEST_01_2, 6)],
	skip: false,
});

static TEST_02: &'static str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

fn day08_2(input: &str) -> ChallengeResult {
	use num_integer::Integer;

	let map = DesertMap::parse(input)?;

	let res = map
		.nodes
		.keys()
		.cloned()
		.filter(NodeId::is_starting_node)
		.map(|start| map.n_steps(&start, NodeId::is_ending_node))
		.fold(1, |acc, v| acc.lcm(&v));

	Ok(res as u128)
}

// wrong answer: 1124424219
submit!(Challenge {
	year: 2023,
	day: 08,
	part: 2,
	f: day08_2,
	unit_tests: &[(TEST_02, 6)],
	skip: false,
});
