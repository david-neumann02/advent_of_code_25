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
    let operation_vec: Vec<Operation> = input.last().unwrap().split_whitespace()
    .map(|s|
        match s {
            "+" => Operation::Addition,
            "*" => Operation::Multiplication,
            _ => panic!("Unknown Operation")
        }
    ).collect();

    todo!()
}