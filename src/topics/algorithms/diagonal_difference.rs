// https://www.hackerrank.com/challenges/diagonal-difference/problem?isFullScreen=true

#![allow(unused_imports)]
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'diagonal_difference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let mut result: i32 = 0;
    for i in 0..arr.len() {
        result += arr[i][i];
        result -= arr[i][arr.len() - i - 1];
    }

    result.abs()
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create("output.txt").unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));

        arr[i] = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = diagonal_difference(&arr);
    println!("{}", result);
    
    writeln!(&mut fptr, "{}", result).ok();
}
