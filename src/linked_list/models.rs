pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        Self { val, next }
    }
}
