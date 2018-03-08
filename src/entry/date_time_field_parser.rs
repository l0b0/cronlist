pub struct DateTimeFieldParser {
    pub min: u8,
    pub max: u8,
}

impl DateTimeFieldParser {
    pub fn parse(&self, string_value: &str) -> Vec<u8> {
        let mut values = Vec::with_capacity((self.max - self.min + 1) as usize);

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
            Err(_) => self.parse_range(string_value),
        };
        values.iter().for_each(|value| self.verify_range(*value));
        values
    }

    fn parse_range(&self, string_value: &str) -> Vec<u8> {
        let values: Vec<u8> = string_value
            .splitn(2, "-")
            .map(|part| part.parse().ok().unwrap())
            .collect();

        { values[0]..(values[1] + 1) }.collect()
    }

    fn verify_range(&self, value: u8) {
        let range = { self.min..self.max + 1 };
        assert!(range.start <= value);
        assert!(range.end > value);
    }
}

#[cfg(test)]
mod tests {
    use super::DateTimeFieldParser;

    #[test]
    fn should_parse_complex_pattern() {
        let parser = DateTimeFieldParser { min: 1, max: 12 };
        assert_eq!(parser.parse("4-8,1,12,6"), vec![1, 4, 5, 6, 7, 8, 12]);
    }

    #[test]
    fn should_parse_comma_separated_numbers() {
        let parser = DateTimeFieldParser { min: 0, max: 23 };
        assert_eq!(parser.parse("0,23"), vec![0, 23]);
    }

    #[test]
    fn should_remove_duplicates() {
        let parser = DateTimeFieldParser { min: 1, max: 2 };
        assert_eq!(parser.parse("1,1,2,2,2"), vec![1, 2]);
    }

    #[test]
    fn should_sort_values() {
        let parser = DateTimeFieldParser { min: 1, max: 2 };
        assert_eq!(parser.parse("2,1"), vec![1, 2]);
    }

    #[test]
    fn should_parse_range() {
        let parser = DateTimeFieldParser { min: 0, max: 23 };
        assert_eq!(parser.parse_range("1-3"), vec![1, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn should_fail_verification_below_min() {
        let parser = DateTimeFieldParser { min: 1, max: 12 };
        parser.verify_range(0)
    }

    #[test]
    fn should_verify_at_min() {
        let parser = DateTimeFieldParser { min: 0, max: 23 };
        parser.verify_range(0);
    }

    #[test]
    fn should_verify_between_min_and_max() {
        let parser = DateTimeFieldParser { min: 0, max: 23 };
        parser.verify_range(12);
    }

    #[test]
    fn should_verify_at_both_min_and_max() {
        let parser = DateTimeFieldParser { min: 1, max: 1 };
        parser.verify_range(1);
    }

    #[test]
    fn should_verify_at_max() {
        let parser = DateTimeFieldParser { min: 0, max: 23 };
        parser.verify_range(23);
    }

    #[test]
    #[should_panic]
    fn should_fail_verification_above_max() {
        let parser = DateTimeFieldParser { min: 0, max: 23 };
        parser.verify_range(24);
    }
}
