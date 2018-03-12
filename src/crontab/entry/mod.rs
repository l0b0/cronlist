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
        let trimmed = entry.trim_left();
        match trimmed.chars().next().unwrap() {
            '@' => Entry::split_with_datetime_nickname(trimmed),
            _ => Entry::splitn_whitespace(trimmed, 6),
        }
    }

    fn split_with_datetime_nickname(entry: &str) -> Vec<&str> {
        let split = Entry::splitn_whitespace(entry, 2);
        let mut fields = match split[0] {
            "@yearly" | "@annually" => vec!["0", "0", "1", "1", "*"],
            "@monthly" => vec!["0", "0", "1", "*", "*"],
            "@weekly" => vec!["0", "0", "*", "*", "0"],
            "@daily" => vec!["0", "0", "*", "*", "*"],
            "@hourly" => vec!["0", "*", "*", "*", "*"],
            value => panic!("Unhandled datetime nickname ‘{}’", value),
        };
        fields.push(split[1]);
        fields
    }

    fn splitn_whitespace(entry: &str, max_entries: usize) -> Vec<&str> {
        let mut last_whitespace = false;
        entry
            .splitn(max_entries, |character: char| {
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

    #[test]
    fn should_handle_datetime_nicknames() {
        let actual = Entry::fields("@yearly   command  with  spaces ");
        assert_eq!(actual, vec!["0", "0", "1", "1", "*", "command  with  spaces "]);
    }

    #[test]
    #[should_panic(expected = "Unhandled datetime nickname ‘@reboot’")]
    fn should_fail_on_reboot_nickname() {
        Entry::fields("@reboot command");
    }
}
