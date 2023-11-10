#![allow(dead_code)]

pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn top(&self) -> Option<&T> {
        self.stack.last()
    }

    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.stack.last_mut()
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

pub struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self { queue: Vec::new() }
    }

    fn length(&self) -> usize {
        self.queue.len()
    }

    fn enqueue(&mut self, item: T) {
        self.queue.push(item)
    }

    fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }
    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
}

pub struct CircularQueue {
    capacity: i32,
    pub queue: Vec<i32>,
}

impl CircularQueue {
    pub fn new(k: i32) -> Self {
        Self {
            capacity: k,
            queue: Vec::new(),
        }
    }

    pub fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.queue.push(value);
        true
    }

    pub fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.queue.remove(0);
        true
    }
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn front(&self) -> i32 {
        *self.queue.first().unwrap_or(&-1)
    }

    pub fn rear(&self) -> i32 {
        *self.queue.last().unwrap_or(&-1)
    }

    pub fn is_full(&self) -> bool {
        self.queue.len() == self.capacity as usize
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

pub struct NestedIteratorStack {
    stack: Vec<(NestedInteger, usize)>,
    next: Option<i32>,
}
impl NestedIteratorStack {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        Self {
            stack: vec![(NestedInteger::List(nested_list), 0)],
            next: None,
        }
    }

    fn next(&self) -> i32 {
        self.next.unwrap_or_else(|| panic!("No next value"))
    }

    fn has_next(&mut self) -> bool {
        use std::mem::swap;
        use NestedInteger::*;

        self.next = None;

        while let Some((nested, idx)) = self.stack.pop() {
            match nested {
                Int(val) => {
                    self.next = Some(val);
                    break;
                }
                List(mut list) => {
                    let len = list.len();

                    if idx < len {
                        let mut val = Int(i32::MIN);

                        swap(&mut val, &mut list[idx]);

                        if idx + 1 < len {
                            self.stack.push((List(list), idx + 1));
                        }
                        self.stack.push((val, 0));
                    }
                }
            }
        }
        self.next.is_some()
    }
}

struct NestedIteratorFlattened {
    flattened: Vec<i32>,
    index: usize,
}

impl NestedIteratorFlattened {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut flattened = Vec::new();
        Self::flatten(&nested_list, &mut flattened);

        Self {
            flattened,
            index: 0,
        }
    }

    fn flatten(nested: &[NestedInteger], result: &mut Vec<i32>) {
        for ni in nested {
            match ni {
                NestedInteger::Int(val) => result.push(*val),
                NestedInteger::List(list) => Self::flatten(list, result),
            }
        }
    }

    fn next(&mut self) -> i32 {
        let val = self.flattened[self.index];
        self.index += 1;
        val
    }

    fn has_next(&self) -> bool {
        self.index < self.flattened.len()
    }
}
