extern crate clap;

use clap::{value_parser, Arg, Command};
// use eval::{eval, to_value};

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

    // let mut numbers = args.numbers.to_vec();
    println!("expected: {}", args.expected);
    println!("numbers: {:?}", args.numbers);
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    #[derive(Debug, Clone, PartialEq)]
    enum Expression {
        Number(usize),
        // Undefined(Box<Expression>, Box<Expression>),
        // Add(Box<Expression>, Box<Expression>),
        // Subtract(Box<Expression>, Box<Expression>),
        // Multiply(Box<Expression>, Box<Expression>),
        // Divide(Box<Expression>, Box<Expression>),
        Paren(Box<Expression>, Box<Expression>),
    }

    fn merge(current: usize, vec: Vec<usize>) -> Vec<usize> {
        let mut result = vec![current];
        for v in vec {
            result.push(v);
        }
        result
    }

    fn permute(data: Vec<usize>) -> Vec<Vec<usize>> {
        if data.len() == 1 {
            return vec![data];
        }

        let mut result: Vec<Vec<usize>> = Vec::new();

        for i in 0..data.len() {
            let mut vec = data.to_vec();
            let current = vec[i];
            vec.swap_remove(i);
            for rest in permute(vec.clone()) {
                let row = merge(current, rest);
                result.push(row)
            }
        }

        result
    }

    fn number(number: usize) -> Box<Expression> {
        Box::new(Expression::Number(number))
    }
    fn paren(a: Box<Expression>, b: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::Paren(a, b))
    }

    fn convert(data: Vec<usize>) -> Vec<Box<Expression>> {
        match data.len() {
            1 => vec![number(data[0])],
            2 => vec![paren(number(data[0]), number(data[1]))],
            3 => vec![
                paren(paren(number(data[0]), number(data[1])), number(data[2])),
                paren(number(data[0]), paren(number(data[1]), number(data[2]))),
            ],
            4 => vec![
                paren(
                    paren(number(data[0]), number(data[1])),
                    paren(number(data[2]), number(data[3])),
                ),
                paren(
                    paren(paren(number(data[0]), number(data[1])), number(data[2])),
                    number(data[3]),
                ),
                paren(
                    paren(number(data[0]), paren(number(data[1]), number(data[2]))),
                    number(data[3]),
                ),
                paren(
                    number(data[0]),
                    paren(paren(number(data[1]), number(data[2])), number(data[3])),
                ),
                paren(
                    number(data[0]),
                    paren(number(data[1]), paren(number(data[2]), number(data[3]))),
                ),
            ],
            _ => panic!("Unexpected data length"),
        }
    }

    #[test]
    fn test_permute() {
        let vec = [1, 2, 3];
        assert_eq!(
            permute(vec.to_vec()),
            [
                [1, 3, 2],
                [1, 2, 3],
                [2, 1, 3],
                [2, 3, 1],
                [3, 1, 2],
                [3, 2, 1]
            ]
        );

        let vec = [1, 2, 3, 4];
        assert_eq!(
            permute(vec.to_vec()),
            [
                [1, 4, 3, 2],
                [1, 4, 2, 3],
                [1, 2, 4, 3],
                [1, 2, 3, 4],
                [1, 3, 4, 2],
                [1, 3, 2, 4],
                [2, 1, 3, 4],
                [2, 1, 4, 3],
                [2, 4, 1, 3],
                [2, 4, 3, 1],
                [2, 3, 1, 4],
                [2, 3, 4, 1],
                [3, 1, 4, 2],
                [3, 1, 2, 4],
                [3, 2, 1, 4],
                [3, 2, 4, 1],
                [3, 4, 1, 2],
                [3, 4, 2, 1],
                [4, 1, 3, 2],
                [4, 1, 2, 3],
                [4, 2, 1, 3],
                [4, 2, 3, 1],
                [4, 3, 1, 2],
                [4, 3, 2, 1]
            ]
        );
    }

    #[test]
    fn test_convert() {
        let vec = [1];
        let actual = convert(vec.to_vec());
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], number(1));

        let vec = [1, 2];
        let actual = convert(vec.to_vec());
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], paren(number(1), number(2)));

        let vec = [1, 2, 3];
        let actual = convert(vec.to_vec());
        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0], paren(paren(number(1), number(2)), number(3)));
        assert_eq!(actual[1], paren(number(1), paren(number(2), number(3))));

        let vec = [1, 2, 3, 4];
        let actual = convert(vec.to_vec());
        assert_eq!(actual.len(), 5);
        assert_eq!(
            actual[0],
            paren(paren(number(1), number(2)), paren(number(3), number(4))),
        );
        assert_eq!(
            actual[1],
            paren(paren(paren(number(1), number(2)), number(3)), number(4)),
        );
        assert_eq!(
            actual[2],
            paren(paren(number(1), paren(number(2), number(3))), number(4)),
        );
        assert_eq!(
            actual[3],
            paren(number(1), paren(paren(number(2), number(3)), number(4)))
        );
        assert_eq!(
            actual[4],
            paren(number(1), paren(number(2), paren(number(3), number(4))))
        );
    }
}
