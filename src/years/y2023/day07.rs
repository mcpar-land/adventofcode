use crate::util::counter::Counter;

use super::*;

static TEST: &'static str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

submit!(Challenge {
	year: 2023,
	day: 07,
	part: 1,
	f: day07_1,
	unit_tests: &[(TEST, 6440)],
	skip: false,
});

submit!(Challenge {
	year: 2023,
	day: 07,
	part: 2,
	f: day07_2,
	unit_tests: &[(TEST, 5905)],
	skip: false,
});

fn day07_1(input: &str) -> ChallengeResult {
	let hands = HandList::<NormalCard>::parse(input)?;
	Ok(hands.total_winnings())
}

fn day07_2(input: &str) -> ChallengeResult {
	let hands = HandList::<JokerCard>::parse(input)?;
	Ok(hands.total_winnings())
}

#[derive(Debug)]
struct HandList<C: Card>(Vec<Hand<C>>);

impl<C: Card> HandList<C> {
	fn parse(line: &str) -> Result<Self, anyhow::Error> {
		let hands = line
			.lines()
			.map(Hand::parse)
			.collect::<Result<Vec<Hand<C>>, anyhow::Error>>()?;
		Ok(HandList(hands))
	}
	fn total_winnings(self) -> u128 {
		let mut hands = self.0;
		hands.sort();
		let mut res = 0;
		for (i, hand) in hands.iter().enumerate() {
			// println!("{:?}: {:?}", hand.cards, hand.hand_type());
			// println!("= {} * {}", hand.bid, i + 1);
			res += hand.bid as u128 * (i + 1) as u128;
		}
		res
	}
}

#[derive(PartialEq, Eq, Debug)]
struct Hand<C: Card> {
	cards: [C; 5],
	bid: u64,
}

impl<C: Card> Hand<C> {
	fn parse(line: &str) -> Result<Self, anyhow::Error> {
		let (cards, bid) = line
			.split_once(" ")
			.ok_or_else(|| anyhow!("invalid hand {}", line))?;

		if cards.len() != 5 {
			return Err(anyhow!("invalid hand {}, expected 5 cards", cards));
		}

		let cards: [C; 5] = cards
			.chars()
			.map(C::parse)
			.collect::<Result<Vec<C>, anyhow::Error>>()?
			.try_into()
			.map_err(|_| anyhow!("couldn't convert hand to array"))?;

		let bid: u64 = bid.parse()?;

		Ok(Hand { cards, bid })
	}
	fn hand_type(&self) -> HandType {
		C::hand_type(&self.cards)
	}
}

impl<C: Card> PartialOrd for Hand<C> {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		match self.hand_type().cmp(&other.hand_type()) {
			std::cmp::Ordering::Equal => {}
			ok => return Some(ok),
		};
		for i in 0..5 {
			let (a, b) = (&self.cards[i], &other.cards[i]);
			if a != b {
				return Some(a.cmp(&b));
			}
		}
		panic!("Hands should never be equal");
	}
}

impl<C: Card> Ord for Hand<C> {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.partial_cmp(other).unwrap()
	}
}

trait Card: Sized + PartialEq + Eq + PartialOrd + Ord {
	fn parse(c: char) -> Result<Self, anyhow::Error>;
	fn hand_type(hand: &[Self; 5]) -> HandType;
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
struct NormalCard(u8);

impl Card for NormalCard {
	fn parse(c: char) -> Result<Self, anyhow::Error> {
		Ok(Self(match c {
			'2' => 02,
			'3' => 03,
			'4' => 04,
			'5' => 05,
			'6' => 06,
			'7' => 07,
			'8' => 08,
			'9' => 09,
			'T' => 10,
			'J' => 11,
			'Q' => 12,
			'K' => 13,
			'A' => 14,
			_ => return Err(anyhow!("unrecognized card '{}'", c)),
		}))
	}

	fn hand_type(hand: &[Self; 5]) -> HandType {
		let counter = Counter::<Self>::from(hand.iter());
		match counter.sorted_counts().as_slice() {
			&[5] => HandType::FiveOfAKind,
			&[4, 1] => HandType::FourOfAKind,
			&[3, 2] => HandType::FullHouse,
			&[3, 1, 1] => HandType::ThreeOfAKind,
			&[2, 2, 1] => HandType::TwoPair,
			&[2, 1, 1, 1] => HandType::OnePair,
			&[1, 1, 1, 1, 1] => HandType::HighCard,
			v => panic!("unrecognized hand {:?}", v),
		}
	}
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
struct JokerCard(u8);

impl JokerCard {
	fn is_joker(&self) -> bool {
		self.0 == 0
	}
}

impl Card for JokerCard {
	fn parse(c: char) -> Result<Self, anyhow::Error> {
		Ok(Self(match c {
			'J' => 00,
			'2' => 02,
			'3' => 03,
			'4' => 04,
			'5' => 05,
			'6' => 06,
			'7' => 07,
			'8' => 08,
			'9' => 09,
			'T' => 10,
			'Q' => 12,
			'K' => 13,
			'A' => 14,
			_ => return Err(anyhow!("unrecognized card '{}'", c)),
		}))
	}

	fn hand_type(hand: &[Self; 5]) -> HandType {
		let mut counts = Counter::<JokerCard>::new();
		let mut joker_count = 0;
		for card in hand {
			if card.is_joker() {
				joker_count += 1;
			} else {
				counts.add(card);
			}
		}

		let mut counts = counts.sorted_counts();
		if counts.len() == 0 {
			return HandType::FiveOfAKind;
		}
		counts[0] += joker_count;

		let res = if counts.len() == 0 || counts[0] >= 5 {
			HandType::FiveOfAKind
		} else if counts[0] == 4 {
			HandType::FourOfAKind
		} else if counts[0] == 3 && counts[1] >= 2 {
			HandType::FullHouse
		} else if counts[0] == 3 && counts[1] == 1 {
			HandType::ThreeOfAKind
		} else if counts[0] == 2 && counts[1] >= 2 {
			HandType::TwoPair
		} else if counts[0] == 2 && counts[1] == 1 {
			HandType::OnePair
		} else if counts[0] == 1 {
			HandType::HighCard
		} else {
			panic!("Unknown hand type {:?}", hand);
		};
		res
	}
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
	HighCard,
	OnePair,
	TwoPair,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfAKind,
}
