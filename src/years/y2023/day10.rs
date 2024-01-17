use super::*;

struct Map {
	tiles: Vec<Vec<Tile>>,
}

// impl Map {
// 	pub fn parse(input: &str) -> Self {
// 		let tiles = input
// 			.lines()
// 			.map(|line| line.chars().map(Tile::parse).collect())
// 			.collect();
// 	}
// }

enum Tile {
	Vertical,
	Horizontal,
	NorthToEast,
	NorthToWest,
	SouthToWest,
	SouthToEast,
	Ground,
	StartingPosition,
}

impl Tile {
	fn parse(c: char) -> Self {
		use Tile::*;
		match c {
			'|' => Vertical,
			'-' => Horizontal,
			'L' => NorthToEast,
			'J' => NorthToWest,
			'7' => SouthToWest,
			'F' => SouthToEast,
			'.' => Ground,
			'S' => StartingPosition,
			_ => panic!("unrecognized tile {}", c),
		}
	}
	fn connections(&self) -> Connections {
		let (north, south, east, west) = match self {
			Tile::Vertical => (true, true, false, false),
			Tile::Horizontal => (false, false, true, true),
			Tile::NorthToEast => (true, false, true, false),
			Tile::NorthToWest => (true, false, false, true),
			Tile::SouthToEast => (false, true, true, false),
			Tile::SouthToWest => (false, true, false, true),
			Tile::Ground => (false, false, false, false),
			Tile::StartingPosition => (true, true, true, true),
		};
		Connections {
			north,
			south,
			east,
			west,
		}
	}
	fn can_connect(&self, other: &Tile) -> bool {
		self.connections().can_connect(&other.connections())
	}
}

struct Connections {
	north: bool,
	south: bool,
	east: bool,
	west: bool,
}

impl Connections {
	fn can_connect(&self, other: &Self) -> bool {
		(self.north && other.south)
			|| (self.south && other.north)
			|| (self.east && self.west)
			|| (self.west && other.east)
	}
}
