extern crate chrono;
extern crate clap;
extern crate serde_json;

use chrono::prelude::*;
use clap::{App, Arg};
use serde_json::value::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde_json::map::Map;

fn main() {
    let matches = App::new("Lunar Calender Calculator")
        .version("0.0.1")
        .author("Leo Jia <leojia@bu.edu>")
        .about("Getting the lunar date from normal date.")
        .arg(
            Arg::with_name("today's date")
                .short("t")
                .long("today")
                .help("Getting today's lunar date."),
        )
        .get_matches();

    if matches.is_present("today's date") {
        let(year, month, day) = get_today(); 
        println!("{}", year);
        let lunar_date = find_lunar_date(year, month, day).unwrap();
        println!("Today is {}", lunar_date);
    }
}

fn find_lunar_date(year: i32, month: i32, day: i32) -> Option<String> {
    let file = File::open(Path::new("data/date_map.json")).unwrap();
    let reader = BufReader::new(file);
    let date_map: Map<String, Value> = serde_json::from_reader(reader).unwrap().as_object().unwrap();

    println!("{:#?}", date_map);
    if let None = date_map.get(year as usize) {
        println!("Can find year in data.");
        return None;
    }

    let mut sun_date: i32 = date_to_nth_date(&year, &month, &day);
    let mut lunar_date: i32 = date_map[year as usize][0].as_i64().unwrap() as i32;

    if sun_date < lunar_date && date_map.get((year-1) as usize) == None {
        println!("last year data missing");
        return None;
    }

    if lunar_date >= sun_date {
        Some(map_to_lunar_date(year, lunar_date, sun_date, date_map))
    } else {
        if (year - 1 % 4 == 0 && year - 1 % 100 != 0) || year - 1 % 400 == 0 {
            sun_date += 366;
        } else {
            sun_date += 365;
        }
        lunar_date = date_map[(year - 1) as usize][0].as_i64().unwrap() as i32;
        Some(map_to_lunar_date(year - 1, lunar_date, sun_date, date_map))
    }
}

fn map_to_lunar_date(year: i32, mut lunar_date: i32, sun_date: i32, date_map: Value) -> String {
    let lunar_month_len: String = date_map[year as usize][1].to_string();
    let mut lunar_month = 1;
    for i in 0..lunar_month_len.len() {
        let temp_month_len = match &lunar_month_len[i..i + 1] {
            "1" => 30,
            "0" => 29,
            "-" => {
                lunar_month -= 1;
                continue;
            }
            _ => return String::from("Error"),
        };

        if lunar_date + temp_month_len > sun_date {
            break;
        }

        lunar_date += temp_month_len;
        lunar_month += 1;
    }
    format!("Lunar date: {}/{}", lunar_month, sun_date - lunar_date + 1)
}
fn date_to_nth_date(year: &i32, month: &i32, day: &i32) -> i32 {
    let mon_vec = match (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        true => vec![31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
        false => vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    };

    let mut nth_date: i32 = 0;
    for i in 0..*month - 1 {
        nth_date += mon_vec[i as usize];
    }
    nth_date + *day
}

fn get_today() -> (i32, i32, i32) {
    let today = Local::now();
    let t_year = today.year();
    let t_month = today.month();
    let t_day = today.day();
    (t_year, t_month as i32, t_day as i32)
}
