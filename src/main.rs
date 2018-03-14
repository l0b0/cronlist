extern crate chrono;
extern crate cronlist;

use chrono::Local;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read standard input");

    let crontab = cronlist::crontab::Crontab::new(&buffer);
    let next_run = crontab.next_run(Local::now().naive_local());

    println!("{} {}", next_run.datetime, next_run.entry.command);
}
