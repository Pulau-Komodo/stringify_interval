Generates a user-friendly string from a `chrono::Duration`, like "1 day, 5 hours and 20 minutes".

Years and months can be displayed, but they will need some date as a reference point, because the exact length of a year or month can vary.

It is fairly configurable.

## Usage

```rs
let text = stringify_interval::without_date(
	chrono::Duration::seconds(1_234_567),
	DisplayConfigConstant::default(),
	Text::default(),
);
assert_eq!(text, Ok(String::from("14 days, 6 hours and 56 minutes")));
```

## Configuration

Each individual unit can have a range of values set for when it should show up. This allows it to do things like automatically drop seconds for long durations, or not mention years unless the interval includes 5 of them. When units are dropped, the interval is rounded to the nearest multiple of the smallest unit still displayed.

Each individual unit can also be padded with zeroes, or be set to display even when the value is 0.

Additionally, all the string elements can be changed out with the `Text` struct. This allows for formatting changes and for some degree of localisation. For each unit, a `ThresholdMap` allows setting for which number range which text should be displayed.

The default values for `Text` are as follows:

```rs
Text {
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
```