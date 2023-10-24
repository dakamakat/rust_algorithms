#![allow(dead_code)]

use super::{helpers::{array_to_linked_list, print_linked_list}, algs::{merge_two_lists, reverse_list}};

pub fn test_merge_sorted_lists() {
    let arr1 = [1, 2, 3, 4, 5, 6, 7, 8];
    let arr2 = [1, 23, 45, 68];

    let list1 = array_to_linked_list(&arr1);
    let list2 = array_to_linked_list(&arr2);

    let merged = merge_two_lists(list1, list2);
    print_linked_list(merged)
}


pub fn test_reverse_list() {
    let arr1 = [1, 2, 3, 4, 5, 6, 7, 8];

    let list1 = array_to_linked_list(&arr1);

    let reversed1 = reverse_list(list1);

    print_linked_list(reversed1);
}
