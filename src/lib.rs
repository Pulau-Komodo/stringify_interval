//! stringify_interval generates a user-friendly string from a `chrono::Duration`, like "1 day, 5 hours and 20 minutes". Years and months can be displayed, but they will need some date as a reference point, because the exact length of a year or month can vary.
//!
//! It can be configured to show different units depending on the size of the interval, and to customize the strings used to compose the output.
//! 
//! ```
//! # use stringify_interval::{DisplayConfigConstant, Text};
//! let text = stringify_interval::without_date(
//! 	chrono::Duration::seconds(1_234_567),
//! 	&DisplayConfigConstant::default(),
//! 	&Text::default(),
//! );
//! assert_eq!(text, Ok(String::from("14 days, 6 hours and 56 minutes")));
//!```

use chrono::{DateTime, Duration, Utc};

use errors::StringifyError;
use stringify::stringify_interval;

pub mod errors;
mod options;
mod stringify;
mod tests;
mod threshold_map;
mod util;

pub use options::{DisplayConfig, DisplayConfigConstant, DisplayRange, DisplaySettings, Text};
pub use threshold_map::ThresholdMap;

/// Stringify an interval with a configurable format. Years and months cannot be included.
/// 
/// The default looks like "14 days, 6 hours and 56 minutes".
pub fn without_date(
	interval: Duration,
	config: &DisplayConfigConstant,
	text: &Text,
) -> Result<String, StringifyError> {
	stringify_interval(interval, None, config.into(), text)
}

/// Stringify an interval with a configurable format. Years and months can be included, and they will be calculated with the given date as a reference point.
/// 
/// The default looks like "14 days, 6 hours and 56 minutes".
pub fn with_date(
	interval: Duration,
	date: DateTime<Utc>,
	config: &DisplayConfig,
	text: &Text,
) -> Result<String, StringifyError> {
	stringify_interval(interval, Some(Box::new(move || date)), config.into(), text)
}

/// Stringify an interval with a configurable format. Years and months can be included, and they will be calculated with the date yielded by the given closure as a reference point.
/// 
/// The default looks like "14 days, 6 hours and 56 minutes".
pub fn with_lazy_date<D>(
	interval: Duration,
	get_date: D,
	config: &DisplayConfig,
	text: &Text,
) -> Result<String, StringifyError>
where
	D: FnOnce() -> DateTime<Utc> + 'static,
{
	stringify_interval(interval, Some(Box::new(get_date)), config.into(), text)
}

/// Stringify an interval with a configurable format. Years and months can be included, and they will be calculated with the current system time as a reference point.
/// 
/// The default looks like "14 days, 6 hours and 56 minutes".
pub fn with_now(
	interval: Duration,
	config: &DisplayConfig,
	text: &Text,
) -> Result<String, StringifyError> {
	with_lazy_date(interval, Utc::now, config, text)
}
