use super::date_time_field_parser::DateTimeFieldParser;

#[derive(Debug, PartialEq)]
pub struct Recurrence {
    minutes: Vec<u8>,
    hours: Vec<u8>,
    days_of_month: Vec<u8>,
    months: Vec<u8>,
    days_of_week: Vec<u8>,
}

impl Recurrence {
    pub fn new(fields: &[&str]) -> Recurrence {
        let minutes_parser: DateTimeFieldParser = DateTimeFieldParser::new(0, 59);
        let hours_parser: DateTimeFieldParser = DateTimeFieldParser::new(0, 23);
        let days_of_month_parser: DateTimeFieldParser = DateTimeFieldParser::new(1, 31);
        let months_parser: DateTimeFieldParser = DateTimeFieldParser::new(1, 12);
        let days_of_week_parser: DateTimeFieldParser = DateTimeFieldParser::new(0, 7);

        Recurrence {
            minutes: minutes_parser.parse(fields[0]),
            hours: hours_parser.parse(fields[1]),
            days_of_month: days_of_month_parser.parse(fields[2]),
            months: months_parser.parse(fields[3]),
            days_of_week: days_of_week_parser.parse(fields[4]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Recurrence;

    #[test]
    fn should_construct_a_recurrence_from_parser_responses() {
        let recurrence = Recurrence::new(&vec!["1", "2", "3", "4", "5"]);
        assert_eq!(
            recurrence,
            Recurrence {
                minutes: vec![1],
                hours: vec![2],
                days_of_month: vec![3],
                months: vec![4],
                days_of_week: vec![5],
            }
        );
    }
}
