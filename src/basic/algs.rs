#![allow(dead_code)]

use std::{cmp::Ordering, collections::HashMap};

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    for i in 0..nums.len() {
        if nums[i] == target {
            return i as i32;
        }
    }

    -1
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut min: i32 = 0;
    let mut max: i32 = (nums.len() - 1) as i32;
    let mut middle: i32;

    while min <= max {
        middle = (min + max) / 2;

        match nums[middle as usize].cmp(&target) {
            Ordering::Equal => {
                return middle;
            }
            Ordering::Greater => {
                max = middle - 1;
            }
            Ordering::Less => {
                min = middle + 1;
            }
        }
    }
    min
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit: i32 = 0;
    let mut min_price: i32 = 10000;

    for price in prices {
        max_profit = max_profit.max(price - min_price);
        min_price = min_price.min(price);
    }

    max_profit
}

pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut vec = vec![];

    for i in 1..n + 1 {
        if i % 3 == 0 && i % 5 == 0 {
            vec.push("FizzBuzz".to_string());
            continue;
        } else if i % 3 == 0 {
            vec.push("Fizz".to_string());
            continue;
        } else if i % 5 == 0 {
            vec.push("Buzz".to_string());
            continue;
        } else {
            vec.push(i.to_string())
        }
    }

    vec
}

pub fn is_palindrome(x: i32) -> bool {
    let mut r = 0;
    let mut c = x;

    while c > 0 {
        r = r * 10 + c % 10;
        c /= 10;
    }

    r == x
}

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    if nums.len() == 1 || k == 0 {
        return;
    }

    let mut arr = nums.clone();

    let mut rs: usize = k as usize;

    if k as usize > nums.len() {
        rs = k as usize % nums.len();
    }

    let mut temp: i32;

    for i in 0..(nums.len()) {
        temp = nums[i];

        if i + rs >= nums.len() {
            let cind = i + rs - nums.len();
            arr[cind] = temp;
        } else {
            arr[i + rs] = temp;
        }
    }

    for i in 0..arr.len() {
        nums[i] = arr[i]
    }
}

pub fn rotate_basic(nums: &mut Vec<i32>, k: i32) {
    let rs: usize = k as usize % nums.len();
    nums.rotate_right(rs);
}

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut f_ind = m + n - 1;

    for i in (0..nums2.len() - 1).rev() {
        nums1[f_ind as usize] = nums2[i];
        f_ind -= 1;
    }

    nums1.sort();
}

pub fn roman_to_int(s: String) -> i32 {
    let roman_numerals: HashMap<char, i32> = HashMap::from([
        ('I'.to_owned(), 1),
        ('V'.to_owned(), 5),
        ('X'.to_owned(), 10),
        ('L'.to_owned(), 50),
        ('C'.to_owned(), 100),
        ('D'.to_owned(), 500),
        ('M'.to_owned(), 1000),
    ]);

    let mut sum = 0;
    let mut last = 0;
    let mut current: i32;

    let bstr: &[u8] = s.as_bytes();

    for i in (0..s.len()).rev() {
        current = roman_numerals[&(bstr[i] as char)];
        if current < last {
            sum -= current;
        } else {
            sum += current;
        }
        last = current;
    }

    sum
}
