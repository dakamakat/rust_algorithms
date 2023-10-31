#![allow(dead_code)]

use super::algs::is_valid;

pub fn test_is_valid() {
    let s = "()";
    let s2 = "()[]{}";
    let s3 = "(]";
    let s4 = "{()}";
    let s5 = "){";
    let s6 = "([}}])";

    println!(
        "{0} {1} {2} {3} {4} {5}",
        is_valid(s.to_string()),
        is_valid(s2.to_string()),
        is_valid(s3.to_string()),
        is_valid(s4.to_string()),
        is_valid(s5.to_string()),
        is_valid(s6.to_string())
    )
}
