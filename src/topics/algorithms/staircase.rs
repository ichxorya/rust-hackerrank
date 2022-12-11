// https://www.hackerrank.com/challenges/staircase/problem?isFullScreen=true
#![allow(unused_imports)]
use std::io::{self, BufRead};

/*
 * Complete the 'staircase' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn staircase(n: i32) {
    for i in 1..=n {
        let empty = (0..n - i).map(|_| " ").collect::<String>();
        let fill = (0..i).map(|_| "#").collect::<String>();
        // let mut empty = String::new();
        // empty.push_str(" ".repeat((n - i).try_into().unwrap()).as_str());
        // let mut fill = String::new();
        // fill.push_str("#".repeat(i.try_into().unwrap()).as_str());

        println!("{}", empty + &fill);
    }
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    staircase(n);
}
