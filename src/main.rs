extern crate clap;

use clap::{value_parser, Arg, Command};
use eval::{eval, to_value};

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

// fn permute(data: &mut Vec<usize>, start: usize, result: &mut Vec<Vec<usize>>) {
//     let length = data.len();
//     if start == length - 1 {
//         result.push(data.clone());
//     }
//     for i in start..length {
//         data.swap(start, i);
//         permute(data, start + 1, result);
//         data.swap(start, i);
//     }
// }

fn permute(data: &mut Vec<usize>, start: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let length = data.len();
    if start == length - 1 {
        result.push(data.clone());
        return result;
    }
    for i in start..length {
        data.swap(start, i);
        let mut r = permute(data, start + 1);
        result.append(&mut r);
        data.swap(start, i);
    }
    result
}

fn split_vec(data: &[usize], start: usize, current: Vec<Vec<usize>>) -> Vec<Vec<Vec<usize>>> {
    let mut result = Vec::new();

    if start == data.len() {
        result.push(current.clone());
        return result;
    }

    // Extend existing sub-vectors
    for i in 0..current.len() {
        let mut new_current = current.clone();
        new_current[i].push(data[start]);
        let mut sub_splits = split_vec(data, start + 1, new_current);
        result.append(&mut sub_splits);
    }

    // Create a new sub-vector
    let mut new_current = current.clone();
    new_current.push(vec![data[start]]);
    let mut sub_splits = split_vec(data, start + 1, new_current);
    result.append(&mut sub_splits);

    result
}
fn main() {
    let args = parse_args();

    let mut numbers = args.numbers.to_vec();
    let result = permute(&mut numbers, 0);
    for p in result {
        for _ in 1..p.len() {
            let result = split_vec(&p, 0, Vec::new());
            for j in result {
                println!("{:?}", j);
            }
        }
    }

    // sort order
    // split point
    // operand

    // 1, 2, 3
    // () () ()
    // () ()
    // () ()

    // 1,2,3
    // (1),2,3
    //
}

#[cfg(test)]
mod tests {
    // #[derive(Debug, PartialEq)]
    // enum Operator {
    //     Undefined,
    //     Add,
    //     Sub,
    //     Mult,
    //     Div,
    // }

    // struct Expression {
    //     operator: Operator,
    //     expresssion: Option<Box<Expression>>,
    //     numbers: Vec<usize>,
    // }

    // struct Expr {
    //     expr: Option<Box<Vec<Expr>>>,
    //     number: Option<usize>,
    // }

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

    // fn split(data: Vec<usize>) -> Vec<Expression> {
    //     let mut result = Vec::new();

    //     for i in 1..data.len() + 1 {
    //         if data.len() % i == 0 {
    //             let row = data.to_vec();
    //             result.push(Expression {
    //                 operator: Operator::undefined,
    //                 expresssion: None,
    //                 numbers: row,
    //             });
    //         }
    //     }

    //     result
    // }

    fn number(number: usize) -> Box<Expression> {
        Box::new(Expression::Number(number))
    }
    fn paren(a: Box<Expression>, b: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::Paren(a, b))
    }

    fn split(data: Vec<usize>) -> Vec<Box<Expression>> {
        // let mut result = Vec::new();

        match data.len() {
            1 => vec![number(data[0])],
            2 => vec![paren(number(data[0]), number(data[1]))],
            3 => vec![paren(
                paren(number(data[0]), number(data[1])),
                number(data[2]),
            )],
            4 => vec![paren(
                paren(number(data[0]), number(data[1])),
                paren(number(data[2]), number(data[3])),
            )],
            _ => panic!("Unexpected data length"),
        }

        // // [11] => 1
        // // [11, 22] => 1,2
        // // [11, 22, 33] => 1,2,3
        // // data.len() // 3
        // // [11,22,33,44] => 1,2,3,4
        // // data.len() // 4
        // for i in 1..data.len() + 1 {
        //     for ii in 0..i {
        //         let number = Box::new(Expression::Number(data[ii]));
        //         result.push(Box::new(Expression::Paren(number)));
        //     }
        // }

        // result
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
    fn test_split() {
        let vec = [1];
        let actual = split(vec.to_vec());
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], number(1));

        let vec = [1, 2];
        let actual = split(vec.to_vec());
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], paren(number(1), number(2)));

        let vec = [1, 2, 3];
        let actual = split(vec.to_vec());
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], paren(paren(number(1), number(2)), number(3)));

        let vec = [1, 2, 3, 4];
        let actual = split(vec.to_vec());
        assert_eq!(actual.len(), 1);
        assert_eq!(
            actual[0],
            paren(paren(number(1), number(2)), paren(number(3), number(4)))
        );

        // let vec = [1, 2];
        // let actual = split(vec.to_vec());
        // assert_eq!(actual.len(), 2);
        // assert_eq!(
        //     actual[1],
        //     Box::new(Expression::Paren(Box::new(Expression::Number(1))))
        // );
        // assert_eq!(
        //     actual[1],
        //     Box::new(Expression::Paren(ox::new(Expression::Number(1))))
        // )

        // let vec = [1, 2];
        // let actual = split(vec.to_vec());

        // let vec = [1, 2, 3];
        // let actual = split(vec.to_vec());
        // assert_eq!(actual.len(), 3);
        // assert!(actual[0].expr.is_none());
        // assert!(actual[1].expr.is_none());
        // assert!(actual[2].expr.is_none());
        // assert_eq!(actual[0].number, Some(1));
        // assert_eq!(actual[1].number, Some(2));
        // assert_eq!(actual[2].number, Some(3));
    }
}
