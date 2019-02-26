extern crate clap;
extern crate chrono;

use chrono::prelude::*;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Lunar Calender Calculator")
        .version("0.0.1")
        .author("Leo Jia <leojia@bu.edu>")
        .about("Getting the lunar date from normal date.")
        .arg(Arg::with_name("today's date")
             .short("t")
             .long("today")
             .help("Getting today's lunar date."))
        .get_matches();

    if matches.is_present("today's date") {
        let (year, date) = get_today();
        println!("Today is year {}, date {}.", year, date);
    }
}

fn get_today() -> (String, String) {
    let today = Local::now();
    let t_year = today.year();
    let t_month = today.month();
    let t_day = today.day();
    let t_date_str: String;
    
    if t_month < 10 && t_day < 10 {
        t_date_str = format!("0{}0{}", t_month, t_day);
    } else if t_month < 10 && t_day >= 10 {
        t_date_str = format!("0{}{}", t_month, t_day);
    } else if t_month >= 10 && t_day < 10 {
        t_date_str = format!("{}0{}", t_month, t_day);
    } else {
        t_date_str = format!("{}{}", t_month, t_day);
    }
    (t_year.to_string(), t_date_str)
}
