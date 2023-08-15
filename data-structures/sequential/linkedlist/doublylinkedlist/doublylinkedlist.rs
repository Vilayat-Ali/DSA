/*

EXPLANATION:
===========

*/

use std::default;

#[derive(Debug)]
struct ListNode<T> {
    data: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

#[derive(Debug)]
struct SinglyLinkedList<T> {
    head: Option<ListNode<T>>,
    size: usize,
}

impl<T> default::Default for SinglyLinkedList<T> {
    fn default() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }
}

impl<T> SinglyLinkedList<T> {
    fn init() -> Self {
        SinglyLinkedList::default()
    }

    fn push(&mut self, data: T) {}

    fn pop(&mut self) -> T {}

    fn append(&mut self, data: T) {}

    fn remove(&mut self, index: usize) -> usize {}

    fn swap_remove(&mut self, index: usize) -> usize {}
}
