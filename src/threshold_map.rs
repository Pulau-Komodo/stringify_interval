/// A map that stores values and thresholds that determine what value belongs to any given number.
#[derive(Debug, Clone)]
pub struct ThresholdMap<T> {
	thresholds: Vec<u32>,
	values: Vec<T>,
}

impl<T> ThresholdMap<T> {
	/// Capacity is the number of thresholds, not the number of values.
	pub fn with_capacity(capacity: usize, lowest_value: T) -> Self {
		let thresholds = Vec::with_capacity(capacity);
		let mut values = Vec::with_capacity(capacity + 1);
		values.push(lowest_value);
		Self { thresholds, values }
	}
	/// Creates a new `ThresholdMap` from an iterator yielding thresholds and values applying at or over those thresholds.
	///
	/// The lowest value is essentially the value for threshold 0.
	pub fn from_iter<V: Into<T>>(
		lowest_value: V,
		iter: impl IntoIterator<Item = (u32, V)>,
	) -> Option<Self> {
		let iter = iter.into_iter();
		let mut map = Self::with_capacity(iter.size_hint().1.unwrap_or(0), lowest_value.into());
		for (threshold, value) in iter {
			if !map.push(threshold, value.into()) {
				return None;
			}
		}
		Some(map)
	}
	/// Insert a new threshold and value to apply at and beyond that threshold.
	///
	/// Fails if the threshold was already in the map. Returns whether it succeeded.
	pub fn insert(&mut self, threshold: u32, value: T) -> bool {
		let Err(index) = self.get_index(&threshold) else {
			return false;
		};
		self.thresholds.insert(index, threshold);
		self.values.insert(index + 1, value);
		true
	}
	/// Pushes a new threshold to the end of the list, and the value to apply at and beyond that threshold.
	///
	/// Fails if the threshold was not larger than the last threshold. Returns whether it succeeded.
	pub fn push(&mut self, threshold: u32, value: T) -> bool {
		if let Some(last) = self.thresholds.last() {
			if *last >= threshold {
				return false;
			}
		}
		self.thresholds.push(threshold);
		self.values.push(value);
		true
	}
	pub fn get(&self, key: u32) -> &T {
		let index = self
			.thresholds
			.iter()
			.position(|&threshold| key < threshold)
			.unwrap_or(self.thresholds.len());
		&self.values[index]
	}
	fn get_index(&self, key: &u32) -> Result<usize, usize> {
		self.thresholds.binary_search(key)
	}
}
