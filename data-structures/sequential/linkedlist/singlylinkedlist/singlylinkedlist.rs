#![allow(unused_mut, dead_code, unused_variables)]

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    // pushing to the linked list
    fn push(&mut self, data: T) {
        let old_head = self.head.take();
        let mut new_node: Node<T> = Node::new(data);
        new_node.next = old_head;
        self.head = Some(Box::new(new_node));
        self.size += 1;
    }

    // popping from the linked list
    fn pop(&mut self) -> Option<T> {
        let old_head = self.head.take();
        match old_head {
            Some(node_val) => {
                self.head = node_val.next;
                self.size -= 1;
                Some(node_val.data)
            }
            None => None,
        }
    }

    // inserting to the linked list
    fn insert(&mut self, data: T, index: usize) -> Result<(), String> {
        let mut curr_node: &mut Option<Box<Node<T>>> = &mut self.head;

        if index > self.size {
            Err(String::from("Error: Index out of bounds"))
        } else {
            if index == 0 {
                self.push(data);
                self.size += 1;
                Ok(())
            } else {
                let mut new_node: Node<T> = Node::new(data);
                let mut rest_list: Option<Box<Node<T>>> = None;
                for _ in 0..index {
                    match curr_node {
                        Some(node_val) => {
                            curr_node = &mut node_val.next;
                        }
                        None => break,
                    }
                }
                new_node.next = curr_node.take();
                *curr_node = Some(Box::new(new_node));
                self.size += 1;
                Ok(())
            }
        }
    }

    // removing from the linked list
    fn remove(&mut self, index: usize) -> Result<T, String> {
        if index > self.size {
            Err(String::from("Error: Index out of bounds!"))
        } else {
            if index == 0 {
                return Ok(self.pop().unwrap());
            } else {
                let mut curr_node: &mut Option<Box<Node<T>>> = &mut self.head;
                for _ in 0..index {
                    match curr_node {
                        Some(ref mut node_val) => {
                            curr_node = &mut node_val.next;
                        }
                        None => break,
                    }
                }
                if let Some(node_val) = curr_node.take() {
                    *curr_node = node_val.next;
                    return Ok(node_val.data);
                } else {
                    return Err(String::from("Error: Removed value doesnt exists!"));
                }
            }
        }
    }
}

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    println!("List size: {}", list.size);
    let _ = list.insert(4, 1);
    println!("{:#?}", list);
    list.remove(1);
    println!("{:#?}", list);
}
