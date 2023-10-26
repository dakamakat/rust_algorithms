#![allow(dead_code)]

use std::collections::HashSet;

use super::models::ListNode;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(l1), None) => Some(l1),
        (None, Some(l2)) => Some(l2),
        (Some(l1), Some(l2)) => match l1.val <= l2.val {
            true => Some(Box::new(ListNode {
                val: l1.val,
                next: merge_two_lists(l1.next, Some(l2)),
            })),
            false => Some(Box::new(ListNode {
                val: l2.val,
                next: merge_two_lists(Some(l1), l2.next),
            })),
        },
    }
}

pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
    let mut curr = &head;
    let mut set: HashSet<&Box<ListNode>> = HashSet::new();

    while let Some(node) = curr {
        if set.contains(&node) {
            return true;
        }

        curr = &node.next;
        set.insert(&node);
    }

    false
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    head
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut prev, mut curr) = (None, head);
    while let Some(mut node) = curr {
        curr = node.next;

        node.next = prev;

        prev = Some(node);
    }
    prev
}

pub fn reverse_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn recurse(head: Option<Box<ListNode>>, prev: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut node) => {
                let next = node.next.take();
                node.next = prev;
                recurse(next, Some(node))
            }
            None => prev,
        }
    }
    recurse(head, None)
}
