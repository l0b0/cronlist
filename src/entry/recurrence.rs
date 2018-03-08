use super::date_time_field_parser::DateTimeFieldParser;

const MINUTES_PARSER: DateTimeFieldParser = DateTimeFieldParser { min: 0, max: 59 };
const HOURS_PARSER: DateTimeFieldParser = DateTimeFieldParser { min: 0, max: 23 };
const DAYS_OF_MONTH_PARSER: DateTimeFieldParser = DateTimeFieldParser { min: 1, max: 31 };
const MONTHS_PARSER: DateTimeFieldParser = DateTimeFieldParser { min: 1, max: 12 };
const DAYS_OF_WEEK_PARSER: DateTimeFieldParser = DateTimeFieldParser { min: 0, max: 7 };

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
        Recurrence {
            minutes: MINUTES_PARSER.parse(fields[0]),
            hours: HOURS_PARSER.parse(fields[1]),
            days_of_month: DAYS_OF_MONTH_PARSER.parse(fields[2]),
            months: MONTHS_PARSER.parse(fields[3]),
            days_of_week: DAYS_OF_WEEK_PARSER.parse(fields[4]),
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
