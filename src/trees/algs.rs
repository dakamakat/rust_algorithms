#![allow(dead_code)]

use super::models::TreeNode;
use std::{cell::RefCell, cmp::Ordering, rc::Rc, collections::VecDeque};

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

pub fn is_cousins_bfs(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut vd = VecDeque::new();
        if let Some(r) = root {
            vd.push_back((r, None));
        }
        while !vd.is_empty() {
            let mut x_found = None;
            let mut y_found = None;
            for _ in 0..vd.len() {
                if let Some((node, parent)) = vd.pop_front() {
                    let val = node.borrow().val;
                    if val == x {
                        x_found = parent;
                    }
                    if val == y {
                        y_found = parent;
                    }
                    if let Some(n) = node.borrow_mut().left.take() {
                        vd.push_back((n, Some(val)));
                    }
                    if let Some(n) = node.borrow_mut().right.take() {
                        vd.push_back((n, Some(val)));
                    }
                }
            }
            if let (Some(x_parent), Some(y_parent)) = (x_found, y_found) {
                return x_parent != y_parent;
            }
        }
        false
    }

pub fn is_cousins_dfs(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
    let mut depth: Vec<i32> = Vec::new();
    let mut par: Vec<i32> = Vec::new();

    dfs(&root, -1, x, y, &mut depth, &mut par, 0);

    return depth[0] == depth[1] && par[0] != par[1];
}

pub fn dfs(
    root: &Option<Rc<RefCell<TreeNode>>>,
    value: i32,
    x: i32,
    y: i32,
    depth: &mut Vec<i32>,
    par: &mut Vec<i32>,
    d: i32,
) {
    if let Some(root) = root {
        let node = root.borrow();

        if node.val == x || node.val == y {
            depth.push(d);
            par.push(value);
        }

        dfs(&node.left, node.val, x, y, depth, par, d + 1);
        dfs(&node.right, node.val, x, y, depth, par, d + 1);
    } else {
        return;
    }
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
