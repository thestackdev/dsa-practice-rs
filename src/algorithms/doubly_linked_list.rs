use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

struct Node {
    val: i32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            val,
            left: None,
            right: None,
        }))
    }
}

pub struct DoublyLinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    pub len: usize,
}

impl DoublyLinkedList {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_back(&mut self, val: i32) {
        let new_node = Node::new(val);

        match self.tail.take() {
            Some(current) => {
                current.borrow_mut().right = Some(Rc::clone(&new_node));
                new_node.borrow_mut().left = Some(current);
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }

        self.len += 1;
    }

    pub fn push_front(&mut self, val: i32) {
        let new_node = Node::new(val);

        match self.head.take() {
            Some(current) => {
                current.borrow_mut().left = Some(Rc::clone(&new_node));
                new_node.borrow_mut().right = Some(Rc::clone(&current));
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }

        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        self.head.take().map(|head| {
            match &head.borrow_mut().right {
                Some(node) => {
                    node.borrow_mut().left = None;
                    self.head = Some(Rc::clone(node));
                }
                None => {
                    self.tail.take();
                }
            }

            self.len -= 1;
            Rc::try_unwrap(head).ok().unwrap().into_inner().val
        })
    }

    pub fn pop_back(&mut self) -> Option<i32> {
        self.tail.take().map(|tail| {
            match &tail.borrow_mut().left {
                Some(node) => {
                    node.borrow_mut().right = None;
                    self.tail = Some(Rc::clone(node));
                }
                None => {
                    self.head.take();
                }
            }

            self.len -= 1;
            Rc::try_unwrap(tail).ok().unwrap().into_inner().val
        })
    }

    pub fn print_nodes(&self) {
        let mut current = self.head.clone();

        while let Some(node) = current.take() {
            println!("{}", node.borrow().val);
            current = node.borrow().right.clone();
        }
    }
}

