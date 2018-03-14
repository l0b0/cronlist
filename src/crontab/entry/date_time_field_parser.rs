use std::ops::Range;

use crontab::entry::stepped_range::SteppedRange;

pub struct DateTimeFieldParser {
    range: Range<u8>,
    wrap_around_at_end: bool,
}

impl DateTimeFieldParser {
    pub fn new(min: u8, max: u8) -> DateTimeFieldParser {
        DateTimeFieldParser {
            range: min..max + 1,
            wrap_around_at_end: false,
        }
    }

    pub fn new_with_wrap_around(min: u8, max: u8) -> DateTimeFieldParser {
        DateTimeFieldParser {
            range: min..max + 1,
            wrap_around_at_end: true,
        }
    }

    pub fn parse_field(&self, string_value: &str) -> Vec<u8> {
        let mut values = Vec::with_capacity((self.range.end - self.range.start) as usize);

        string_value
            .split(',')
            .for_each(|part| values.append(&mut self.parse_list_entry(part)));

        values.sort_unstable();
        values.dedup();

        values
    }

    fn parse_list_entry(&self, string_value: &str) -> Vec<u8> {
        let mut parts = string_value.splitn(2, '/');
        let values = parts
            .next()
            .unwrap()
            .replace("*", &format!("{}-{}", self.range.start, self.range.end - 1));

        let values = match values.to_lowercase().as_ref() {
            "sun" => "0",
            "jan" | "mon" => "1",
            "feb" | "tue" => "2",
            "mar" | "wed" => "3",
            "apr" | "thu" => "4",
            "may" | "fri" => "5",
            "jun" | "sat" => "6",
            "jul" => "7",
            "aug" => "8",
            "sep" => "9",
            "oct" => "10",
            "nov" => "11",
            "dec" => "12",
            _ => &values,
        };

        let values: Range<u8> = self.parse_range(values);

        let step = match parts.next() {
            Some(string_value) => string_value.parse::<u8>().unwrap(),
            None => 1,
        };

        // TODO: Use step_by when stable
        let mut values: Vec<u8> = SteppedRange::new(values.start, values.end, step).collect();

        let last_value = values.pop().unwrap();
        if last_value == self.range.end && self.wrap_around_at_end {
            values.push(0);
        } else {
            values.push(last_value);
        }
        values
    }

    fn parse_range(&self, values: &str) -> Range<u8> {
        let mut range_or_value = values.splitn(2, '-').map(|part| part.parse().unwrap());
        let first = range_or_value.next().expect("Empty range");
        let last = match range_or_value.next() {
            Some(value) => value,
            None => first,
        };

        // TODO: Use inclusive range when stable
        first..last + 1
    }

    fn verify_range(&self, value: u8) {
        assert!(self.range.start <= value);
        assert!(self.range.end > value);
    }
}

#[cfg(test)]
mod tests {
    use super::DateTimeFieldParser;

    #[test]
    fn should_parse_complex_pattern() {
        let parser = DateTimeFieldParser::new(1, 12);
        assert_eq!(parser.parse_field("5-9/2,1,*/5"), vec![1, 5, 6, 7, 9, 11]);
    }

    #[test]
    fn should_parse_comma_separated_numbers() {
        let parser = DateTimeFieldParser::new(0, 23);
        assert_eq!(parser.parse_field("0,23"), vec![0, 23]);
    }

    #[test]
    fn should_parse_range_with_step() {
        let parser = DateTimeFieldParser::new(0, 23);
        assert_eq!(parser.parse_list_entry("1-7/2"), vec![1, 3, 5, 7]);
    }

    #[test]
    fn should_parse_asterisk() {
        let parser = DateTimeFieldParser::new(1, 12);
        assert_eq!(parser.parse_list_entry("*/4"), vec![1, 5, 9]);
    }

    #[test]
    fn should_parse_january_name() {
        let parser = DateTimeFieldParser::new(1, 12);
        assert_eq!(parser.parse_list_entry("Jan"), vec![1]);
    }

    #[test]
    fn should_parse_february_name() {
        let parser = DateTimeFieldParser::new(1, 12);
        assert_eq!(parser.parse_list_entry("Feb"), vec![2]);
    }

    #[test]
    fn should_parse_march_name() {
        let parser = DateTimeFieldParser::new(1, 12);
        assert_eq!(parser.parse_list_entry("Mar"), vec![3]);
    }

    #[test]
    fn should_parse_april_name() {
        let parser = DateTimeFieldParser::new(1, 12);
        assert_eq!(parser.parse_list_entry("Apr"), vec![4]);
    }

    #[test]
    fn should_parse_may_name() {
        let parser = DateTimeFieldParser::new(1, 12);
        assert_eq!(parser.parse_list_entry("May"), vec![5]);
    }

    #[test]
    fn should_parse_june_name() {
        let parser = DateTimeFieldParser::new(1, 12);
        assert_eq!(parser.parse_list_entry("Jun"), vec![6]);
    }

    #[test]
    fn should_parse_july_name() {
        let parser = DateTimeFieldParser::new(1, 12);
        assert_eq!(parser.parse_list_entry("Jul"), vec![7]);
    }

    #[test]
    fn should_parse_august_name() {
        let parser = DateTimeFieldParser::new(1, 12);
        assert_eq!(parser.parse_list_entry("Aug"), vec![8]);
    }

    #[test]
    fn should_parse_september_name() {
        let parser = DateTimeFieldParser::new(1, 12);
        assert_eq!(parser.parse_list_entry("Sep"), vec![9]);
    }

    #[test]
    fn should_parse_october_name() {
        let parser = DateTimeFieldParser::new(1, 12);
        assert_eq!(parser.parse_list_entry("Oct"), vec![10]);
    }

    #[test]
    fn should_parse_november_name() {
        let parser = DateTimeFieldParser::new(1, 12);
        assert_eq!(parser.parse_list_entry("Nov"), vec![11]);
    }

    #[test]
    fn should_parse_december_name() {
        let parser = DateTimeFieldParser::new(1, 12);
        assert_eq!(parser.parse_list_entry("Dec"), vec![12]);
    }

    #[test]
    fn should_parse_month_name_case_insensitively() {
        let parser = DateTimeFieldParser::new(1, 12);
        assert_eq!(parser.parse_list_entry("dEC"), vec![12]);
    }

    #[test]
    fn should_parse_sunday_name() {
        let parser = DateTimeFieldParser::new_with_wrap_around(0, 6);
        assert_eq!(parser.parse_list_entry("Sun"), vec![0]);
    }

    #[test]
    fn should_parse_monday_name() {
        let parser = DateTimeFieldParser::new_with_wrap_around(0, 6);
        assert_eq!(parser.parse_list_entry("Mon"), vec![1]);
    }

    #[test]
    fn should_parse_tuesday_name() {
        let parser = DateTimeFieldParser::new_with_wrap_around(0, 6);
        assert_eq!(parser.parse_list_entry("Tue"), vec![2]);
    }

    #[test]
    fn should_parse_wednesday_name() {
        let parser = DateTimeFieldParser::new_with_wrap_around(0, 6);
        assert_eq!(parser.parse_list_entry("Wed"), vec![3]);
    }

    #[test]
    fn should_parse_thursday_name() {
        let parser = DateTimeFieldParser::new_with_wrap_around(0, 6);
        assert_eq!(parser.parse_list_entry("Thu"), vec![4]);
    }

    #[test]
    fn should_parse_friday_name() {
        let parser = DateTimeFieldParser::new_with_wrap_around(0, 6);
        assert_eq!(parser.parse_list_entry("Fri"), vec![5]);
    }

    #[test]
    fn should_parse_saturday_name() {
        let parser = DateTimeFieldParser::new_with_wrap_around(0, 6);
        assert_eq!(parser.parse_list_entry("Sat"), vec![6]);
    }

    #[test]
    fn should_parse_week_day_name_case_insensitively() {
        let parser = DateTimeFieldParser::new_with_wrap_around(0, 6);
        assert_eq!(parser.parse_list_entry("sAT"), vec![6]);
    }

    #[test]
    fn should_remove_duplicates() {
        let parser = DateTimeFieldParser::new(1, 2);
        assert_eq!(parser.parse_field("1,1,2,2,2"), vec![1, 2]);
    }

    #[test]
    fn should_sort_values() {
        let parser = DateTimeFieldParser::new(1, 2);
        assert_eq!(parser.parse_field("2,1"), vec![1, 2]);
    }

    #[test]
    fn should_parse_wraparound_sunday() {
        let parser = DateTimeFieldParser::new_with_wrap_around(0, 6);
        assert_eq!(parser.parse_field("7"), vec![0]);
    }

    #[test]
    fn should_parse_week_range_with_sunday_at_end() {
        let parser = DateTimeFieldParser::new_with_wrap_around(0, 6);
        assert_eq!(parser.parse_field("4-7"), vec![0, 4, 5, 6]);
    }

    #[test]
    fn should_parse_week_range_with_sunday_at_both_sides() {
        let parser = DateTimeFieldParser::new_with_wrap_around(0, 6);
        assert_eq!(parser.parse_field("0-7"), vec![0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn should_parse_range() {
        let parser = DateTimeFieldParser::new(0, 23);
        assert_eq!(parser.parse_range("1-3"), { 1..4 });
    }

    #[test]
    #[should_panic]
    fn should_fail_verification_below_min() {
        let parser = DateTimeFieldParser::new(1, 12);
        parser.verify_range(0)
    }

    #[test]
    fn should_verify_at_min() {
        let parser = DateTimeFieldParser::new(0, 23);
        parser.verify_range(0);
    }

    #[test]
    fn should_verify_between_min_and_max() {
        let parser = DateTimeFieldParser::new(0, 23);
        parser.verify_range(12);
    }

    #[test]
    fn should_verify_at_both_min_and_max() {
        let parser = DateTimeFieldParser::new(1, 1);
        parser.verify_range(1);
    }

    #[test]
    fn should_verify_at_max() {
        let parser = DateTimeFieldParser::new(0, 23);
        parser.verify_range(23);
    }

    #[test]
    #[should_panic]
    fn should_fail_verification_above_max() {
        let parser = DateTimeFieldParser::new(0, 23);
        parser.verify_range(24);
    }
}
