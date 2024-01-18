use std::collections::HashSet;

use crate::common::Pos;

use super::*;

struct Map {
	tiles: Vec<Vec<Tile>>,
	start_pos: Pos,
	size: Pos,
}

impl Map {
	pub fn parse(input: &str) -> Self {
		let mut start_pos = None;
		let mut width = 0;
		let mut height = 0;
		let tiles: Vec<Vec<Tile>> = input
			.lines()
			.enumerate()
			.map(|(y, line)| {
				height = y + 1;
				line
					.chars()
					.enumerate()
					.map(|(x, c)| {
						width = x + 1;
						let tile = Tile::parse(c);
						if tile == Tile::StartingPosition {
							start_pos = Some(Pos::new(x as i32, y as i32 * -1))
						}
						tile
					})
					.collect()
			})
			.collect();
		Map {
			start_pos: start_pos.unwrap(),
			tiles,
			size: Pos::new(width as i32, height as i32),
		}
	}
	fn get(&self, pos: Pos) -> Option<&Tile> {
		self
			.tiles
			.get((pos.y * -1) as usize)
			.and_then(|row| row.get(pos.x as usize))
	}

	fn get_loop(&self) -> HashSet<Pos> {
		Direction::all()
			.into_iter()
			.map(|dir| (dir, dir.offset()))
			.find_map(|(dir, pos)| {
				self._get_loop(HashSet::new(), (pos * -1) + self.start_pos, dir)
			})
			.unwrap()
	}

	fn _get_loop(
		&self,
		mut vals: HashSet<Pos>,
		pos: Pos,
		from: Direction,
	) -> Option<HashSet<Pos>> {
		let tile = self.get(pos).cloned()?;
		if tile == Tile::StartingPosition {
			vals.insert(pos);
			return Some(vals);
		}
		let next = tile.travel(&from)?;
		let offset = next.offset();
		vals.insert(pos);
		self._get_loop(vals, pos + offset, next.inverse())
	}

	fn n_enclosed(&self) -> usize {
		// first, we have to find a square that isn't part of the pipe or enclosed
		// by the pipe, to use as our starting point. to do this, we combine a
		// bunch of iterators to look at all the tiles on the very edge of the
		// map.
		let start = (0..self.size.x)
			.map(|x| Pos::new(x, 0))
			.chain((0..self.size.x).map(|x| Pos::new(x, self.size.y - 1)))
			.chain((0..self.size.y).map(|y| Pos::new(0, y)))
			.chain((0..self.size.y).map(|y| Pos::new(self.size.x - 1, y)))
			.find_map(|pos| match self.get(pos).unwrap() {
				Tile::Ground => Some(pos),
				_ => None,
			})
			.unwrap();

		// diagonals
		let diagonals = [
			Pos::new(1, 1),
			Pos::new(-1, 1),
			Pos::new(1, -1),
			Pos::new(-1, -1),
		];
		for start in diagonals {
			if let Some(contigs) = self.contiguous(self.start_pos + start) {
				let loop_vals = self.get_loop();
				return (self.size.x * self.size.y) as usize
					- loop_vals.len()
					- contigs.len();
			}
		}

		panic!("Couldn't find enclosure!");
	}

	fn contiguous(&self, start: Pos) -> Option<HashSet<Pos>> {
		self._contiguous(HashSet::new(), start)
	}

	fn _contiguous(
		&self,
		mut visited: HashSet<Pos>,
		pos: Pos,
	) -> Option<HashSet<Pos>> {
		for dir in Direction::all() {
			let new_pos = pos + dir.offset();
			if visited.contains(&new_pos) {
				continue;
			}
			// if we reach a border, exit instantly, we are not contained
			match self.get(new_pos)? {
				Tile::Ground => {
					visited.insert(pos);
					visited = self._contiguous(visited, new_pos)?;
				}
				_ => {}
			}
		}
		Some(visited)
	}
}

#[derive(PartialEq, Eq, Clone)]
enum Tile {
	Pipe(Direction, Direction),
	Ground,
	StartingPosition,
}

impl Tile {
	fn parse(c: char) -> Self {
		use Direction::*;
		use Tile::*;
		match c {
			'|' => Pipe(North, South),
			'-' => Pipe(East, West),
			'L' => Pipe(North, East),
			'J' => Pipe(North, West),
			'7' => Pipe(South, West),
			'F' => Pipe(South, East),
			'.' => Ground,
			'S' => StartingPosition,
			_ => panic!("unrecognized tile {}", c),
		}
	}
	fn travel(&self, from: &Direction) -> Option<Direction> {
		match self {
			Tile::Pipe(a, b) => {
				if from == a {
					return Some(*b);
				}
				if from == b {
					return Some(*a);
				}
				// cannot approach this pipe, not connected.
				None
			}
			Tile::Ground => None,
			Tile::StartingPosition => None,
		}
	}
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
	North,
	South,
	East,
	West,
}

impl Direction {
	fn offset(self) -> Pos {
		match self {
			Direction::North => Pos::new(0, 1),
			Direction::South => Pos::new(0, -1),
			Direction::East => Pos::new(1, 0),
			Direction::West => Pos::new(-1, 0),
		}
	}
	fn inverse(self) -> Self {
		match self {
			Direction::North => Self::South,
			Direction::South => Self::North,
			Direction::East => Self::West,
			Direction::West => Self::East,
		}
	}
	fn all() -> [Self; 4] {
		[
			Direction::North,
			Direction::South,
			Direction::East,
			Direction::West,
		]
	}
}

const TEST1: &'static str = "\
.....
.S-7.
.|.|.
.L-J.
.....
";

const TEST2: &'static str = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";

const TEST3: &'static str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

const TEST4: &'static str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

fn day10_1(input: &str) -> ChallengeResult {
	let map = Map::parse(input);
	let res = map.get_loop().len() / 2;

	Ok(res as u128)
}

submit!(Challenge {
	year: 2023,
	day: 10,
	part: 1,
	f: day10_1,
	unit_tests: &[(TEST1, 4), (TEST2, 4), (TEST3, 8), (TEST4, 8)],
	skip: false,
});
