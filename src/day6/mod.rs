use core::num;
use std::ops::Range;

use itertools::Itertools;

pub(crate) fn day6(input: &Vec<&str>) -> (u64, u64) {
    (part1(input), part2(input))
}

enum Operation {
    Addition,
    Multiplication
}


fn part1(input: &Vec<&str>) -> u64 {
    let operation_vec: Vec<Operation> = input.last().unwrap().split_whitespace()
    .map(|s|
        match s {
            "+" => Operation::Addition,
            "*" => Operation::Multiplication,
            _ => panic!("Unknown Operation")
        }
    ).collect();

    let num_vecs: Vec<Vec<u64>> = input.iter().take(input.len() - 1)
        .map(|s| s.split_whitespace().map(|s| s.parse().unwrap()).collect()).collect();

    let mut total_sum = 0;
    for (i, op) in operation_vec.iter().enumerate() {
        let line_iter = num_vecs.iter().map(|v| v[i]);
        total_sum += match op {
            Operation::Addition => line_iter.sum::<u64>(),
            Operation::Multiplication => line_iter.product()
        }
    }
    total_sum
}

fn part2(input: &Vec<&str>) -> u64 {
    let mut number_regions = Vec::new();
    let mut operation_vec = Vec::new();

    let mut last_index = 0;
    for (i, c) in input.last().unwrap().chars().enumerate().skip(1) {
        if c != ' ' {
            number_regions.push(last_index..i - 1);
            last_index = i;
            operation_vec.push(match c {
                '+' => Operation::Addition,
                '*' => Operation::Multiplication,
                _ => panic!("Unknwon symbol")
            });
        }
    }

    let digit_count: u32 = (input.len() - 1) as u32;

    let range_mapper = |r: Range<usize>| -> Vec<u64> {
        r.map(|i| 
            input.iter().take(input.len() - 1).enumerate()
                .map(|(j, n)| 
                    n.chars().nth(i).unwrap()
                        .to_digit(10).unwrap_or(0) as u64 * 10u64.pow(digit_count - (j + 1) as u32)
                ).sum()
        ).collect()
    };

    let num_vecs: Vec<Vec<u64>> = number_regions.into_iter()
        .map(range_mapper).collect();

    num_vecs.iter().zip(operation_vec.iter()).map(|(v, o)| match o {
        Operation::Addition => v.iter().sum::<u64>(),
        Operation::Multiplication => v.iter().product()
    }).sum()

}