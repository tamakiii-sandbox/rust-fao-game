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
    fn permute(data: Vec<usize>, start: usize) -> Vec<Vec<usize>> {
        let mut result: Vec<Vec<usize>> = Vec::new();
        for d in data {
            let row: Vec<usize> = vec![d];
            result.push(row)
        }
        result
    }

    #[test]
    fn test() {
        let vec = [1, 2, 3];
        assert_eq!(permute(vec.to_vec(), 0), [[1], [2], [3],]);

        // assert_eq!(
        //     permute(vec.to_vec(), 0),
        //     [
        //         [1, 2, 3],
        //         [1, 3, 2],
        //         [2, 1, 3],
        //         [2, 3, 1],
        //         [3, 1, 2],
        //         [3, 2, 1],
        //     ]
        // );
    }
}
