/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default + std::fmt::Debug,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + std::fmt::Debug,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        let mut value_idx = self.count;
        self.count += 1;
        loop {
            let parent_idx = self.parent_idx(value_idx);
            if value_idx == 0 || (self.comparator)(&self.items[parent_idx], &self.items[value_idx]) {
                break;
            } else {
                self.items.swap(parent_idx, value_idx);
                value_idx = parent_idx;
            }
        }
        println!("heap {:?}", self.items);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        (idx + 1) * 2 - 1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + std::fmt::Debug,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + std::fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.items.swap(0, self.count - 1);
        let ret = self.items.pop();
        self.count -= 1;
        let mut value_idx = 0;
        let mut c = 0;
        loop {
            let left = self.left_child_idx(value_idx);
            let left = if left < self.count { Some(left) } else { None };
            let right = self.right_child_idx(value_idx);
            let right = if right < self.count { Some(right) } else { None };
            println!("idx: {:?} left: {:?} right: {:?} items {:?}", value_idx, left, right, self.items);
            match (left, right) {
                (Some(left), Some(right)) => {
                    if (self.comparator)(&self.items[left], &self.items[value_idx]) {
                        if (self.comparator)(&self.items[right], &self.items[left]) {
                            self.items.swap(value_idx, right);
                            value_idx = right;
                        } else {
                            self.items.swap(value_idx, left);
                            value_idx = left;
                        }
                    } else if (self.comparator)(&self.items[right], &self.items[value_idx]) {
                        self.items.swap(value_idx, right);
                        value_idx = right;
                    } else {
                        break;
                    }
                },
                (Some(left), None) => {
                    if (self.comparator)(&self.items[left], &self.items[value_idx]) {
                        self.items.swap(value_idx, left);
                        value_idx = left;
                    } else {
                        break;
                    }
                },
                (None, Some(right)) => {
                    if (self.comparator)(&self.items[right], &self.items[value_idx]) {
                        self.items.swap(value_idx, right);
                        value_idx = right;
                    } else {
                        break;
                    }
                },
                (None, None) => {
                    println!("heap is sorted: {:?}", self.items);
                    break
                },
            }
            c += 1;
            if c > 16 {
                panic!("heap is not sorted");
            }
        }
        ret
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::fmt::Debug,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::fmt::Debug,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
