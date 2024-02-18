#[cfg(test)]
mod tests {
	use chrono::{DateTime, Duration, NaiveDate, NaiveTime, Utc};

	use crate::{
		options::{DisplayConfig, Text},
		stringify_interval,
	};

	fn date_year_month_day(year: i32, month: u32, day: u32) -> DateTime<Utc> {
		NaiveDate::from_ymd_opt(year, month, day)
			.unwrap()
			.and_time(NaiveTime::default())
			.and_utc()
	}
	#[test]
	fn standard() {
		assert_eq!(
			stringify_interval(
				Duration::seconds(5_000_000),
				None,
				DisplayConfig::default_no_inconstant(),
				Text::default()
			),
			Ok(String::from("57 days, 20 hours and 53 minutes"))
		);
	}
}
