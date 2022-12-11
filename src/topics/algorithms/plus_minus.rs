// https://www.hackerrank.com/challenges/plus-minus/problem?isFullScreen=true

#![allow(unused_imports)]
use std::io::{self, BufRead};

/*
 * Complete the 'plus_minus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plus_minus(arr: &[i32]) {
    let mut result: [i32; 4] = [0, 0, 0, arr.len() as i32];
    
    for i in arr {
        if *i > 0 {
            result[0] += 1;
        } else if *i < 0 {
            result[1] += 1;
        } else {
            result[2] += 1;
        }
    }

    println!("{:.6}\n{:.6}\n{:.6}", 
        (result[0] as f64 / result[3] as f64), 
        (result[1] as f64 / result[3] as f64), 
        (result[2] as f64 / result[3] as f64)
    );
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plus_minus(&arr);
}
