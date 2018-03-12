mod entry;

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
                .filter(|line| !line.is_empty())
                .map(|line| Entry::new(line))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Crontab;
    use crontab::entry::Entry;

    #[test]
    fn should_create_single_entry() {
        let actual = Crontab::new("1 2 3 4 5 command");
        assert_eq!(actual.entries.len(), 1);
        assert_eq!(actual.entries[0].command, "command");
    }

    #[test]
    fn should_create_multiple_entries() {
        let actual = Crontab::new(
            "1 2 3 4 5 first
2 3 4 5 6 second
",
        );
        assert_eq!(actual.entries.len(), 2);
        assert_eq!(actual.entries[0].command, "first");
        assert_eq!(actual.entries[1].command, "second");
    }

    #[test]
    fn should_ignore_empty_lines() {
        let actual = Crontab::new(
            "1 2 3 4 5 first

2 3 4 5 6 second
",
        );
        assert_eq!(actual.entries.len(), 2);
        assert_eq!(actual.entries[0].command, "first");
        assert_eq!(actual.entries[1].command, "second");
    }
}
