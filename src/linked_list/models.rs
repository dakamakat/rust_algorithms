use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        Self { val, next }
    }
}

#[derive(Debug, Copy, Clone)]
struct CacheItem {
    value: i32,
    version: i32,
}

#[derive(Debug)]
pub struct LRUCache {
    cache: HashMap<i32, CacheItem>,
    capacity: i32,
    version: i32,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            cache: HashMap::new(),
            capacity,
            version: 0,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        self.version += 1;
        let version = self.version;
        self.cache
            .get_mut(&key)
            .map(|item| {
                item.version = version;
                item.value
            })
            .unwrap_or(-1)
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.version += 1;
        let version = self.version;
        if !self.cache.contains_key(&key) && self.cache.len() == self.capacity as usize {
            if let Some((&remove_key, _)) = self.cache.iter().min_by_key(|(_, item)| item.version) {
                self.cache.remove(&remove_key);
            }
        }
        self.cache
            .entry(key)
            .and_modify(|item| {
                item.version = version;
                item.value = value;
            })
            .or_insert(CacheItem {
                value,
                version: self.version,
            });
    }
}
