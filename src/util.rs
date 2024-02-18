#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct UnitValues<T> {
	pub years: T,
	pub months: T,
	pub weeks: T,
	pub days: T,
	pub hours: T,
	pub minutes: T,
	pub seconds: T,
}

impl<T> UnitValues<T> {
	pub(crate) fn iter(
		&self,
	) -> impl Iterator<Item = &T> + ExactSizeIterator + DoubleEndedIterator {
		[
			&self.years,
			&self.months,
			&self.weeks,
			&self.days,
			&self.hours,
			&self.minutes,
			&self.seconds,
		]
		.into_iter()
	}
	pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
		[
			&mut self.years,
			&mut self.months,
			&mut self.weeks,
			&mut self.days,
			&mut self.hours,
			&mut self.minutes,
			&mut self.seconds,
		]
		.into_iter()
	}
}
