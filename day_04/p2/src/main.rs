extern crate chrono;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use chrono::prelude::*;
use chrono::Duration;
// use chrono::offset::LocalResult;

#[derive(Debug)]
struct Log {
    datetime: DateTime<Utc>,
    action: String,
}

impl Log {
    fn from(line: String) -> Log {
        let mut line_data = line.split("] ");
        let mut d_str = String::from(line_data.next().unwrap());
        d_str.remove(0);
        // let datetime = DateTime::parse_from_str("1987-05-04 14:10:00", "%Y-%m-%d %H:%M").unwrap().with_timezone(&Utc);
        let nd = NaiveDateTime::parse_from_str(&d_str, "%Y-%m-%d %H:%M").unwrap();
        // println!("{:?}", nd);
        let datetime = DateTime::from_utc(nd, Utc);
        // println!("{:?}", datetime);
        Log { datetime, action: String::from(line_data.next().unwrap()) }
    }

    fn begin_shift(&self) -> Option<i32> {
        if self.action.contains("begins shift") {
            let mut d = self.action.split_whitespace();
            d.next();
            let mut id_str = String::from(d.next().unwrap());
            id_str.remove(0);
            Some(id_str.parse::<i32>().unwrap())
        } else {
            None
        }
    }

    fn falls_asleep(&self) -> bool {
        // self.action.contains("falls asleep")
        self.action == "falls asleep"
    }

    fn wakes_up(&self) -> bool {
        // self.action.contains("wakes up")
        self.action == "wakes up"
    }
}

fn print_flags(flags: Vec<bool>) {
    for f in flags {
        let v = if f { '.' } else { '#' };
        print!("{}", v);
    }
}

fn println_flags(flags: Vec<bool>) {
    print_flags(flags);
    println!("");
}

fn slept(date_flags: &mut HashMap<Date<Utc>, Vec<bool>>, from: DateTime<Utc>, to: DateTime<Utc>) {
    // print_flags(flags.to_vec());
    // println!("{:?} {:?}", from, to);
    for m in 0..(to - from).num_minutes() {
        let t = from + Duration::minutes(m);
        // println!("{:?}", t);
        if t.hour() == 0 {
            let d = t.date();
            let flags = date_flags.entry(d).or_insert(vec![true; 60]);
            flags[t.minute() as usize] = false;
        }
    }
    // print_flags(flags.to_vec());
}

fn main() {
    let file = File::open("../input").unwrap();
    // let file = File::open("../example").unwrap();
    let reader = BufReader::new(file);
    let mut logs: Vec<Log> = vec![];

    for line in reader.lines() {
        logs.push(Log::from(line.unwrap()));
    }
    logs.sort_by(|a, b| a.datetime.cmp(&b.datetime));
    // for log in logs.iter() {
    //     println!("{:?}", log);
    // }
    let mut guard_flags: HashMap<i32, HashMap<Date<Utc>, Vec<bool>>> = HashMap::new();
    {
        let mut guard_id = -1;
        // let mut waking_up = true;
        let mut prev_time = logs[0].datetime;
        for log in logs.iter() {
            match log.begin_shift() {
                Some(id) => {
                    guard_id = id;
                    // waking_up = true;
                    prev_time = log.datetime;
                    continue;
                },
                None => {},
            }
            if guard_id == -1 { continue; }
            let mut wakeup_flags = guard_flags.entry(guard_id).or_insert(HashMap::new());
            if log.falls_asleep() {
                prev_time = log.datetime;
                // waking_up = false;
            }
            if log.wakes_up() {
                slept(wakeup_flags, prev_time, log.datetime);
                prev_time = log.datetime;
                // waking_up = true;
            }
        }
    }
    let mut most_slept_guard_id = -1;
    let mut most_slept_guard_slept_minutes = 0;
    let mut most_slept_guard_most_slept_minute = 0;
    let mut most_frequent_slept_guard_id = -1;
    let mut most_frequent_slept_minute = 0;
    let mut most_frequent_slept_count = 0;
    for (&guard_id, date_flags) in &guard_flags {
        let mut keep_sleep_counts = vec![0; 60];
        let mut slept_minutes = 0;
        for (date, flags) in date_flags {
            match flags.binary_search(&true) {
                Ok(_) =>  {
                    print_flags(flags.to_vec());
                    println!(" {} {:?}", guard_id, date);
                },
                Err(_) => {},
            }
            for (i, &waking_up) in flags.iter().enumerate() {
                if ! waking_up {
                    keep_sleep_counts[i] += 1;
                    slept_minutes += 1;
                }
            }
        }
        // print_flags(keep_sleep_flags.to_vec());
        println!(" total {} minutes", slept_minutes);
        let mut max_count = 0;
        let mut most_slept_minute = 0;
        for (i, &c) in keep_sleep_counts.iter().enumerate() {
            if c > max_count {
                max_count = c;
                most_slept_minute = i
            }
        }
        if slept_minutes > most_slept_guard_slept_minutes {
            most_slept_guard_id = guard_id;
            most_slept_guard_slept_minutes = slept_minutes;
            most_slept_guard_most_slept_minute = most_slept_minute;
        }
        if max_count > most_frequent_slept_count {
            most_frequent_slept_guard_id = guard_id;
            most_frequent_slept_count = max_count;
            most_frequent_slept_minute = most_slept_minute;
        }
    }
    println!("Part1 result: {} * {} = {}",
             most_slept_guard_id,
             most_slept_guard_most_slept_minute,
             most_slept_guard_id * most_slept_guard_most_slept_minute as i32);
    println!("Part2 result: {} * {} = {}",
             most_frequent_slept_guard_id,
             most_frequent_slept_minute,
             most_frequent_slept_guard_id * most_frequent_slept_minute as i32);
}
