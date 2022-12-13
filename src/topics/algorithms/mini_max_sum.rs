// https://www.hackerrank.com/challenges/mini-max-sum/problem?isFullScreen=true

#![allow(unused_imports)]
use std::io::{self, BufRead};

/*
 * Complete the 'mini_max_sum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn mini_max_sum(arr: &[i32]) {
    let mut sum: i64 = 0;
    let mut max: i64 = i64::MIN;
    let mut min: i64 = i64::MAX;

    for i in arr {
        let n: i64 = *i as i64;
        sum += n;

        if min > n {
            min = n;
        }

        if max < n {
            max = n;
        }
    }

    println!("{} {}", sum - max, sum - min);
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    mini_max_sum(&arr);
}
