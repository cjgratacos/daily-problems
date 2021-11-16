use std::iter::IntoIterator;
use std::sync::{Arc, Mutex};

fn main() {}

type Link<T> = Option<Arc<Mutex<Box<Node<T>>>>>;

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub data: T,
    pub next: Link<T>,
    pub prev: Link<T>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            next: None,
            prev: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LinkedList<T> {
    pub head: Link<T>,
    pub tail: Link<T>,
    pub size: usize,
}

impl<T> LinkedList<T> {
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

            head
        })
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut vec = vec![];

        if self.head.is_some() {
            let next = self.head.as_ref().unwrap();
            loop {
                let next_lock = next.lock().unwrap();
                vec.push(next_lock.data);
            }
        }

        vec.i
    }
}
#[test]
fn test() {
    let mut list: LinkedList<usize> = LinkedList::new();
    list.push(0);
    list.push(1);
    list.push(2);
    loop {
        let node = list.pop();

        if node.is_none() {
            break;
        }

        let n = node.as_ref().unwrap().lock().unwrap();
        println!("{}:{:p}", &n.data, &n.data);
        println!("n:{:p}", &n);
        println!("next:{:p}", &n.next);
        println!("prev:{:p}", &n.prev);
    }

    assert_eq!(1 == 2, true);
}
