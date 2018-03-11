use super::date_time_field_parser::DateTimeFieldParser;
use chrono::prelude::*;
use time::Duration;

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
            minutes: minutes_parser.parse_field(fields[0]),
            hours: hours_parser.parse_field(fields[1]),
            days_of_month: days_of_month_parser.parse_field(fields[2]),
            months: months_parser.parse_field(fields[3]),
            days_of_week: days_of_week_parser.parse_field(fields[4]),
        }
    }

    fn next(&self, after: NaiveDateTime) -> NaiveDateTime {
        let mut current = after + Duration::minutes(1); // Avoid matching `after`
        while !self.matches(current) {
            current += Duration::minutes(1);
        }
        current
    }

    fn matches(&self, instant: NaiveDateTime) -> bool {
        let minute = &(instant.minute() as u8);
        let hour = &(instant.hour() as u8);
        let day_of_month = &(instant.day() as u8);
        let month = &(instant.month() as u8);
        let day_of_week = &(instant.weekday() as u8);
        self.minutes.contains(minute) && self.hours.contains(hour) && self.days_of_month.contains(day_of_month)
            && self.months.contains(month) && self.days_of_week.contains(day_of_week)
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

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

    #[test]
    fn should_get_next_occurrence() {
        let mut recurrence = Recurrence {
            minutes: vec![0],
            hours: vec![0],
            days_of_month: vec![1],
            months: vec![1],
            days_of_week: vec![0],
        };
        let matching_datetime = NaiveDate::from_ymd(2001, 1, 1).and_hms(0, 0, 0);
        let now = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        assert_eq!(recurrence.next(now), matching_datetime);
    }

    #[test]
    fn should_match_occurrences() {
        let mut recurrence = Recurrence {
            minutes: vec![0],
            hours: vec![0],
            days_of_month: vec![1],
            months: vec![1],
            days_of_week: vec![0],
        };
        let matching_datetime = NaiveDate::from_ymd(2001, 1, 1).and_hms(0, 0, 0);
        assert!(recurrence.matches(matching_datetime));
    }

    #[test]
    fn should_not_match_other_date() {
        let mut recurrence = Recurrence {
            minutes: vec![0],
            hours: vec![0],
            days_of_month: vec![1],
            months: vec![1],
            days_of_week: vec![0],
        };
        let mismatch = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        assert!(!recurrence.matches(mismatch));
    }
}
