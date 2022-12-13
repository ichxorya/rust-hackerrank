// https://www.hackerrank.com/challenges/grading/problem?isFullScreen=true

#![allow(unused_imports)]
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'grading_students' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY grades as parameter.
 */

fn grading_students(grades: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for grade_iter in grades {
        let mut grade: i32 = *grade_iter;
        let grade_mod_10: i32 = grade % 10;

        if (*grade_iter >= 38 && *grade_iter < 100) && (*grade_iter % 5 != 0) {
            match grade_mod_10 {
                3 => grade += 2,
                8 => grade += 2,
                4 => grade += 1,
                9 => grade += 1,
                _ => (),
            }
        }

        result.push(grade);
    }

    result
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create("output.txt").unwrap();

    let grades_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut grades: Vec<i32> = Vec::with_capacity(grades_count as usize);

    for _ in 0..grades_count {
        let grades_item = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        grades.push(grades_item);
    }

    let result = grading_students(&grades);
    println!("{:#?}", result);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
