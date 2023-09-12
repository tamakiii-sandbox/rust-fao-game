extern crate clap;

use clap::{value_parser, Arg, Command};

fn main() {
    let matches = Command::new("fao")
        .version("0.1")
        .arg(
            Arg::new("expected")
                .long("expected")
                .required(true)
                .value_parser(value_parser!(usize)),
        )
        .arg(Arg::new("numbers").long("numbers").required(true))
        .get_matches();

    let expected = matches.get_one::<usize>("expected").unwrap();
    let numbers_str = matches.get_one::<String>("numbers").unwrap();
    let numbers: Vec<usize> = numbers_str
        .split(',')
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();

    println!("Expected: {}", expected);
    println!("Numbers: {:?}", numbers);
}
