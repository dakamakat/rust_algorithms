pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

trait NewTrait {
    fn new(val: i32) -> Self;
}

impl NewTrait for ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
