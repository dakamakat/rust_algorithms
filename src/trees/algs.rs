#![allow(dead_code)]

use super::models::TreeNode;
use std::{cell::RefCell, rc::Rc, cmp::Ordering};

type OptNode = Option<Rc<RefCell<TreeNode>>>;


pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(node) => {
            let value = node.clone();

            if value.borrow().val == val {
                return Some(node);
            } else if value.borrow().val < val {
                search_bst(value.borrow().right.clone(), val)
            } else if value.borrow().val > val {
                search_bst(value.borrow().left.clone(), val)
            } else {
                None
            }
        }
        None => None,
    }
}

pub fn search_bst_iter(root: OptNode, val: i32) -> OptNode {
    let mut node = root;
    while let Some(rc_node) = node.clone() {
        let cur_node = rc_node.borrow();
        match cur_node.val.cmp(&val) {
            Ordering::Equal => return node,
            Ordering::Less if cur_node.right.is_some() => node = cur_node.right.clone(),
            Ordering::Greater if cur_node.left.is_some() => node = cur_node.left.clone(),
            _ => break,
        }
    }
    None
}
