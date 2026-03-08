struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self { value, next: None }
    }
}

pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn len(&self) -> usize {
        let mut counter = 0;
        let mut current_node = &self.head;

        while let Some(node) = current_node {
            counter += 1;
            current_node = &node.next;
        }

        counter
    }

    pub fn print(&self) {
        let mut current_node = &self.head;

        while let Some(node) = current_node {
            println!("{}", node.value);
            current_node = &node.next;
        }
    }

    pub fn insert_head(&mut self, value: i32) {
        let mut new_node = Box::new(Node::new(value));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    pub fn insert_last(&mut self, value: i32) {
        let new_node = Box::new(Node::new(value));

        if self.head.is_none() {
            self.insert_head(value);
            return;
        }

        let mut current_node = self.head.as_mut().unwrap();

        while current_node.next.is_some() {
            current_node = current_node.next.as_mut().unwrap();
        }

        current_node.next = Some(new_node)
    }

    pub fn insert_at_index(&mut self, index: usize, value: i32) -> bool {
        let length = self.len();

        if index > length {
            return false;
        }

        if index == 0 {
            self.insert_head(value);
            return true;
        }

        if index == length {
            self.insert_last(value);
            return true;
        }

        let mut new_node = Box::new(Node::new(value));
        let mut current_node = self.head.as_mut().unwrap();

        for _ in 0..(index - 1) {
            current_node = current_node.next.as_mut().unwrap();
        }

        new_node.next = current_node.next.take();
        current_node.next = Some(new_node);

        true
    }

    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current_node = self.head.take();

        while let Some(mut node) = current_node {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current_node = next;
        }

        self.head = prev
    }
}
