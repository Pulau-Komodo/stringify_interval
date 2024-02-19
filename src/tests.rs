#[cfg(test)]
mod tests {
	use chrono::{DateTime, Duration, NaiveDate, NaiveTime, Utc};

	use crate::{
		options::{DisplayConfig, DisplayConfigConstant, DisplaySettings, Text},
		threshold_map::ThresholdMap,
		with_date, without_date,
	};

	fn date_year_month_day(year: i32, month: u32, day: u32) -> DateTime<Utc> {
		NaiveDate::from_ymd_opt(year, month, day)
			.unwrap()
			.and_time(NaiveTime::default())
			.and_utc()
	}

	#[test]
	fn two_minutes() {
		assert_eq!(
			without_date(
				Duration::minutes(2),
				DisplayConfigConstant::default(),
				Text::default(),
			),
			Ok(String::from("2 minutes"))
		);
	}
	#[test]
	fn standard_a() {
		assert_eq!(
			without_date(
				Duration::seconds(500),
				DisplayConfigConstant::default(),
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
				DisplayConfigConstant::default(),
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
				DisplayConfigConstant::default(),
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
				DisplayConfigConstant::default(),
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
				DisplayConfigConstant::default(),
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
				DisplayConfigConstant::default(),
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
				DisplayConfigConstant::default(),
				Text::default(),
			),
			Ok(String::from("578 days, 16 hours and 53 minutes"))
		);
	}

	fn config_weeks_seconds() -> DisplayConfigConstant {
		DisplayConfigConstant::none().with_weeks().with_seconds()
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

	fn config_weeks_minutes_seconds() -> DisplayConfigConstant {
		DisplayConfigConstant {
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
				Text::default(),
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
				Text::default(),
			),
			Ok(String::from(
				"1 year, 6 months, 29 days, 16 hours and 53 minutes"
			)),
		)
	}

	#[test]
	fn months_only() {
		assert_eq!(
			with_date(
				Duration::seconds(50_000_000),
				date_year_month_day(2020, 1, 1),
				DisplayConfig::none().with_months(),
				Text::default(),
			),
			Ok(String::from("19 months"))
		)
	}
	#[test]
	fn months_only_neg() {
		assert_eq!(
			with_date(
				Duration::seconds(-50_000_000),
				date_year_month_day(2020, 1, 1),
				DisplayConfig::none().with_months(),
				Text::default(),
			),
			Ok(String::from("19 months"))
		)
	}
	#[test]
	fn month_just_before_threshold() {
		assert_eq!(
			with_date(
				Duration::days(14) - Duration::seconds(1),
				date_year_month_day(2001, 2, 1),
				DisplayConfig::none().with_months(),
				Text::default(),
			),
			Ok(String::from("0 months"))
		)
	}
	#[test]
	fn month_just_on_threshold() {
		assert_eq!(
			with_date(
				Duration::days(14),
				date_year_month_day(2001, 2, 1),
				DisplayConfig::none().with_months(),
				Text::default(),
			),
			Ok(String::from("1 month"))
		)
	}
	#[test]
	fn month_just_after_threshold() {
		assert_eq!(
			with_date(
				Duration::days(14) + Duration::seconds(1),
				date_year_month_day(2001, 2, 1),
				DisplayConfig::none().with_months(),
				Text::default(),
			),
			Ok(String::from("1 month"))
		)
	}
	#[test]
	fn year_just_before_threshold() {
		assert_eq!(
			with_date(
				Duration::days(366 / 2) - Duration::seconds(1),
				date_year_month_day(2000, 1, 1),
				DisplayConfig::none().with_years(),
				Text::default(),
			),
			Ok(String::from("0 years"))
		)
	}
	#[test]
	fn year_just_on_threshold() {
		assert_eq!(
			with_date(
				Duration::days(366 / 2),
				date_year_month_day(2000, 1, 1),
				DisplayConfig::none().with_years(),
				Text::default(),
			),
			Ok(String::from("1 year"))
		)
	}
	#[test]
	fn year_just_after_threshold() {
		assert_eq!(
			with_date(
				Duration::days(366 / 2) + Duration::seconds(1),
				date_year_month_day(2000, 1, 1),
				DisplayConfig::none().with_years(),
				Text::default(),
			),
			Ok(String::from("1 year"))
		)
	}

	fn config_with_zeroes() -> DisplayConfig {
		DisplayConfig {
			years: Some(DisplaySettings::new(0.., 0, true)),
			months: Some(DisplaySettings::new(0.., 0, true)),
			weeks: None,
			days: Some(DisplaySettings::new(0.., 0, true)),
			hours: Some(DisplaySettings::new(0.., 0, true)),
			minutes: Some(DisplaySettings::new(0.., 0, true)),
			seconds: Some(DisplaySettings::new(0..600, 0, true)),
		}
	}
	#[test]
	fn with_zeroes() {
		assert_eq!(
			with_date(
				Duration::days(15),
				date_year_month_day(2020, 1, 1),
				config_with_zeroes(),
				Text::default(),
			),
			Ok(String::from(
				"0 years, 0 months, 15 days, 0 hours and 0 minutes"
			))
		)
	}
	#[test]
	fn with_zeroes_long() {
		assert_eq!(
			with_date(
				Duration::seconds(32918400),
				date_year_month_day(2020, 1, 1),
				config_with_zeroes(),
				Text::default(),
			),
			Ok(String::from(
				"1 year, 0 months, 15 days, 0 hours and 0 minutes"
			))
		)
	}

	fn config_clocklike() -> DisplayConfigConstant {
		DisplayConfigConstant {
			weeks: None,
			days: None,
			hours: Some(DisplaySettings::new(0.., 2, true)),
			minutes: Some(DisplaySettings::new(0.., 2, true)),
			seconds: Some(DisplaySettings::new(0.., 2, true)),
		}
	}
	fn text_clocklike() -> Text {
		let mut text = Text::default();
		text.hours = ThresholdMap::single_value("");
		text.minutes = ThresholdMap::single_value("");
		text.seconds = ThresholdMap::single_value("");
		text.spacer = String::from("");
		text.joiner = String::from(":");
		text.final_joiner = None;
		text
	}
	#[test]
	fn clocklike() {
		assert_eq!(
			without_date(
				Duration::days(3) + Duration::hours(2) + Duration::seconds(1),
				config_clocklike(),
				text_clocklike(),
			),
			Ok(String::from("74:00:01"))
		)
	}
	#[test]
	fn clocklike_minute() {
		assert_eq!(
			without_date(Duration::minutes(1), config_clocklike(), text_clocklike(),),
			Ok(String::from("00:01:00"))
		)
	}

	#[test]
	fn zero_duration() {
		assert_eq!(
			without_date(
				Duration::zero(),
				DisplayConfigConstant::default(),
				Text::default()
			),
			Ok(String::from("0 seconds"))
		)
	}
	#[test]
	fn zero_duration_months() {
		assert_eq!(
			with_date(
				Duration::zero(),
				date_year_month_day(2000, 1, 1),
				DisplayConfig::none().with_months(),
				Text::default()
			),
			Ok(String::from("0 months"))
		)
	}
	#[test]
	fn zero_duration_years() {
		assert_eq!(
			with_date(
				Duration::zero(),
				date_year_month_day(2000, 1, 1),
				DisplayConfig::none().with_years(),
				Text::default()
			),
			Ok(String::from("0 years"))
		)
	}

	#[test]
	fn readme_examples() {
		let text = crate::without_date(
			chrono::Duration::seconds(1_234_567),
			DisplayConfigConstant::default(),
			Text::default(),
		);
		assert_eq!(text, Ok(String::from("14 days, 6 hours and 56 minutes")));
	}
}
