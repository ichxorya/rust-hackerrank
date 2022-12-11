// https://www.hackerrank.com/challenges/time-conversion/problem?isFullScreen=true

#![allow(unused_imports)]
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'time_conversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn time_conversion(s: &str) -> String {
    let day_or_night: &str = &s[8..=9];
    let mut result: String = s.to_string();

    if day_or_night == "AM" {
        if &s[0..=1] == "12" {
            result = result.replace("12", "00");
        }
    } else {
        if &s[0..=1] != "12" {
            let old_hour = &s[0..=1];

            let mut new_hour: i32 = old_hour.parse().unwrap();
            new_hour += 12;
            let new_hour = new_hour.to_string();

            result = result.replace(old_hour, &new_hour);
        }
    }

    result.truncate(8);
    result
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create("output.txt").unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = time_conversion(&s);
    println!("{}", result);

    writeln!(&mut fptr, "{}", result).ok();
}
