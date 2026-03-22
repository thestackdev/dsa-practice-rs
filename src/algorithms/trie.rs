use std::{
    collections::{self, HashMap},
    ops::Deref,
    vec,
};

use crate::algorithms::Tree;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;

        for ch in word.chars() {
            node = node.children.entry(ch).or_default();
        }
        node.is_end = true
    }

    pub fn search(&self, word: &str) -> bool {
        let mut node = &self.root;

        for ch in word.chars() {
            match node.children.get(&ch) {
                None => return false,
                Some(val) => node = val,
            }
        }
        node.is_end
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;

        for ch in prefix.chars() {
            match node.children.get(&ch) {
                None => return false,
                Some(val) => node = val,
            }
        }

        true
    }

    pub fn delete(&mut self, word: &str) -> bool {
        fn delete_helper(node: &mut TrieNode, chars: Vec<char>, depth: usize) -> bool {
            if chars.len() == depth {
                if !node.is_end {
                    return false;
                }
                node.is_end = false;
                return node.children.is_empty();
            }

            let ch = chars[depth];
            if let Some(child) = node.children.get_mut(&ch) {
                if delete_helper(child, chars, depth + 1) {
                    node.children.remove(&ch);
                    return !node.is_end && node.children.is_empty();
                }
            }

            false
        }

        let chars = word.chars().collect();
        delete_helper(&mut self.root, chars, 0)
    }

    pub fn autocomplete(&self, prefix: &str) -> Vec<String> {
        let mut node = &self.root;

        for ch in prefix.chars() {
            match node.children.get(&ch) {
                None => return vec![],
                Some(child) => node = child,
            }
        }

        let mut results = Vec::new();

        self.collect_words(node, prefix.to_string(), &mut results);

        results
    }

    fn collect_words(&self, node: &TrieNode, current: String, results: &mut Vec<String>) {
        if node.is_end {
            results.push(current.to_string());
        }

        for (ch, child) in &node.children {
            let mut current = current.clone();
            current.push(*ch);
            self.collect_words(child, current, results);
        }
    }

    fn search_wildcard(&self, word: &str) -> bool {
        let chars: Vec<char> = word.chars().collect();
        Self::search_wildcard_helper(&self.root, &chars, 0)
    }

    fn search_wildcard_helper(node: &TrieNode, chars: &[char], depth: usize) -> bool {
        if depth == chars.len() {
            return node.is_end;
        }

        let ch = chars[depth];

        if ch == '.' {
            for (key, child) in &node.children {
                if Self::search_wildcard_helper(child, chars, depth + 1) {
                    return true;
                }
            }
            false
        } else {
            match node.children.get(&ch) {
                None => false,
                Some(child) => Self::search_wildcard_helper(child, chars, depth + 1),
            }
        }
    }
}
