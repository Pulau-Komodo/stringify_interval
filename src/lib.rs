use std::fmt::Write;

use chrono::{DateTime, Datelike, Duration, Months, Utc};

use errors::StringifyError;
use options::{DisplayConfig, Text};
use threshold_map::ThresholdMap;
use util::UnitValues;

mod errors;
mod options;
mod tests;
mod threshold_map;
mod util;

pub fn stringify_interval(
	mut interval: Duration,
	get_date: Option<Box<dyn FnOnce() -> DateTime<Utc>>>,
	config: DisplayConfig,
	text: Text,
) -> Result<String, StringifyError> {
	let in_past = interval.num_seconds() < 0;
	interval = interval.abs();

	let mut enabled = EnabledUnits::from_interval_and_display_config(interval, &config);

	let round_to_months_or_years =
		if let Some(seconds) = enabled.round_to_smallest(interval.num_seconds() as u64) {
			interval = Duration::seconds(seconds as i64);
			false
		} else {
			true
		};

	let mut counts = UnitValues::<u64>::default();

	if config.has_inconstant_enabled() {
		let date = get_date.ok_or(StringifyError::InconstantWithoutDate)?();
		let (years, months, remainder) =
			get_years_months_remainder(date, interval, in_past, round_to_months_or_years, &config)
				.ok_or(StringifyError::NumberOutOfRange)?;
		if let Some(years) = years {
			enabled.0.years = true;
			counts.years = years;
		}
		if let Some(months) = months {
			enabled.0.months = true;
			counts.months = months;
		}
		interval = remainder;
	};

	let mut seconds = interval.num_seconds() as u64;
	for (count, seconds_per) in counts
		.iter_mut()
		.zip(SECONDS_PER.iter())
		.zip(enabled.0.iter())
		.skip(2)
		.filter_map(|(v, enabled)| enabled.then_some(v))
	{
		*count += seconds / seconds_per;
		seconds %= seconds_per;
	}
	if seconds != 0 {
		eprintln!("Something went wrong with rounding.");
	}

	for ((enabled, count), config) in enabled.0.iter_mut().zip(counts.iter()).zip(config.iter()) {
		*enabled = *enabled && (*count > 0 || config.unwrap().display_zero);
	}

	let mut remaining_elements = enabled.0.iter().filter(|e| **e).count();

	let mut output = String::new();
	for (&count, labels, config) in counts
		.iter()
		.zip(text.iter_units())
		.zip(config.iter())
		.zip(enabled.0.iter())
		.filter_map(|(((ct, t), cfg), e)| e.then_some((ct, t, cfg)))
	{
		print_unit(
			&mut output,
			remaining_elements,
			count
				.try_into()
				.map_err(|_| StringifyError::NumberOutOfRange)?,
			&text.spacer,
			labels,
			&text.joiner,
			text.final_joiner.as_deref(),
			config.unwrap().pad,
		);
		remaining_elements -= 1;
	}

	Ok(output)
}

fn print_unit(
	output: &mut String,
	remaining_elements: usize,
	count: u32,
	spacer: &str,
	label: &ThresholdMap<String>,
	joiner: &str,
	final_joiner: Option<&str>,
	pad: u8,
) {
	let joiner = match final_joiner {
		Some(final_joiner) if remaining_elements == 2 => final_joiner,
		_ if remaining_elements == 1 => "",
		_ => joiner,
	};
	write!(
		output,
		"{}{}{:0pad$}{}",
		count,
		spacer,
		label.get(count),
		joiner,
		pad = pad as usize
	)
	.unwrap();
}

#[derive(Debug)]
struct EnabledUnits(UnitValues<bool>);

impl EnabledUnits {
	fn from_interval_and_display_config(interval: Duration, config: &DisplayConfig) -> Self {
		Self(UnitValues {
			years: false,
			months: false,
			weeks: config
				.weeks
				.is_some_and(|weeks| weeks.range.contains(interval.num_weeks() as u64)),
			days: config
				.days
				.is_some_and(|days| days.range.contains(interval.num_days() as u64)),
			hours: config
				.hours
				.is_some_and(|hours| hours.range.contains(interval.num_hours() as u64)),
			minutes: config
				.minutes
				.is_some_and(|minutes| minutes.range.contains(interval.num_minutes() as u64)),
			seconds: config
				.seconds
				.is_some_and(|seconds| seconds.range.contains(interval.num_seconds() as u64)),
		})
	}
	fn round_to_smallest(&self, seconds: u64) -> Option<u64> {
		let rounded = if self.0.seconds {
			seconds
		} else if self.0.minutes {
			round_to_nearest_multiple(seconds, SECONDS_PER.minutes)
		} else if self.0.hours {
			round_to_nearest_multiple(seconds, SECONDS_PER.hours)
		} else if self.0.days {
			round_to_nearest_multiple(seconds, SECONDS_PER.days)
		} else if self.0.weeks {
			round_to_nearest_multiple(seconds, SECONDS_PER.weeks)
		} else {
			return None;
		};
		Some(rounded)
	}
}

const SECONDS_PER: UnitValues<u64> = UnitValues {
	years: 0,
	months: 0,
	weeks: 7 * 24 * 60 * 60,
	days: 24 * 60 * 60,
	hours: 60 * 60,
	minutes: 60,
	seconds: 1,
};

fn round_to_nearest_multiple(n: u64, m: u64) -> u64 {
	(n + m / 2) / m * m
}

fn get_years_months_remainder(
	start_date: DateTime<Utc>,
	interval: Duration,
	in_past: bool,
	should_round: bool,
	config: &DisplayConfig,
) -> Option<(Option<u64>, Option<u64>, Duration)> {
	let target_date = start_date.checked_add_signed(interval)?;

	let (larger_date, smaller_date) = if in_past {
		(start_date, target_date)
	} else {
		(target_date, start_date)
	};
	let mut months = ((larger_date.year() - smaller_date.year()) * 12 + larger_date.month() as i32
		- smaller_date.month() as i32) as u32;

	let adjusted_date = if in_past {
		let new_date = start_date.checked_sub_months(Months::new(months))?;
		if new_date < target_date {
			// Went too far
			months -= 1;
			start_date.checked_sub_months(Months::new(months))?
		} else {
			new_date
		}
	} else {
		let new_date = start_date.checked_add_months(Months::new(months))?;
		if new_date > target_date {
			// Went too far
			months -= 1;
			start_date.checked_add_months(Months::new(months))?
		} else {
			new_date
		}
	};

	let years = months / 12;

	let enable_years = config.years.is_some_and(|year| year.range.contains(years));
	let enable_months = config
		.years
		.is_some_and(|month| month.range.contains(months));

	match (should_round, enable_years, enable_months) {
		(_, false, false) => Some((None, None, interval)), // Neither ends up displayed
		(false, true, false) => {
			// Only years
			let months = Months::new(years * 12);
			let adjusted_date = if in_past {
				start_date.checked_sub_months(months)?
			} else {
				start_date.checked_add_months(months)?
			};
			let remaining_interval = interval - (start_date - adjusted_date).abs();
			Some((Some(years as u64), None, remaining_interval))
		}
		(false, false, true) => {
			// Only months
			let remaining_interval = interval - (start_date - adjusted_date).abs();
			Some((None, Some(months as u64), remaining_interval))
		}
		(false, true, true) => {
			// Both years and months
			let remaining_interval = interval - (start_date - adjusted_date).abs();
			Some((
				Some(years as u64),
				Some(months as u64 % 12),
				remaining_interval,
			))
		}
		(true, true, false) => {
			// Only years and round to them
			let mut years: u32 = (larger_date.year() - smaller_date.year()).try_into().ok()?;
			let adjusted_date = if in_past {
				let new_date = start_date.checked_sub_months(Months::new(years * 12))?;
				if new_date < target_date {
					// Went too far
					years -= 1;
					start_date.checked_sub_months(Months::new(years * 12))?
				} else {
					new_date
				}
			} else {
				let new_date = start_date.checked_add_months(Months::new(years * 12))?;
				if new_date > target_date {
					// Went too far
					years -= 1;
					start_date.checked_add_months(Months::new(years * 12))?
				} else {
					new_date
				}
			};
			if is_one_year_further_closer(target_date, adjusted_date, in_past)? {
				years += 1;
			}
			Some((Some(years as u64), None, Duration::zero()))
		}
		(true, false, true) => {
			// Only months and round to them
			if is_one_month_further_closer(target_date, adjusted_date, in_past)? {
				months += 1;
			}
			Some((None, Some(months as u64), Duration::zero()))
		}
		(true, true, true) => {
			// Years and months and round to months
			if is_one_month_further_closer(target_date, adjusted_date, in_past)? {
				months += 1;
			}
			Some((
				Some(months as u64 / 12),
				Some(months as u64 % 12),
				Duration::zero(),
			))
		}
	}
}

fn is_n_months_further_closer(
	target_date: DateTime<Utc>,
	date_before: DateTime<Utc>,
	in_past: bool,
	n: u32,
) -> Option<bool> {
	let month_further = if in_past {
		date_before.checked_sub_months(Months::new(n))?
	} else {
		date_before.checked_add_months(Months::new(n))?
	};
	Some((target_date - date_before).abs() > (target_date - month_further).abs())
}

fn is_one_month_further_closer(
	target_date: DateTime<Utc>,
	date_before: DateTime<Utc>,
	in_past: bool,
) -> Option<bool> {
	is_n_months_further_closer(target_date, date_before, in_past, 1)
}

fn is_one_year_further_closer(
	target_date: DateTime<Utc>,
	date_before: DateTime<Utc>,
	in_past: bool,
) -> Option<bool> {
	is_n_months_further_closer(target_date, date_before, in_past, 12)
}
