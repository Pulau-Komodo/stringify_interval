#[cfg(test)]
mod tests {
	use chrono::{DateTime, Duration, NaiveDate, NaiveTime, Utc};

	use crate::{
		options::{DisplayConfig, DisplaySettings, Text},
		with_date, without_date,
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
			without_date(
				Duration::seconds(500),
				DisplayConfig::default_no_inconstant(),
				Text::default(),
			),
			Ok(String::from("8 minutes and 20 seconds"))
		);
	}
	#[test]
	fn standard_b() {
		assert_eq!(
			without_date(
				Duration::seconds(-5_000),
				DisplayConfig::default_no_inconstant(),
				Text::default(),
			),
			Ok(String::from("1 hour and 23 minutes"))
		);
	}
	#[test]
	fn standard_c() {
		assert_eq!(
			without_date(
				Duration::seconds(50_000),
				DisplayConfig::default_no_inconstant(),
				Text::default(),
			),
			Ok(String::from("13 hours and 53 minutes"))
		);
	}
	#[test]
	fn standard_d() {
		assert_eq!(
			without_date(
				Duration::seconds(-500_000),
				DisplayConfig::default_no_inconstant(),
				Text::default(),
			),
			Ok(String::from("5 days, 18 hours and 53 minutes"))
		);
	}
	#[test]
	fn standard_e() {
		assert_eq!(
			without_date(
				Duration::seconds(5_000_000),
				DisplayConfig::default_no_inconstant(),
				Text::default(),
			),
			Ok(String::from("57 days, 20 hours and 53 minutes"))
		);
	}
	#[test]
	fn standard_f() {
		assert_eq!(
			without_date(
				Duration::seconds(50_000_000),
				DisplayConfig::default_no_inconstant(),
				Text::default(),
			),
			Ok(String::from("578 days, 16 hours and 53 minutes"))
		);
	}
	#[test]
	fn standard_g() {
		assert_eq!(
			without_date(
				Duration::seconds(-50_000_000),
				DisplayConfig::default_no_inconstant(),
				Text::default(),
			),
			Ok(String::from("578 days, 16 hours and 53 minutes"))
		);
	}

	fn config_weeks_seconds() -> DisplayConfig {
		DisplayConfig::none().with_weeks().with_seconds()
	}
	#[test]
	fn weeks_and_seconds() {
		assert_eq!(
			without_date(
				Duration::seconds(-5_000_000),
				config_weeks_seconds(),
				Text::default(),
			),
			Ok(String::from("8 weeks and 161600 seconds"))
		);
	}

	fn config_weeks_minutes_seconds() -> DisplayConfig {
		DisplayConfig {
			years: None,
			months: None,
			weeks: Some(DisplaySettings::new(0.., 0, false)),
			days: None,
			hours: None,
			minutes: Some(DisplaySettings::new(0.., 0, false)),
			seconds: Some(DisplaySettings::new(0.., 0, false)),
		}
	}
	#[test]
	fn weeks_minutes_and_seconds() {
		assert_eq!(
			without_date(
				Duration::seconds(-5_000_000),
				config_weeks_minutes_seconds(),
				Text::default(),
			),
			Ok(String::from("8 weeks, 2693 minutes and 20 seconds"))
		);
	}

	#[test]
	fn long_date() {
		assert_eq!(
			with_date(
				Duration::seconds(50_000_000),
				date_year_month_day(1950, 1, 1),
				DisplayConfig::default(),
				Text::default()
			),
			Ok(String::from(
				"1 year, 7 months, 1 day, 16 hours and 53 minutes"
			)),
		)
	}
	#[test]
	fn long_date_neg() {
		assert_eq!(
			with_date(
				Duration::seconds(-50_000_000),
				date_year_month_day(1950, 1, 1),
				DisplayConfig::default(),
				Text::default()
			),
			Ok(String::from(
				"1 year, 6 months, 29 days, 16 hours and 53 minutes"
			)),
		)
	}
}
