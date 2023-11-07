#![allow(dead_code)]

use super::{
    algs::{merge_two_lists, reverse_list},
    helpers::{array_to_linked_list, print_linked_list},
    models::{LRUCache, CircularQueue},
};

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

pub fn test_lru_cache() {
    let mut lRUCache = LRUCache::new(2);
    lRUCache.put(1, 1); // cache is {1=1}
    lRUCache.put(2, 2); // cache is {1=1, 2=2}
    lRUCache.get(1); // return 1
    lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
    lRUCache.get(2); // returns -1 (not found)
    lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
    lRUCache.get(1); // return -1 (not found)
    lRUCache.get(3); // return 3
    lRUCache.get(4); // return 4
    println!("{:?}", lRUCache.cache)
}

pub fn test_circular_queue() {
    let queue = CircularQueue::new(3);
}
