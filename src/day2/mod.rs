use std::ops::RangeInclusive;

pub(crate) fn day2(input_lines: &Vec<&str>) -> u64 {
    let intervals: Vec<RangeInclusive<u64>> = input_lines
        .iter()
        .next()
        .unwrap()
        .split(",")
        .map(|s| {
            let (s1, s2) = s.split_once("-").unwrap();
            let (n1, n2): (u64, u64) = (s1.parse().unwrap(), s2.parse().unwrap());
            n1..=n2
        })
        .collect();

    intervals.iter().map(|r| sum_doubles(r)).sum()
}

fn sum_doubles(interval: &RangeInclusive<u64>) -> u64 {
    let mut n1 = *interval.start();
    let mut n2 = *interval.end();

    let l1 = n1.to_string().len();
    let l2 = n2.to_string().len();

    let l1mod = l1 % 2;

    if l1 == l2 && l1mod == 1 {
        return 0;
    }

    let d1 = l1.div_ceil(2);
    if l1mod == 1 {
        n1 = 10u64.pow((2 * d1 - 1) as u32)
    }

    let front_number: u64 = n1.to_string().get(0..d1).unwrap().parse().unwrap();

    let d2 = l2 / 2;
    if l1mod == 1 {
        n2 = 10u64.pow((2 * d2) as u32) - 1
    }

    let back_number: u64 = n2.to_string().get(0..d2).unwrap().parse().unwrap();

    println!("l1: {l1}, l2: {l2}, n1: {n1}, n2: {n2}, d1: {d1}, d2: {d2}");

    (front_number..=back_number)
        .map(|current_number| {
            let doubled_number = (current_number.to_string() + &current_number.to_string())
                .parse()
                .unwrap();
            if interval.contains(&doubled_number) {
                doubled_number
            } else {
                0
            }
        })
        .sum()

    /*
    let mut sum = 0;
    let mut current_number: u64 = n1.to_string().get(0..d).unwrap().parse().unwrap();
    let mut current_number_doubled: u64 = (current_number.to_string() + &current_number.to_string()).parse().unwrap();
    println!("l1:{l1}, l2: {l2}, n1: {n1}, n2: {n2}, d: {d}, current_number: {current_number}");
    while interval.contains(&current_number_doubled){
        sum += current_number_doubled;
        println!("{current_number_doubled}");
        current_number += 1;
        current_number_doubled = (current_number.to_string() + &current_number.to_string()).parse().unwrap();
    };
    26262335584
    31070077552
    31110183885
    */
}
