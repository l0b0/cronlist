mod entry;

use chrono::NaiveDateTime;
use crontab::entry::Entry;

pub struct Crontab<'a> {
    entries: Vec<Entry<'a>>,
}

impl<'a> Crontab<'a> {
    fn new(input: &str) -> Crontab {
        Crontab {
            entries: input
                .lines()
                .map(|line| line.trim_left())
                .filter(|line| !line.starts_with('#'))
                .filter(|line| !line.is_empty())
                .map(|line| Entry::new(line))
                .collect(),
        }
    }

    fn next_run(&self, from: NaiveDateTime) -> Run {
        self.entries
            .iter()
            .map(|entry| Run {
                entry: &entry,
                datetime: entry.recurrence.next_match(from),
            })
            .min_by(|this, other| this.datetime.cmp(&other.datetime))
            .unwrap()
    }
}

struct Run<'a> {
    entry: &'a Entry<'a>,
    datetime: NaiveDateTime,
}

#[cfg(test)]
mod tests {
    use super::Crontab;
    use chrono::NaiveDate;
    use chrono::NaiveDateTime;
    use crontab::entry::Entry;

    #[test]
    fn should_create_single_entry() {
        let actual = Crontab::new("1 2 3 4 5 command");
        assert_eq!(actual.entries.len(), 1);
        assert_eq!(actual.entries[0].command, "command");
    }

    #[test]
    fn should_create_multiple_entries() {
        let crontab = ["1 2 3 4 5 first", "2 3 4 5 6 second"].join("\n");
        let actual = Crontab::new(&crontab);
        assert_eq!(actual.entries.len(), 2);
        assert_eq!(actual.entries[0].command, "first");
        assert_eq!(actual.entries[1].command, "second");
    }

    #[test]
    fn should_ignore_empty_lines() {
        let crontab = ["", " \t \t", "1 2 3 4 5 command", " ", ""].join("\n");
        let actual = Crontab::new(&crontab);
        assert_eq!(actual.entries.len(), 1);
        assert_eq!(actual.entries[0].command, "command");
    }

    #[test]
    fn should_ignore_comment_lines() {
        let crontab = ["# Comment", "1 2 3 4 5 command", " \t \t# Comment"].join("\n");
        let actual = Crontab::new(&crontab);
        assert_eq!(actual.entries.len(), 1);
        assert_eq!(actual.entries[0].command, "command");
    }

    #[test]
    fn should_get_next_run() {
        let crontab = ["0 * * * * first", "30 * * * * second"].join("\n");
        let crontab = Crontab::new(&crontab);
        let next_run = crontab.next_run(NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 10, 0));
        assert_eq!(next_run.entry.command, "second");
        assert_eq!(next_run.datetime, NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 30, 0));
    }
}
