use super::*;

const DAY5_DEBUG: bool = false;

const TEST_01: &'static str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

struct Almanac {
	seeds: Vec<u128>,
	seed_pairs: Vec<std::ops::Range<u128>>,
	maps: Vec<Map>,
}

impl Almanac {
	pub fn parse(input: &str) -> Result<Almanac, anyhow::Error> {
		let mut sections = input.split("\n\n");
		let seeds = sections
			.next()
			.unwrap()
			.strip_prefix("seeds: ")
			.unwrap()
			.split_ascii_whitespace()
			.map(|d| -> Result<u128, anyhow::Error> {
				Ok(
					d.parse()
						.map_err(|e| anyhow::anyhow!("Error parsing {} - {}", d, e))?,
				)
			})
			.collect::<Result<Vec<u128>, anyhow::Error>>()?;

		if seeds.len() % 2 != 0 {
			return Err(anyhow::anyhow!("Needs an even number of seeds"));
		}

		let mut seed_pairs: Vec<std::ops::Range<u128>> = seeds
			.chunks(2)
			.map(|chunk| (chunk[0]..(chunk[0] + chunk[1])))
			.collect();

		seed_pairs.sort_by_key(|r| r.start);

		let maps = sections
			.map(|section| {
				Map::parse(section)
					.map_err(|e| anyhow::anyhow!("Error parsing {} - {}", section, e))
			})
			.collect::<Result<Vec<Map>, anyhow::Error>>()?;

		Ok(Almanac {
			maps,
			seeds,
			seed_pairs,
		})
	}

	pub fn run(&self, mut val: u128) -> u128 {
		for m in &self.maps {
			val = m.run(val);
		}
		val
	}

	pub fn run_backwards(&self, mut val: u128) -> u128 {
		for m in self.maps.iter().rev() {
			val = m.run_backwards(val);
		}
		val
	}

	pub fn has_seed(&self, seed: u128, use_ranges: bool) -> bool {
		if use_ranges {
			for r in &self.seed_pairs {
				if r.contains(&seed) {
					return true;
				}
			}
			false
		} else {
			self.seeds.contains(&seed)
		}
	}

	pub fn smallest_location(self, use_ranges: bool) -> u128 {
		// use indicatif::ParallelProgressIterator;
		use rayon::prelude::*;
		if use_ranges {
			// running it backwards - the smallest number must be here somewhere.
			let max_location = self
				.maps
				.last()
				.unwrap()
				.map
				.iter()
				.min_by_key(|m| m.dest)
				.unwrap()
				.dest;

			let res = (0..=max_location)
				.into_par_iter()
				// .progress_count(max_location as u64)
				.filter(|location_id| {
					let seed = self.run_backwards(*location_id);
					self.has_seed(seed, use_ranges)
				})
				.min()
				.unwrap();

			res
		} else {
			let mut res = u128::MAX;

			for seed in &self.seeds {
				// println!("=======");
				res = res.min(self.run(*seed));
			}
			res
		}
	}
}

struct Map {
	from: String,
	to: String,
	map: Vec<MapRange>,
	map_backwards: Vec<MapRange>,
}

impl Map {
	pub fn parse(lines: &str) -> Result<Map, anyhow::Error> {
		let mut lines = lines.lines();
		let first_line = lines.next().unwrap();

		let (from, to) = first_line.split_once("-to-").unwrap();
		let to = to.strip_suffix(" map:").unwrap();

		let mut map = lines
			.map(MapRange::parse)
			.collect::<Result<Vec<MapRange>, anyhow::Error>>()?;
		map.sort_by_key(|v| v.start);

		let mut map_backwards = map.clone();
		map_backwards.sort_by_key(|v| v.dest);

		Ok(Map {
			from: from.to_string(),
			to: to.to_string(),
			map,
			map_backwards,
		})
	}
	pub fn run(&self, input: u128) -> u128 {
		for r in &self.map {
			if input >= r.start {
				if let Some(res) = r.run(input) {
					return res;
				}
			}
		}
		input
	}
	pub fn run_backwards(&self, input: u128) -> u128 {
		for r in &self.map_backwards {
			if input >= r.dest {
				if let Some(res) = r.run_backwards(input) {
					if DAY5_DEBUG {
						println!("{}->{} mapped {}->{}", self.to, self.from, input, res);
					}
					return res;
				}
			}
		}
		if DAY5_DEBUG {
			println!("{}->{} passed through {}", self.to, self.from, input);
		}
		input
	}
}

#[derive(Debug, Clone)]
struct MapRange {
	pub dest: u128,
	pub start: u128,
	pub len: u128,
}

impl MapRange {
	pub fn parse(line: &str) -> Result<MapRange, anyhow::Error> {
		let mut numerals = line.split_ascii_whitespace();
		let dest: u128 = numerals.next().unwrap().parse()?;
		let start: u128 = numerals.next().unwrap().parse()?;
		let len: u128 = numerals.next().unwrap().parse()?;

		Ok(MapRange { dest, start, len })
	}

	fn run(&self, val: u128) -> Option<u128> {
		if val < self.start || val >= (self.start + self.len) {
			None
		} else {
			if self.start > self.dest {
				Some(val - (self.start - self.dest))
			} else {
				Some(val + (self.dest - self.start))
			}
		}
	}

	fn run_backwards(&self, val: u128) -> Option<u128> {
		if val < self.dest || val >= (self.dest + self.len) {
			None
		} else {
			if self.dest > self.start {
				Some(val - (self.dest - self.start))
			} else {
				Some(val + (self.start - self.dest))
			}
		}
	}
}

submit!(Challenge {
	year: 2023,
	day: 05,
	part: 1,
	f: day05_1,
	unit_tests: &[(TEST_01, 35)],
	skip: false,
});

fn day05_1(input: &str) -> ChallengeResult {
	let almanac = Almanac::parse(input)?;

	Ok(almanac.smallest_location(false) as u128)
}

submit!(Challenge {
	year: 2023,
	day: 05,
	part: 2,
	f: day05_2,
	unit_tests: &[(TEST_01, 46)],
	skip: false,
});

fn day05_2(input: &str) -> ChallengeResult {
	let almanac = Almanac::parse(input)?;

	Ok(almanac.smallest_location(true) as u128)
}
