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
        let days_of_week_parser: DateTimeFieldParser = DateTimeFieldParser::new_with_wrap_around(0, 6);

        Recurrence {
            minutes: minutes_parser.parse_field(fields[0]),
            hours: hours_parser.parse_field(fields[1]),
            days_of_month: days_of_month_parser.parse_field(fields[2]),
            months: months_parser.parse_field(fields[3]),
            days_of_week: days_of_week_parser.parse_field(fields[4]),
        }
    }

    pub fn next_match(&self, after: NaiveDateTime) -> NaiveDateTime {
        let next_minute = NextPeriod::new(&(after.minute() as u8 + 1), &self.minutes);
        let next_hour = NextPeriod::new(
            &(after.hour() as u8 + next_minute.overflow as u8),
            &self.hours,
        );
        let next_time_of_day = NaiveTime::from_hms(
            u32::from(next_hour.period),
            u32::from(next_minute.period),
            0,
        );
        let mut current = after.date().and_time(next_time_of_day) + Duration::days(i64::from(next_hour.overflow));

        while !self.matches(current) {
            current += Duration::days(1);
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

struct NextPeriod {
    period: u8,
    overflow: u8,
}

impl NextPeriod {
    fn new(after: &u8, possibilities: &[u8]) -> NextPeriod {
        for period in possibilities {
            if period >= after {
                return NextPeriod {
                    period: *period,
                    overflow: 0,
                };
            }
        }
        NextPeriod {
            period: possibilities[0],
            overflow: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;

    use super::{NextPeriod, Recurrence};

    const ANY_SECOND: u32 = 59;

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
    fn should_get_occurrence_next_minute() {
        let recurrence = Recurrence {
            minutes: vec![0, 1],
            hours: vec![0],
            days_of_month: vec![1],
            months: vec![1],
            days_of_week: vec![0, 1, 2, 3, 4, 5, 6],
        };
        let now = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, ANY_SECOND);
        assert_eq!(
            recurrence.next_match(now),
            NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 1, 0)
        );
    }

    #[test]
    fn should_get_occurrence_across_hour_boundary() {
        let recurrence = Recurrence {
            minutes: vec![0],
            hours: vec![1],
            days_of_month: vec![1],
            months: vec![1],
            days_of_week: vec![0, 1, 2, 3, 4, 5, 6],
        };
        let now = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 59, ANY_SECOND);
        assert_eq!(
            recurrence.next_match(now),
            NaiveDate::from_ymd(2000, 1, 1).and_hms(1, 0, 0)
        );
    }

    #[test]
    fn should_get_occurrence_across_day_boundary() {
        let recurrence = Recurrence {
            minutes: vec![0],
            hours: vec![0],
            days_of_month: vec![2],
            months: vec![1],
            days_of_week: vec![0, 1, 2, 3, 4, 5, 6],
        };
        let now = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, ANY_SECOND);
        assert_eq!(
            recurrence.next_match(now),
            NaiveDate::from_ymd(2000, 1, 2).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn should_get_occurrence_across_year_boundary() {
        let recurrence = Recurrence {
            minutes: vec![0],
            hours: vec![0],
            days_of_month: vec![1],
            months: vec![1],
            days_of_week: vec![0],
        };
        let now = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, ANY_SECOND);
        assert_eq!(
            recurrence.next_match(now),
            NaiveDate::from_ymd(2001, 1, 1).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn should_get_next_period_without_overflow() {
        let next = NextPeriod::new(&20, &vec![0, 30]);
        assert_eq!(next.period, 30);
        assert_eq!(next.overflow, 0);
    }

    #[test]
    fn should_get_next_period_with_overflow() {
        let next = NextPeriod::new(&46, &vec![15, 45]);
        assert_eq!(next.period, 15);
        assert_eq!(next.overflow, 1);
    }

    #[test]
    fn should_match_occurrences() {
        let recurrence = Recurrence {
            minutes: vec![0],
            hours: vec![0],
            days_of_month: vec![1],
            months: vec![1],
            days_of_week: vec![0],
        };
        assert!(recurrence.matches(NaiveDate::from_ymd(2001, 1, 1).and_hms(0, 0, ANY_SECOND)));
    }

    #[test]
    fn should_not_match_other_date() {
        let recurrence = Recurrence {
            minutes: vec![0],
            hours: vec![0],
            days_of_month: vec![1],
            months: vec![1],
            days_of_week: vec![0],
        };
        assert!(!recurrence.matches(NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, ANY_SECOND)));
    }
}
