use std::mem;

pub struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    fn insert(node: Option<Box<Node>>, val: i32) -> Option<Box<Node>> {
        match node {
            Some(mut n) => {
                if val < n.val {
                    n.left = Node::insert(n.left.take(), val);
                } else if val > n.val {
                    n.right = Node::insert(n.right.take(), val);
                }

                Some(n)
            }
            None => Some(Box::new(Node::new(val))),
        }
    }

    fn in_order(node: &Option<Box<Node>>) {
        if let Some(n) = node {
            Self::in_order(&n.left);
            println!("{}", n.val);
            Self::in_order(&n.right);
        }
    }

    fn pre_order(node: &Option<Box<Node>>) {
        if let Some(n) = node {
            println!("{}", n.val);
            Self::pre_order(&n.left);
            Self::pre_order(&n.right);
        }
    }

    fn post_order(node: &Option<Box<Node>>) {
        if let Some(n) = node {
            Self::post_order(&n.left);
            Self::post_order(&n.right);
            println!("{}", n.val);
        }
    }

    fn invert(node: &mut Option<Box<Node>>) {
        if let Some(n) = node {
            mem::swap(&mut n.left, &mut n.right);

            Node::invert(&mut n.left);
            Node::invert(&mut n.right);
        }
    }
}

pub struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, val: i32) {
        self.root = Node::insert(self.root.take(), val)
    }

    pub fn in_order(&self) {
        Node::in_order(&self.root);
    }

    pub fn pre_order(&self) {
        Node::pre_order(&self.root);
    }

    pub fn post_order(&self) {
        Node::post_order(&self.root);
    }

    pub fn invert(&mut self) {
        Node::invert(&mut self.root);
    }

    pub fn invert_using_stack(&mut self) {
        if self.root.is_none() {
            return;
        }

        let mut stack = Vec::new();
        stack.push(self.root.as_mut().unwrap());

        while let Some(current_node) = stack.pop() {
            mem::swap(&mut current_node.left, &mut current_node.right);

            if let Some(node) = &current_node.left {
                stack.push(current_node.left.as_mut().unwrap());
            }
            if let Some(node) = &current_node.right {
                stack.push(current_node.right.as_mut().unwrap());
            }
        }
    }
}
