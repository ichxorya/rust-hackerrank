// https://www.hackerrank.com/challenges/birthday-cake-candles/problem?isFullScreen=true

#![allow(unused_imports)]
use std::convert::TryInto;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'birthday_cake_candles' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY candles as parameter.
 */

fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let highest: &i32 = candles.iter().max().unwrap();
    candles
        .iter()
        .filter(|&candle| candle == highest)
        .count()
        .try_into()
        .unwrap()
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create("output.txt").unwrap();

    let _candles_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let candles: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = birthday_cake_candles(&candles);
    println!("{}", result);

    writeln!(&mut fptr, "{}", result).ok();
}
