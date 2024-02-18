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
	fn standard_a() {
		assert_eq!(
			stringify_interval(
				Duration::seconds(500),
				None,
				DisplayConfig::default_no_inconstant(),
				Text::default(),
			),
			Ok(String::from("8 minutes and 20 seconds"))
		);
	}
	#[test]
	fn standard_b() {
		assert_eq!(
			stringify_interval(
				Duration::seconds(-5_000),
				None,
				DisplayConfig::default_no_inconstant(),
				Text::default(),
			),
			Ok(String::from("1 hour and 23 minutes"))
		);
	}
	#[test]
	fn standard_c() {
		assert_eq!(
			stringify_interval(
				Duration::seconds(50_000),
				None,
				DisplayConfig::default_no_inconstant(),
				Text::default(),
			),
			Ok(String::from("13 hours and 53 minutes"))
		);
	}
	#[test]
	fn standard_d() {
		assert_eq!(
			stringify_interval(
				Duration::seconds(-500_000),
				None,
				DisplayConfig::default_no_inconstant(),
				Text::default(),
			),
			Ok(String::from("5 days, 18 hours and 53 minutes"))
		);
	}
	#[test]
	fn standard_e() {
		assert_eq!(
			stringify_interval(
				Duration::seconds(5_000_000),
				None,
				DisplayConfig::default_no_inconstant(),
				Text::default(),
			),
			Ok(String::from("57 days, 20 hours and 53 minutes"))
		);
	}
}
