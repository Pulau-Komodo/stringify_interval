use chrono::{DateTime, Duration, Utc};

use errors::StringifyError;
use options::{DisplayConfig, DisplayConfigConstant, Text};
use stringify::stringify_interval;

mod errors;
mod options;
mod stringify;
mod tests;
mod threshold_map;
mod util;

pub fn without_date(
	interval: Duration,
	config: DisplayConfigConstant,
	text: Text,
) -> Result<String, StringifyError> {
	stringify_interval(interval, None, config.into(), text)
}

pub fn with_date(
	interval: Duration,
	date: DateTime<Utc>,
	config: DisplayConfig,
	text: Text,
) -> Result<String, StringifyError> {
	stringify_interval(interval, Some(Box::new(move || date)), config, text)
}

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

pub fn with_now(
	interval: Duration,
	config: DisplayConfig,
	text: Text,
) -> Result<String, StringifyError> {
	with_lazy_date(interval, Utc::now, config, text)
}
