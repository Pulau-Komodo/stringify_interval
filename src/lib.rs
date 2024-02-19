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

/// Stringify an interval with a configurable format. The default looks like "14 days, 6 hours and 56 minutes".
pub fn without_date(
	interval: Duration,
	config: DisplayConfigConstant,
	text: Text,
) -> Result<String, StringifyError> {
	stringify_interval(interval, None, config.into(), text)
}

/// Stringify an interval with a configurable format. Years and months can be included, and they will be calculated with the given date as a reference point. The default looks like "14 days, 6 hours and 56 minutes".
pub fn with_date(
	interval: Duration,
	date: DateTime<Utc>,
	config: DisplayConfig,
	text: Text,
) -> Result<String, StringifyError> {
	stringify_interval(interval, Some(Box::new(move || date)), config, text)
}

/// Stringify an interval with a configurable format. Years and months can be included, and they will be calculated with the date yielded by the given closure as a reference point. The default looks like "14 days, 6 hours and 56 minutes".
pub fn with_lazy_date<D>(
	interval: Duration,
	get_date: D,
	config: DisplayConfig,
	text: Text,
) -> Result<String, StringifyError>
where
	D: FnOnce() -> DateTime<Utc> + 'static,
{
	stringify_interval(interval, Some(Box::new(get_date)), config, text)
}

/// Stringify an interval with a configurable format. Years and months can be included, and they will be calculated with the current system time as a reference point. The default looks like "14 days, 6 hours and 56 minutes".
pub fn with_now(
	interval: Duration,
	config: DisplayConfig,
	text: Text,
) -> Result<String, StringifyError> {
	with_lazy_date(interval, Utc::now, config, text)
}
