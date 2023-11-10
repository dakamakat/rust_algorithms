#![allow(dead_code)]

use super::models::Stack;

pub fn is_valid(s: String) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }

    let mut stack: Stack<char> = Stack::new();

    let mut filled = false;

    let letters: Vec<char> = s.chars().collect();

    for symbol in letters {
        if symbol == '(' || symbol == '[' || symbol == '{' {
            filled = true;
            stack.push(symbol);
            continue;
        }

        if !stack.is_empty() {
            if symbol == ')' && *stack.top().unwrap() == '(' {
                stack.pop();
                continue;
            } else if symbol == ']' && *stack.top().unwrap() == '[' {
                stack.pop();
                continue;
            } else if symbol == '}' && *stack.top().unwrap() == '{' {
                stack.pop();
                continue;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    stack.is_empty() && filled
}

pub fn is_valid_clean(s: String) -> bool {
    if s.len() & 1 == 1 {
        return false;
    }

    let mut v = Vec::with_capacity(s.len());

    for c in s.chars() {
        match c {
            '(' | '[' | '{' => v.push(c),
            _ => match v.pop() {
                Some('(') if c == ')' => (),
                Some('[') if c == ']' => (),
                Some('{') if c == '}' => (),
                _ => return false,
            },
        }
    }

    v.is_empty()
}

pub fn trap(height: Vec<i32>) -> i32 {
    todo!()
}
