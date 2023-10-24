#![allow(dead_code)]

use super::models::ListNode;

pub fn array_to_linked_list(arr: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;

    for n in arr {
        let new_node = ListNode::new(*n, head);
        head = Some(Box::new(new_node))
    }
    head
}

pub fn print_linked_list(head: Option<Box<ListNode>>) {
    let mut curr = head;
    while let Some(node) = curr {
        curr = node.next;
        println!("{0}" , node.val);
    }
}
