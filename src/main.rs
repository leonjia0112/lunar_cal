extern crate clap;
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

}

