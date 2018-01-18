#[macro_use]
extern crate clap;
extern crate chrono;

use std::fs::File;
use std::io::Read;
use chrono::prelude::*;
use clap::App;

struct Entry {
    blocked_hours: Vec<(DateTime<Utc>, DateTime<Utc>)>
}

// #rhctrl M:9-18,22-00 T:9-18,22-00 W:9-18,22-00 R:9-18,22-00 F:9-18,22-00 S:allow U:allow

impl Entry {
    fn parse(line: &str) -> Result<Entry, ()> {
        if line.contains("rhctrl") {
            let mut blocked_hours = Vec::new();

            for v in Entry::parse_blocked_hours(line) {
                blocked_hours.push(v);
            }

            Ok( Entry {
                blocked_hours: blocked_hours
            }
            )
        } else {
            Err(())
        }
    }

    fn parse_blocked_hours(line: &str) -> Vec<(DateTime<Utc>, DateTime<Utc>)> {
        let line = line.clone();
        let dt = Local::now();
        let letter = match dt.weekday() {
            Weekday::Fri => "F",
            _ => "M"
        };
        let prefix = format!("{}:", letter);
        let line: Vec<&str> = line.split(&prefix).collect();
        let line: &str = line.get(1).unwrap_or(&"");
        let line: Vec<&str> = line.split(" ").collect();
        let hours: Vec<&str> = line.first().unwrap_or(&"").split(",").collect();

        let mut list = Vec::new();

        list.push((Utc::now(), Utc::now()));

        list
    }

    fn is_blocked(&self) -> bool {
        for &(ref start_blocking, ref end_blocking) in &self.blocked_hours {
            return true
        }

        false
    }
}

fn format_line<'a>(entry: &Entry, line: &'a mut String) -> String {
    if entry.is_blocked() && !line.starts_with("#") {
        return format!("# {}", line);
    }

    if !entry.is_blocked() && line.starts_with("#") {
        line.remove(0);
        return line.to_string();
    }

    return line.to_string();
}

fn read_hosts_file() -> String {
    let mut data = String::new();
    let mut f = File::open("/etc/hosts").expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");

    data
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let _matches = App::from_yaml(yaml).get_matches();
    let mut lines: Vec<String> = Vec::new();

    for line in read_hosts_file().lines() {
        let mut line = line.to_string();

        match Entry::parse(&line) {
            Ok(entry) => {
                lines.push(format_line(&entry, &mut line).trim().to_string());
                println!("{}", lines.last().unwrap_or(&"missing".to_string()));
            },
            Err(_) => {
                lines.push(line.to_string());
            }
        }
    }
}
