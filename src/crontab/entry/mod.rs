mod date_time_field_parser;
mod recurrence;
mod stepped_range;

use self::recurrence::Recurrence;

pub struct Entry<'a> {
    recurrence: Recurrence,
    pub command: &'a str,
}

impl<'a> Entry<'a> {
    pub fn new(entry: &str) -> Entry {
        let fields = Entry::fields(entry);

        Entry {
            recurrence: Recurrence::new(&fields[0..5]),
            command: fields[5],
        }
    }

    fn fields(entry: &'a str) -> Vec<&'a str> {
        let mut last_whitespace = false;
        entry
            .trim_left()
            .splitn(6, |character: char| {
                if character.is_whitespace() {
                    if last_whitespace {
                        return false;
                    }
                    last_whitespace = true;
                    true
                } else {
                    last_whitespace = false;
                    false
                }
            })
            .map(str::trim_left)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Entry;

    #[test]
    fn should_create_entry_with_command() {
        let actual = Entry::new("1 2 3 4 5 command");
        assert_eq!(actual.command, "command");
    }

    #[test]
    fn should_split_into_six_fields() {
        let actual = Entry::fields("  1  2   3 4 5   command  with   spaces  ");
        assert_eq!(actual, vec!["1", "2", "3", "4", "5", "command  with   spaces  "]);
    }
}
