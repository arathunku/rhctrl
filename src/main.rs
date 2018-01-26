extern crate chrono;
#[macro_use]
extern crate clap;
extern crate regex;

// #rhctrl M:9-18,22-00 T:9-18,22-00 W:9-18,22-00 R:9-18,22-00 F:9-18,22-00 S:allow U:allow
//
use std::fs::File;
use std::thread;
use std::io::Read;
use chrono::prelude::*;
use chrono::Duration;
use clap::App;
use regex::{Captures, Regex};

struct Entry {
    blocked_hours: Vec<(DateTime<Local>, DateTime<Local>)>,
}

impl Entry {
    fn parse(line: &str) -> Result<Entry, ()> {
        if line.contains("rhctrl") {
            let mut blocked_hours = Vec::new();

            for v in Entry::parse_blocked_hours(line) {
                blocked_hours.push(v);
            }

            Ok(Entry {
                blocked_hours: blocked_hours,
            })
        } else {
            Err(())
        }
    }

    fn parse_blocked_hours(line: &str) -> Vec<(DateTime<Local>, DateTime<Local>)> {
        let line = line.clone();
        let dt = Local::now();
        let letter = match dt.weekday() {
            Weekday::Mon => "M",
            Weekday::Tue => "T",
            Weekday::Wed => "W",
            Weekday::Thu => "R",
            Weekday::Fri => "F",
            Weekday::Sat => "F",
            Weekday::Sun => "U",
        };
        let prefix = format!("{}:", letter);
        let line: Vec<&str> = line.split(&prefix).collect();
        let line: &str = line.get(1).unwrap_or(&"");
        let line: Vec<&str> = line.split(" ").collect();
        let hour_ranges: Vec<&str> = line.first().unwrap_or(&"").split(",").collect();
        let mut list = Vec::new();

        for hours in hour_ranges {
            match hours {
                "allow" => {
                    // no range -> no blocking
                }
                "block" => {
                    list.push((
                        Local::now().with_hour(0).unwrap(),
                        Local::now() + Duration::days(1),
                    ));
                }
                _ => {
                    list.push(Entry::parse_hour_range(hours));
                }
            }
        }

        list
    }

    fn is_blocked(&self) -> bool {
        let now = Local::now();
        for &(ref start_blocking, ref end_blocking) in &self.blocked_hours {
            if start_blocking < &now && end_blocking > &now {
                return true;
            }
        }

        false
    }

    fn parse_hour_range(hours: &str) -> (DateTime<Local>, DateTime<Local>) {
        let re = Regex::new(
            r"(?P<start_hour>\d+):*(?P<start_min>\d*)-(?P<end_hour>\d+)(:*)(?P<end_min>\d*)",
        ).unwrap();

        let parse = |captures: &Captures, name| {
            captures
                .name(name)
                .map_or("0", |v| v.as_str())
                .parse()
                .unwrap_or(0)
        };
        let captures = re.captures(hours).unwrap();
        let start_hour: u32 = parse(&captures, "start_hour");
        let start_min: u32 = parse(&captures, "start_min");
        let end_hour: u32 = parse(&captures, "end_hour");
        let end_min: u32 = parse(&captures, "end_min");

        let start = Local::now()
            .with_hour(start_hour)
            .unwrap()
            .with_minute(start_min)
            .unwrap();
        let end = Local::now()
            .with_hour(end_hour)
            .unwrap()
            .with_minute(end_min)
            .unwrap();

        (start, end)
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

fn build_hosts_file(hosts: &str) -> String {
    let mut lines: Vec<String> = Vec::new();
    for line in hosts.lines() {
        let mut line = line.to_string();

        match Entry::parse(&line) {
            Ok(entry) => {
                lines.push(format_line(&entry, &mut line).trim().to_string());
            }
            Err(_) => {
                lines.push(line.to_string());
            }
        }
    }

    lines.join("\n")
}

fn read_src_file(src: &str) -> String {
    let mut data = String::new();
    let mut f = File::open(src).expect("Unable to open hosts file. File missing?");
    f.read_to_string(&mut data)
        .expect("Unable to read hosts file");

    data
}

fn write_dst_file(dst: &str, content: &str) {
    use std::io::prelude::*;
    let mut f = File::create(dst).expect("Unable to open hosts file");
    f.write_all(content.as_bytes())
        .expect("Unable to write hosts");
}

fn run(matches: &clap::ArgMatches) {
    let hosts = match matches.value_of("source") {
        Some(src) => read_src_file(src),
        None => read_src_file("/etc/hosts"),
    };

    match matches.value_of("destination") {
        Some(dst) => {
            write_dst_file(dst, &build_hosts_file(&hosts));
        }
        None => {
            println!("{}", build_hosts_file(&hosts));
        }
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.value_of("interval") {
        Some(minutes) => loop {
            run(&matches);
            thread::sleep(Duration::minutes(
                minutes.parse::<i64>().expect("cannot parse interval"),
            ).to_std().expect("Interval cannot be lower than 0!"));
        },
        None => {
            run(&matches);
        }
    }
}
