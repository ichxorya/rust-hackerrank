// https://www.hackerrank.com/challenges/2d-array/problem?isFullScreen=true
#![allow(unused_imports)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'hourglass_sum' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn hourglass_sum(arr: &[Vec<i32>]) -> i32 {
    let mut max_hourglass_sum: i32 = i32::MIN;
    for i in 1..=4 {
        for j in 1..=4 {
            let top_sum: i32 = arr[i - 1][j - 1] + arr[i - 1][j] + arr[i - 1][j + 1];
            let bot_sum: i32 = arr[i + 1][j - 1] + arr[i + 1][j] + arr[i + 1][j + 1];
            let sum: i32 = top_sum + arr[i][j] + bot_sum;
            if max_hourglass_sum < sum {
                max_hourglass_sum = sum;
            }
        }
    }
    max_hourglass_sum
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create("output.txt").unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(6_usize);

    for i in 0..6_usize {
        arr.push(Vec::with_capacity(6_usize));

        arr[i] = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = hourglass_sum(&arr);
    println!("{}", result);

    writeln!(&mut fptr, "{}", result).ok();
}
