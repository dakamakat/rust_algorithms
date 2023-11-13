#![allow(dead_code)]

use super::models::TreeNode;
use std::{cell::RefCell, cmp::Ordering, rc::Rc};

type OptNode = Option<Rc<RefCell<TreeNode>>>;

pub fn find_bt_node_depth(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> i32 {
    let mut depth = -1;
    if let Some(node) = root {
        let rc_node = node.clone();
        let value = rc_node.borrow();

        if value.val == val {
            return depth + 1;
        }

        depth = find_bt_node_depth(value.left.clone(), val);

        if depth >= 0 {
            return depth + 1;
        }

        depth = find_bt_node_depth(value.right.clone(), val);
        if depth >= 0 {
            return depth + 1;
        }
    }
    depth
}
pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(node) => {
            let rc_node = node.clone();
            let value = rc_node.borrow();

            if value.val == val {
                return Some(node);
            } else if value.val < val {
                search_bst(value.right.clone(), val)
            } else if value.val > val {
                search_bst(value.left.clone(), val)
            } else {
                None
            }
        }
        None => None,
    }
}

pub fn search_bt(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(node) => {
            let rc_node = node.clone();
            let value = rc_node.borrow();

            if value.val == val {
                return Some(node);
            }

            let left = search_bt(value.left.clone(), val);

            if left.is_some() {
                return left;
            }

            let right = search_bt(value.right.clone(), val);

            if right.is_some() {
                return right;
            }

            None
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

pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
    fn find_bt_node_depth(
        root: Option<Rc<RefCell<TreeNode>>>,
        prevNodeVal: i32,
        val: i32,
    ) -> (i32, i32) {
        let mut depth = (prevNodeVal, -1);
        if let Some(node) = root {
            let rc_node = node.clone();
            let value = rc_node.borrow();

            if value.val == val {
                return (prevNodeVal, depth.1 + 1);
            }

            depth = find_bt_node_depth(value.left.clone(), value.val, val);

            if depth.1 >= 0 {
                return (depth.0, depth.1 + 1);
            }

            depth = find_bt_node_depth(value.right.clone(), value.val, val);
            if depth.1 >= 0 {
                return (depth.0, depth.1 + 1);
            }
        }
        (prevNodeVal, depth.1)
    }

    let x_depth = find_bt_node_depth(root.clone(), root.clone().unwrap().borrow().val, x);
    let y_depth = find_bt_node_depth(root.clone(), root.clone().unwrap().borrow().val, y);

    matches!((x_depth.0 != y_depth.0) && (x_depth.1 == y_depth.1), true)
}
