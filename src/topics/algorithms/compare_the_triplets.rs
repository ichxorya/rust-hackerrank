// https://www.hackerrank.com/challenges/compare-the-triplets/problem?isFullScreen=true

#![allow(unused_imports)]
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'compare_triplets' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn compare_triplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0, 0];

    for it in a.iter().zip(b.iter()) {
        let (_ai, _bi) = it;
        if _ai > _bi {
            result[0] += 1;
        } else if _ai < _bi {
            result[1] += 1;
        }
    }

    result
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create("output.txt").unwrap();

    let a: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let b: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = compare_triplets(&a, &b);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    println!("{:#?}", result);

    writeln!(&mut fptr).ok();
}
