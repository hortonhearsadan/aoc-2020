use aoc_2020::{get_input_as_int, get_input_as_int_set};
use itertools::{cons_tuples, Itertools};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let input = get_input_as_int_set("1");

    let start = Instant::now();

    let f = run1_alt(&input);
    let g = run2_alt(&input);

    let t = start.elapsed().as_micros();

    println!("Part 1: {}", f);
    println!("Part 2: {}", g);
    println!("Duration: {:.3}us", t);
}

fn run1_alt(numbers: &HashSet<i64>) -> i64 {
    let mut part = 0;
    for v in numbers {
        let u = 2020 - v;
        if numbers.contains(&u) {
            part = u * v
        }
    }
    part
}

fn run2_alt(numbers: &HashSet<i64>) -> i64 {
    let mut part = 0;
    // for some reason this is faster than combinations
    for i in numbers {
        for j in numbers {
            if i == j {
                continue;
            }
            let v = *i + *j;
            if v >= 2020 {
                continue;
            }
            let u = 2020 - v;
            if numbers.contains(&u) {
                part = *i * *j * u;
            }
        }
    }
    // for values in numbers.iter().combinations(2) {
    //     let (i, j) = values.iter().next_tuple().unwrap();
    //     let v = *i + *j;
    //     if v >= 2020 {
    //         continue;
    //     }
    //     let u = 2020 - v;
    //     if numbers.contains(&u) {
    //         part = *i * *j * u;
    //     }
    // }
    part
}
