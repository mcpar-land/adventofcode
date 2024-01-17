use super::*;

submit!(Challenge {
	year: 2023,
	day: 04,
	part: 1,
	f: day04_1,
	unit_tests: &[
		("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8),
		("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2),
		("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2),
		("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1),
		("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0),
		("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0),
	],
	skip: false,
});

struct Card {
	winners: Vec<u128>,
	numbers: Vec<u128>,
}

impl Card {
	fn parse(line: &str) -> Result<Self, anyhow::Error> {
		let (_, res) = line
			.split_once(":")
			.ok_or(anyhow!("Malformed line: {}", line))?;
		let (a, b) = res
			.split_once("|")
			.ok_or(anyhow!("Malformed line: {}", line))?;
		let winners = parse_number_list(a)?;
		let numbers = parse_number_list(b)?;
		Ok(Card { winners, numbers })
	}
	fn n_matches(&self) -> u128 {
		let mut count = 0;
		for winner in &self.winners {
			if self.numbers.contains(winner) {
				count += 1;
			}
		}
		count
	}
	fn score(&self) -> u128 {
		let mut score = 0;
		for winner in &self.winners {
			if self.numbers.contains(winner) {
				if score == 0 {
					score = 1;
				} else {
					score *= 2;
				}
			}
		}
		score
	}
}

fn parse_number_list(list: &str) -> Result<Vec<u128>, anyhow::Error> {
	let res = list
		.split_ascii_whitespace()
		.map(|v| v.parse().map_err(|_| anyhow!("Malformed list: {}", list)))
		.collect::<Result<Vec<u128>, anyhow::Error>>()?;
	Ok(res)
}

fn day04_1(input: &str) -> ChallengeResult {
	let mut sum = 0;
	for line in input.lines() {
		let card = Card::parse(line)?;
		sum += card.score();
	}
	Ok(sum)
}

const TEST_02: &'static str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

submit!(Challenge {
	year: 2023,
	day: 04,
	part: 2,
	f: day04_2,
	unit_tests: &[(TEST_02, 30)],
	skip: false,
});

fn day04_2(input: &str) -> ChallengeResult {
	let cards = input
		.lines()
		.map(Card::parse)
		.collect::<Result<Vec<Card>, anyhow::Error>>()?;

	let mut card_counts = vec![1; cards.len()];

	for (i, card) in cards.iter().enumerate() {
		let card_count = card_counts[i];
		let score = card.n_matches();
		// println!("===== card {} (score {})", i + 1, score);
		if score == 0 {
			continue;
		}

		for j in i + 1..i + 1 + score as usize {
			if j < cards.len() {
				// println!("Won {} instances of card {}", card_count, j + 1);
				card_counts[j] += card_count;
			}
		}
	}

	Ok(card_counts.iter().sum())
}
