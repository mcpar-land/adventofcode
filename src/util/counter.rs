use std::collections::HashMap;
use std::hash::Hash;

pub struct Counter<T: Hash + PartialEq + Eq + Clone>(HashMap<T, usize>);

impl<T: Hash + PartialEq + Eq + Clone> Counter<T> {
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	pub fn add(&mut self, key: &T) {
		if let Some(v) = self.0.get_mut(key) {
			*v += 1;
		} else {
			self.0.insert(key.clone(), 1);
		}
	}

	pub fn get(&self, key: &T) -> usize {
		self.0.get(key).cloned().unwrap_or(0)
	}

	pub fn sorted_counts(&self) -> Vec<usize> {
		let mut res: Vec<usize> = self.0.iter().map(|(_, count)| *count).collect();
		res.sort();
		res.reverse();
		res
	}
}

impl<'a, T, I> From<I> for Counter<T>
where
	T: Hash + PartialEq + Eq + Clone + 'a,
	I: Iterator<Item = &'a T>,
{
	fn from(iter: I) -> Self {
		let mut res = Self::new();
		for v in iter {
			res.add(v);
		}
		res
	}
}
