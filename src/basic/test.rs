use super::algs::{
    fizz_buzz, is_palindrome, max_profit, merge, roman_to_int, rotate, rotate_basic, search,
    search_insert,
};

pub fn test_search() {
    let nums1 = vec![1, 2];
    let nums2 = vec![1, 2, 3, 4, 5, 6, 7];
    let nums3 = vec![2, 2, 3, 5];

    println!("{0}", search(nums1.clone(), 5));
    println!("{0}", search(nums2.clone(), 2));
    println!("{0}", search(nums3.clone(), 4));
}

pub fn test_search_insert() {
    let nums1 = vec![1, 2];
    let nums2 = vec![1, 2, 3, 4, 5, 6, 7];
    let nums3 = vec![2, 2, 3, 5];

    println!("{0}", search_insert(nums1.clone(), 5));
    println!("{0}", search_insert(nums2.clone(), 2));
    println!("{0}", search_insert(nums3.clone(), 4));
}

pub fn test_max_profit() {
    let prices2 = vec![7, 1, 5, 3, 6, 4];
    let prices = vec![7, 6, 4, 3, 1];

    println!("{0} {1}", max_profit(prices), max_profit(prices2))
}

pub fn test_fizz_buzz() {
    println!("{:?}", fizz_buzz(5555));
    println!("{:?}", fizz_buzz(5));
    println!("{:?}", fizz_buzz(22222225));
}

pub fn test_is_palindrome() {
    let x = 121;
    let z = -10;
    println!("{0} {1}", is_palindrome(x), is_palindrome(z));
}

pub fn test_rotate() {
    let mut nums1 = vec![1, 2];
    let mut nums2 = vec![1, 2, 3, 4, 5, 6, 7];
    rotate(&mut nums1, 3);
    rotate(&mut nums2, 3);
    println!("{:?}", nums1);
    println!("{:?}", nums2);
}

pub fn test_merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut nums1 = vec![1, 2];
    let mut nums2 = vec![1, 2, 3, 4, 5, 6, 7];
    merge(&mut nums1, 3, &mut nums2, 4);
    println!("{:?}", nums1);
    println!("{:?}", nums2);
}
pub fn test_roman_to_int(s: String) {
    let s = "IV";
    let z = "LVIII";
    let f = "MCMXCIV";
    println!(
        "{0} {1} {2}",
        roman_to_int(s.to_string()),
        roman_to_int(z.to_string()),
        roman_to_int(f.to_string())
    );
}
