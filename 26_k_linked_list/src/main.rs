use std::sync::{Arc, Mutex};

fn main() {}

type Link<T> = Option<Arc<Mutex<Box<Node<T>>>>>;

#[derive(Clone, Debug)]
pub struct Node<T: Clone> {
    pub data: T,
    pub next: Link<T>,
    pub prev: Link<T>,
}

impl<T: Clone> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            next: None,
            prev: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LinkedList<T: Clone> {
    pub head: Link<T>,
    pub tail: Link<T>,
    pub size: usize,
}

#[derive(Clone, Debug)]
pub struct NodeIterator<T: Clone> {
    pub node: Link<T>,
}

impl<T: Clone> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn push(&mut self, data: T) {
        let new_tail = Arc::new(Mutex::new(Box::new(Node::new(data))));

        let old_tail = std::mem::replace(&mut self.tail, Some(new_tail.clone()));

        match old_tail {
            Some(old_tail) => {
                let mut old_tail_lock = old_tail.lock().unwrap();
                let mut new_tail_lock = new_tail.lock().unwrap();
                old_tail_lock.next = Some(new_tail.clone());
                let _ = std::mem::replace(&mut new_tail_lock.prev, Some(old_tail.clone()));
            }
            None => {
                self.head = Some(new_tail.clone());
            }
        }

        self.size += 1;
        self.tail = Some(new_tail);
    }

    pub fn pop(&mut self) -> Link<T> {
        self.head.take().map(|head| {
            {
                let head_lock = head.lock().unwrap();
                match &head_lock.next {
                    Some(next) => {
                        let _ = std::mem::replace(&mut self.head, Some(next.clone()));
                    }
                    None => {
                        self.head = None;
                        self.tail = None;
                    }
                }
            }

            self.size -= 1;
            head
        })
    }

    pub fn remove_kth_reverse(&mut self, k: usize) -> Link<T> {
        if self.tail.is_none() || self.size < k {
            return None;
        }

        let mut location = 1;
        let mut prev = self.tail.as_ref().unwrap().clone();
        loop {
            if location == k {
                break;
            }

            let p = prev;
            let prev_lock = p.lock().unwrap();

            let new_prev = prev_lock.prev.as_ref().unwrap();

            prev = new_prev.clone();

            location += 1;
        }

        {
            let prev_lock = prev.lock().unwrap();
            match &prev_lock.next {
                // Means this is between two elements not beeing the end
                Some(next) => {
                    match &prev_lock.prev {
                        // If some it is between two elements
                        Some(prev) => {
                            let mut next_lock = next.lock().unwrap();
                            let mut prev_lock = prev.lock().unwrap();
                            // Update next prev to prev
                            let _ = std::mem::replace(&mut next_lock.prev, Some(prev.clone()));
                            // Update prev next to next
                            let _ = std::mem::replace(&mut prev_lock.next, Some(next.clone()));
                        }
                        // it is first
                        None => {
                            let mut next_lock = next.lock().unwrap();
                            // Update head
                            let _ = std::mem::replace(&mut self.head, Some(next.clone()));
                            // Update next prev
                            let _ = std::mem::replace(&mut next_lock.prev, None);
                        }
                    }
                }
                // Means this is the end
                None => {
                    match &prev_lock.prev {
                        // True end
                        Some(prev) => {
                            let mut prev_lock = prev.lock().unwrap();
                            // Update tail with new prev
                            let _ = std::mem::replace(&mut self.tail, Some(prev.clone()));
                            // Update prev next to empty
                            prev_lock.next = None;
                        }
                        // Single Item List, now there are none
                        None => {
                            // Update Head to empty
                            self.head = None;
                            // Update Tail to empty
                            self.tail = None;
                        }
                    }
                }
            }
        }

        self.size -= 1;
        Some(prev)
    }

    pub fn iter(&self) -> NodeIterator<T> {
        NodeIterator {
            node: self.head.clone(),
        }
    }
}

impl<T: Clone> Iterator for NodeIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result: Option<T> = None;
        if let Some(current) = &self.node.clone() {
            let current_lock = current.lock().unwrap();

            result = Some(current_lock.data.clone());

            match &current_lock.next {
                Some(next) => {
                    let _ = std::mem::replace(&mut self.node, Some(next.clone()));
                }
                None => {
                    self.node = None;
                }
            }
            // set node to next
        }

        result
    }
}

#[test]
fn test() {
    let mut list: LinkedList<usize> = LinkedList::new();
    list.push(0);
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(5);

    assert_eq!(list.remove_kth_reverse(2).is_some(), true);
    assert_eq!(
        list.iter().collect::<Vec<usize>>().eq(&vec![0, 1, 2, 3, 5]),
        true
    );

    assert_eq!(list.remove_kth_reverse(1).is_some(), true);
    assert_eq!(
        list.iter().collect::<Vec<usize>>().eq(&vec![0, 1, 2, 3]),
        true
    );

    assert_eq!(list.remove_kth_reverse(4).is_some(), true);
    assert_eq!(list.iter().collect::<Vec<usize>>().eq(&vec![1, 2, 3]), true);

    assert_eq!(list.remove_kth_reverse(4).is_none(), true);

    assert_eq!(list.remove_kth_reverse(1).is_some(), true);
    assert_eq!(list.iter().collect::<Vec<usize>>().eq(&vec![1, 2]), true);
    assert_eq!(list.remove_kth_reverse(1).is_some(), true);
    assert_eq!(list.iter().collect::<Vec<usize>>().eq(&vec![1]), true);
    assert_eq!(list.remove_kth_reverse(1).is_some(), true);
    assert_eq!(list.iter().collect::<Vec<usize>>().eq(&vec![]), true);
    assert_eq!(list.remove_kth_reverse(1).is_none(), true);
    assert_eq!(list.remove_kth_reverse(0).is_none(), true);
}
