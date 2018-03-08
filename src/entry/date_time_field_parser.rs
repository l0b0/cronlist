use std::ops::Range;

pub struct DateTimeFieldParser {
    range: Range<u8>,
}

impl DateTimeFieldParser {
    pub fn new(min: u8, max: u8) -> DateTimeFieldParser {
        DateTimeFieldParser { range: (min..max + 1) }
    }

    pub fn parse(&self, string_value: &str) -> Vec<u8> {
        let mut values = Vec::with_capacity((self.range.end - self.range.start) as usize);

        string_value
            .split(",")
            .for_each(|part| values.append(&mut self.parse_part(part)));

        values.sort_unstable();
        values.dedup();

        values
    }

    fn parse_part(&self, string_value: &str) -> Vec<u8> {
        let values = match string_value.parse::<u8>() {
            Ok(value) => vec![value],
            Err(_) => self.parse_range(string_value).collect(),
        };
        values.iter().for_each(|value| self.verify_range(*value));
        values
    }

    fn parse_range(&self, string_value: &str) -> Range<u8> {
        let values: Vec<u8> = string_value.splitn(2, "-").map(|part| part.parse().unwrap()).collect();

        // TODO: Use inclusive range when stable
        values[0]..values[1] + 1
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
        assert_eq!(parser.parse("4-8,1,12,6"), vec![1, 4, 5, 6, 7, 8, 12]);
    }

    #[test]
    fn should_parse_comma_separated_numbers() {
        let parser = DateTimeFieldParser::new(0, 23);
        assert_eq!(parser.parse("0,23"), vec![0, 23]);
    }

    #[test]
    fn should_remove_duplicates() {
        let parser = DateTimeFieldParser::new(1, 2);
        assert_eq!(parser.parse("1,1,2,2,2"), vec![1, 2]);
    }

    #[test]
    fn should_sort_values() {
        let parser = DateTimeFieldParser::new(1, 2);
        assert_eq!(parser.parse("2,1"), vec![1, 2]);
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
