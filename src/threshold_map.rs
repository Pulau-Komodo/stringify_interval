/// A map that stores values and thresholds that determine what value belongs to any given number.
#[derive(Debug, Clone)]
pub struct ThresholdMap<T> {
	thresholds: Vec<u64>,
	values: Vec<T>,
}

impl<T> ThresholdMap<T> {
	/// A map that will return the same value for any input.
	pub fn single_value<I: Into<T>>(value: I) -> Self {
		Self::with_capacity(0, value.into())
	}
	/// Capacity is the number of thresholds (excluding the ever-present virtual 0 threshold), which is one less than the number of values.
	pub fn with_capacity(capacity: usize, lowest_value: T) -> Self {
		let thresholds = Vec::with_capacity(capacity);
		let mut values = Vec::with_capacity(capacity + 1);
		values.push(lowest_value);
		Self { thresholds, values }
	}
	/// Creates a new `ThresholdMap` from an iterator yielding thresholds and values applying at or over those thresholds.
	///
	/// The lowest value is essentially the value for threshold 0.
	///
	/// Requires that all the thresholds in the iterator are in order, and fails if that is not the case.
	///
	/// ```
	/// # use stringify_interval::ThresholdMap;
	/// #
	/// let map = ThresholdMap::<String>::from_iter("years", [(1, "year"), (2, "years")]).unwrap();
	/// assert_eq!(map.get(25), &String::from("years")); // Or any value other than 1.
	/// assert_eq!(map.get(1), &String::from("year"));
	/// ```
	pub fn from_iter<V: Into<T>>(
		lowest_value: V,
		iter: impl IntoIterator<Item = (u64, V)>,
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
	pub fn insert(&mut self, threshold: u64, value: T) -> bool {
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
	pub fn push(&mut self, threshold: u64, value: T) -> bool {
		if let Some(last) = self.thresholds.last() {
			if *last >= threshold {
				return false;
			}
		}
		self.thresholds.push(threshold);
		self.values.push(value);
		true
	}
	/// Gets the value associated with the first threshold crossed, or the first one, since the virtual threshold of 0 is always crossed.
	pub fn get(&self, key: u64) -> &T {
		let index = self
			.thresholds
			.iter()
			.position(|&threshold| key < threshold)
			.unwrap_or(self.thresholds.len());
		&self.values[index]
	}
	fn get_index(&self, key: &u64) -> Result<usize, usize> {
		self.thresholds.binary_search(key)
	}
}
