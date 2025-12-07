use std::{collections::HashSet, ops::RangeInclusive};
use itertools::Itertools;


pub(crate) fn day5(input: &[&str]) -> (u64, u64) {
    let (ingredient_range_strs, ingredient_strs) = input.split_at(input.iter().position(|l| l == &"").unwrap());

    let ingredient_range_vec: Vec<RangeInclusive<u64>> = ingredient_range_strs.iter()
        .map(|line| {
            let (num1, num2) = line.split_once('-').expect("Tried to split range without '-'-Delimiter");

            num1.parse().unwrap()..=num2.parse().unwrap()
        }).collect();

    let ingredient_vec: Vec<u64> = ingredient_strs.iter().skip(1).map(|l| l.parse().unwrap()).collect();

    (part1(&ingredient_range_vec, &ingredient_vec), part2(ingredient_range_vec))
}

fn part1(ingredient_range_vec: &Vec<RangeInclusive<u64>>, ingredient_vec: &Vec<u64>) -> u64 {
    ingredient_vec.iter().filter(
        |ingredient| ingredient_range_vec.iter()
            .find(
                |range| range.contains(&ingredient)
            ).is_some()
        ).count() as u64
}

fn part2(ingredient_range_vec: Vec<RangeInclusive<u64>>) -> u64 {
    ingredient_range_vec.into_iter()
        .map(|range| range.into_iter()).flatten().counts().len() as u64
}