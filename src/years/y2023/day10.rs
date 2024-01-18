use std::collections::HashSet;

use owo_colors::OwoColorize;

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

	fn enclosed(&self) -> (HashSet<Pos>, HashSet<Pos>) {
		// first we need the loop tiles to tell where the border is.
		let loop_tiles = self.get_loop();

		let mut contained_tiles = HashSet::<Pos>::new();

		// now we set up a list of poisoned tiles that are contiguous
		// with the edge of the map.
		let mut poisoned_tiles = HashSet::<Pos>::new();

		for y in 0..self.size.y {
			for x in 0..self.size.x {
				let (contig, poison) = self._contiguous(
					&loop_tiles,
					&poisoned_tiles,
					HashSet::new(),
					Pos::new(x, y * -1),
				);
				if poison {
					poisoned_tiles.extend(&contig);
				} else {
					// we made it. now we filter by ground tiles.
					contained_tiles.extend(&contig);
				}
			}
		}

		return (
			poisoned_tiles,
			contained_tiles
				.iter()
				.filter(|pos| self.get(**pos).unwrap() == &Tile::Ground)
				.cloned()
				.collect(),
		);
	}

	/// returns all contiguous tiles + whether it touches the edge (or poison tiles)
	fn _contiguous(
		&self,
		loop_tiles: &HashSet<Pos>,
		poison_tiles: &HashSet<Pos>,
		mut visited: HashSet<Pos>,
		pos: Pos,
	) -> (HashSet<Pos>, bool) {
		if visited.contains(&pos) {
			return (visited, false);
		}
		if loop_tiles.contains(&pos) {
			// we're on a loop tile, so we're done.
			return (visited, false);
		}
		if poison_tiles.contains(&pos) {
			// we're on a poison tile, so we are poisoned!
			return (visited, true);
		}
		let tile = self.get(pos);
		if tile.is_none() {
			// we reached the border, so we are poisoned!
			return (visited, true);
		}
		visited.insert(pos);
		for x_offset in -1..=1 {
			for y_offset in -1..=1 {
				let new_pos = pos + Pos::new(x_offset, y_offset);
				let (new_visited, is_poisoned) =
					self._contiguous(loop_tiles, poison_tiles, visited, new_pos);
				if is_poisoned {
					return (new_visited, true);
				}
				visited = new_visited;
			}
		}
		return (visited, false);
	}

	fn pretty_print(&self) {
		let loop_tiles = self.get_loop();
		let (poisoned_tiles, enclosed_tiles) = self.enclosed();
		let mut displ = String::new();
		for y in 0..self.size.y {
			let mut row = String::new();
			for x in 0..self.size.x {
				let pos = Pos::new(x, y * -1);
				if poisoned_tiles.contains(&pos) {
					row.push_str(&"P".red().to_string());
					continue;
				}
				if loop_tiles.contains(&pos) {
					row.push_str(&"#".purple().to_string());
					continue;
				}
				if enclosed_tiles.contains(&pos) {
					row.push_str(&"I".green().to_string());
					continue;
				}
				row.push('.');
			}
			displ.push_str(&row);
			displ.push('\n');
		}
		println!("{}\n", displ);
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
	map.pretty_print();
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

const TEST2_1: &'static str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

const TEST2_2: &'static str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

const TEST2_3: &'static str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

fn day10_2(input: &str) -> ChallengeResult {
	let map = Map::parse(input);
	map.pretty_print();
	let res = map.enclosed().1.len();

	Ok(res as u128)
}

submit!(Challenge {
	year: 2023,
	day: 10,
	part: 2,
	f: day10_2,
	unit_tests: &[(TEST2_1, 4), (TEST2_2, 8), (TEST2_3, 10)],
	skip: false,
});
