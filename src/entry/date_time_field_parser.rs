use std::ops::Range;

use entry::stepped_range::SteppedRange;

pub struct DateTimeFieldParser {
    range: Range<u8>,
}

impl DateTimeFieldParser {
    pub fn new(min: u8, max: u8) -> DateTimeFieldParser {
        DateTimeFieldParser { range: (min..max + 1) }
    }

    pub fn parse_field(&self, string_value: &str) -> Vec<u8> {
        let mut values = Vec::with_capacity((self.range.end - self.range.start) as usize);

        string_value
            .split(",")
            .for_each(|part| values.append(&mut self.parse_list_entry(part)));

        values.sort_unstable();
        values.dedup();

        values
    }

    fn parse_list_entry(&self, string_value: &str) -> Vec<u8> {
        let mut parts = string_value.splitn(2, '/');

        let values: Range<u8> = self.parse_range(parts.next().unwrap());

        let step = match parts.next() {
            Some(string_value) => string_value.parse::<u8>().unwrap(),
            None => 1,
        };

        // TODO: Use step_by when stable
        SteppedRange {
            start: values.start,
            end: values.end,
            step: step,
        }.collect()
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
        assert_eq!(parser.parse_field("5-9/2,1,12,6"), vec![1, 5, 6, 7, 9, 12]);
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
