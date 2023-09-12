extern crate clap;

use clap::{builder::EnumValueParser, value_parser, Arg, Command};

struct Arguments {
    expected: usize,
    numbers: Vec<usize>,
}

fn get_command() -> Command {
    Command::new("fao")
        .version("0.1")
        .arg(
            Arg::new("expected")
                .long("expected")
                .required(true)
                .value_parser(value_parser!(usize)),
        )
        .arg(Arg::new("numbers").long("numbers").required(true))
}

fn get_arguments(expected: &usize, numbers: &String) -> Arguments {
    Arguments {
        expected: *expected,
        numbers: numbers.split(',').filter_map(|n| n.parse().ok()).collect(),
    }
}

fn parse_args() -> Arguments {
    let matches = get_command().get_matches();

    get_arguments(
        matches.get_one::<usize>("expected").unwrap(),
        matches.get_one::<String>("numbers").unwrap(),
    )
}

fn main() {
    let args = parse_args();
    println!("Expected: {}", args.expected);
    println!("Numbers: {:?}", args.numbers);
}
