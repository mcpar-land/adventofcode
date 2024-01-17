use std::{collections::HashMap, hash::Hash};

pub struct TwoWayHashMap<A: Hash + PartialEq + Eq, B: Hash + PartialEq + Eq> {
	a_to_b: HashMap<A, B>,
	b_to_a: HashMap<B, A>,
}

impl<A: Hash + PartialEq + Eq, B: Hash + PartialEq + Eq> TwoWayHashMap<A, B> {
	pub fn new() -> Self {
		Self {
			a_to_b: HashMap::new(),
			b_to_a: HashMap::new(),
		}
	}
	pub fn insert(&mut self, a: A, b: B) {
		self.a_to_b.insert(a, b);
	}
	pub fn contains_a(&mut self, a: &A) -> bool {
		self.a_to_b.contains_key(a)
	}
	pub fn contains_b(&mut self, b: &B) -> bool {
		self.b_to_a.contains_key(b)
	}
	pub fn contains_ab(&mut self, a: &A, b: &B) -> bool {
		self.a_to_b.get(a).map(|_b| _b == b).unwrap_or(false)
	}
	pub fn get_a(&self, a: &A) -> Option<&B> {
		self.a_to_b.get(a)
	}
	pub fn get_b(&self, b: &B) -> Option<&A> {
		self.b_to_a.get(b)
	}
	pub fn remove_a(&mut self, a: &A) -> Option<B> {
		if let Some(b) = self.a_to_b.remove(a) {
			self.b_to_a.remove(&b);
			Some(b)
		} else {
			None
		}
	}
	pub fn remove_b(&mut self, b: &B) -> Option<A> {
		if let Some(a) = self.b_to_a.remove(b) {
			self.a_to_b.remove(&a);
			Some(a)
		} else {
			None
		}
	}
}
