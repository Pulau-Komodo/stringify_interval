use std::ops::{Range, RangeFrom};

use crate::threshold_map::ThresholdMap;

/// All the strings that may be used to compose the final output. This owns the `String`s, so if it's performance-sensitive, you will want to reuse an instance of `Text`.
/// 
/// Each of the units uses a `ThresholdMap<String>`, which allows you to have different text for different counts, like for implementing plurals.
/// 
/// By default, `joiner`, `final_joiner` and `spacer` are `", "`, `Some(" and ")` and `" "`, respectively (except `String`s).
#[derive(Debug, Clone)]
pub struct Text {
	pub years: ThresholdMap<String>,
	pub months: ThresholdMap<String>,
	pub weeks: ThresholdMap<String>,
	pub days: ThresholdMap<String>,
	pub hours: ThresholdMap<String>,
	pub minutes: ThresholdMap<String>,
	pub seconds: ThresholdMap<String>,
	/// In `"14 days[, ]6 hours and 56 minutes"`, the bracketed part.
	pub joiner: String,
	/// In `"14 days, 6 hours[ and ]56 minutes"`, the bracketed part. If none, it will use the regular joiner.
	pub final_joiner: Option<String>,
	/// In `"14[ ]days, 6[ ]hours and 56[ ]minutes"`, the bracketed parts.
	pub spacer: String,
}

/// The strings that are used in building the output. These can be adjusted to change the format or do some degree of localisation.
impl Text {
	pub(crate) fn get_joiner(&self, remaining_elements: usize) -> &str {
		match self.final_joiner.as_deref() {
			Some(final_joiner) if remaining_elements == 2 => final_joiner,
			_ if remaining_elements == 1 => "",
			_ => &self.joiner,
		}
	}
	pub(crate) fn iter_units(&self) -> impl Iterator<Item = &ThresholdMap<String>> {
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
}

impl Default for Text {
	fn default() -> Self {
		Self {
			years: ThresholdMap::from_iter("years", [(1, "year"), (2, "years")]).unwrap(),
			months: ThresholdMap::from_iter("months", [(1, "month"), (2, "months")]).unwrap(),
			weeks: ThresholdMap::from_iter("weeks", [(1, "week"), (2, "weeks")]).unwrap(),
			days: ThresholdMap::from_iter("days", [(1, "day"), (2, "days")]).unwrap(),
			hours: ThresholdMap::from_iter("hours", [(1, "hour"), (2, "hours")]).unwrap(),
			minutes: ThresholdMap::from_iter("minutes", [(1, "minute"), (2, "minutes")]).unwrap(),
			seconds: ThresholdMap::from_iter("seconds", [(1, "second"), (2, "seconds")]).unwrap(),
			joiner: ", ".into(),
			final_joiner: Some(" and ".into()),
			spacer: " ".into(),
		}
	}
}

/// The range of values a unit should be displayed at, expressed in that unit. It implements `From<Range<u64>>` and `From<RangeFrom<u64>>`.
///
/// ```
/// # use stringify_interval::DisplayRange;
/// let my_range: DisplayRange = (0..10).into();
/// assert!(!my_range.contains(15_u64));
/// let my_half_open_range: DisplayRange = (3..).into();
/// assert!(my_half_open_range.contains(15_u64));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct DisplayRange {
	lower: u64,
	upper: Option<u64>,
}

impl DisplayRange {
	pub fn contains(&self, number: impl Into<u64>) -> bool {
		let number = number.into();
		number >= self.lower && self.upper.map(|upper| number <= upper).unwrap_or(true)
	}
}

impl From<Range<u64>> for DisplayRange {
	fn from(value: Range<u64>) -> Self {
		Self {
			lower: value.start,
			upper: Some(value.end),
		}
	}
}

impl From<RangeFrom<u64>> for DisplayRange {
	fn from(value: RangeFrom<u64>) -> Self {
		Self {
			lower: value.start,
			upper: None,
		}
	}
}

/// For an individual unit, the display range, the number of digits it should be padded to, and whether it should display even when it's zero.
#[derive(Debug, Clone, Copy)]
pub struct DisplaySettings {
	pub range: DisplayRange,
	pub pad: u8,
	pub display_zero: bool,
}

impl DisplaySettings {
	pub fn new(range: impl Into<DisplayRange>, pad: u8, display_zero: bool) -> Self {
		Self {
			range: range.into(),
			pad,
			display_zero,
		}
	}
}

pub(crate) trait OptionalDisplaySettings {
	fn range_contains(&self, number: impl Into<u64>) -> bool;
}

impl OptionalDisplaySettings for Option<DisplaySettings> {
	fn range_contains(&self, number: impl Into<u64>) -> bool {
		self.is_some_and(|config| config.range.contains(number))
	}
}

/// The display settings for each unit, including years and months.
#[derive(Debug, Clone)]
pub struct DisplayConfig {
	pub years: Option<DisplaySettings>,
	pub months: Option<DisplaySettings>,
	pub weeks: Option<DisplaySettings>,
	pub days: Option<DisplaySettings>,
	pub hours: Option<DisplaySettings>,
	pub minutes: Option<DisplaySettings>,
	pub seconds: Option<DisplaySettings>,
}

impl DisplayConfig {
	pub fn default_no_inconstant() -> Self {
		Self {
			years: None,
			months: None,
			weeks: None,
			days: Some(DisplaySettings::new(0.., 0, false)),
			hours: Some(DisplaySettings::new(0.., 0, false)),
			minutes: Some(DisplaySettings::new(0.., 0, false)),
			seconds: Some(DisplaySettings::new(0..600, 0, false)),
		}
	}
	pub fn none() -> Self {
		Self {
			years: None,
			months: None,
			weeks: None,
			days: None,
			hours: None,
			minutes: None,
			seconds: None,
		}
	}
	pub fn with_years(mut self) -> Self {
		self.years = Some(DisplaySettings::new(0.., 0, false));
		self
	}
	pub fn with_months(mut self) -> Self {
		self.months = Some(DisplaySettings::new(0.., 0, false));
		self
	}
	pub fn with_weeks(mut self) -> Self {
		self.weeks = Some(DisplaySettings::new(0.., 0, false));
		self
	}
	pub fn with_days(mut self) -> Self {
		self.days = Some(DisplaySettings::new(0.., 0, false));
		self
	}
	pub fn with_hours(mut self) -> Self {
		self.hours = Some(DisplaySettings::new(0.., 0, false));
		self
	}
	pub fn with_minutes(mut self) -> Self {
		self.minutes = Some(DisplaySettings::new(0.., 0, false));
		self
	}
	pub fn with_seconds(mut self) -> Self {
		self.seconds = Some(DisplaySettings::new(0.., 0, false));
		self
	}
}

impl Default for DisplayConfig {
	fn default() -> Self {
		Self {
			years: Some(DisplaySettings::new(0.., 0, false)),
			months: Some(DisplaySettings::new(0.., 0, false)),
			weeks: None,
			days: Some(DisplaySettings::new(0.., 0, false)),
			hours: Some(DisplaySettings::new(0.., 0, false)),
			minutes: Some(DisplaySettings::new(0.., 0, false)),
			seconds: Some(DisplaySettings::new(0..600, 0, false)),
		}
	}
}

/// The display settings for each constant unit (so no years or months).
#[derive(Debug, Clone)]
pub struct DisplayConfigConstant {
	pub weeks: Option<DisplaySettings>,
	pub days: Option<DisplaySettings>,
	pub hours: Option<DisplaySettings>,
	pub minutes: Option<DisplaySettings>,
	pub seconds: Option<DisplaySettings>,
}

impl DisplayConfigConstant {
	pub fn none() -> Self {
		Self {
			weeks: None,
			days: None,
			hours: None,
			minutes: None,
			seconds: None,
		}
	}
	pub fn with_weeks(mut self) -> Self {
		self.weeks = Some(DisplaySettings::new(0.., 0, false));
		self
	}
	pub fn with_days(mut self) -> Self {
		self.days = Some(DisplaySettings::new(0.., 0, false));
		self
	}
	pub fn with_hours(mut self) -> Self {
		self.hours = Some(DisplaySettings::new(0.., 0, false));
		self
	}
	pub fn with_minutes(mut self) -> Self {
		self.minutes = Some(DisplaySettings::new(0.., 0, false));
		self
	}
	pub fn with_seconds(mut self) -> Self {
		self.seconds = Some(DisplaySettings::new(0.., 0, false));
		self
	}
}

impl Default for DisplayConfigConstant {
	fn default() -> Self {
		Self {
			weeks: None,
			days: Some(DisplaySettings::new(0.., 0, false)),
			hours: Some(DisplaySettings::new(0.., 0, false)),
			minutes: Some(DisplaySettings::new(0.., 0, false)),
			seconds: Some(DisplaySettings::new(0..600, 0, false)),
		}
	}
}

impl From<DisplayConfigConstant> for DisplayConfig {
	fn from(value: DisplayConfigConstant) -> Self {
		Self {
			years: None,
			months: None,
			weeks: value.weeks,
			days: value.days,
			hours: value.hours,
			minutes: value.minutes,
			seconds: value.seconds,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DisplayConfigRef<'l> {
	pub(crate) years: &'l Option<DisplaySettings>,
	pub(crate) months: &'l Option<DisplaySettings>,
	pub(crate) weeks: &'l Option<DisplaySettings>,
	pub(crate) days: &'l Option<DisplaySettings>,
	pub(crate) hours: &'l Option<DisplaySettings>,
	pub(crate) minutes: &'l Option<DisplaySettings>,
	pub(crate) seconds: &'l Option<DisplaySettings>,
}

impl<'l> DisplayConfigRef<'l> {
	pub(crate) fn has_inconstant_enabled(&self) -> bool {
		self.years.is_some() || self.months.is_some()
	}
	pub(crate) fn iter(&self) -> impl Iterator<Item = &Option<DisplaySettings>> {
		[
			self.years,
			self.months,
			self.weeks,
			self.days,
			self.hours,
			self.minutes,
			self.seconds,
		]
		.into_iter()
	}
}

impl<'l> From<&'l DisplayConfigConstant> for DisplayConfigRef<'l> {
	fn from(value: &'l DisplayConfigConstant) -> Self {
		Self {
			years: &None,
			months: &None,
			weeks: &value.weeks,
			days: &value.days,
			hours: &value.hours,
			minutes: &value.minutes,
			seconds: &value.seconds,
		}
	}
}

impl<'l> From<&'l DisplayConfig> for DisplayConfigRef<'l> {
	fn from(value: &'l DisplayConfig) -> Self {
		Self {
			years: &value.years,
			months: &value.months,
			weeks: &value.weeks,
			days: &value.days,
			hours: &value.hours,
			minutes: &value.minutes,
			seconds: &value.seconds,
		}
	}
}
