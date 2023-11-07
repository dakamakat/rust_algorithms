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
    queue: Vec<i32>,
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
