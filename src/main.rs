extern crate clap;
extern crate chrono;

use chrono::prelude::*;
use clap::{Arg, App, SubCommand};

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

    println!("config is presented: {}", matches.is_present("today's date"));

    let (year, mon, day): Box<(i32, u32, u32)> = get_today();
    println!("Today is year {}, mont {}, day {}.", year, mon, day);
}

fn get_today() -> Box<(i32, u32, u32)> {
    let today = Local::now();
    Box::new((today.year(), today.month0(), today.day0()))
}
