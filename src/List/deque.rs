use std::marker::PhantomData;

use super::mem::Memory;
use super::node::Node;

pub struct Deque<T> {
    begin: Option<usize>,
    end: Option<usize>,
    phantom: PhantomData<T>,
}

impl<T: Clone + std::fmt::Debug> Deque<T> {
    pub fn new() -> Self {
        Self {
            begin: None,
            end: None,
            phantom: Default::default(),
        }
    }

    pub fn debug_print(&self, memory: &Memory<Node<T>>) {
        let mut iter = self.begin;
        print!("deque: |");
        while let Some(node_ref) = iter {
            print!("{:?} ", memory.deref(node_ref).value);
            iter = memory.deref(node_ref).next;
        }
        print!("|");
        println!();
    }

    pub fn push_front(&mut self, memory: &mut Memory<Node<T>>, value: T) {
        let new_node = memory.alloc(Node::new(value));
        if let Some(begin_node) = self.begin {
            memory.deref_mut(new_node).next = Some(begin_node);
            memory.deref_mut(begin_node).prev = Some(new_node);
            self.begin = Some(new_node);
        } else {
            assert!(self.end.is_none());
            self.begin = Some(new_node);
            self.end = Some(new_node);
        }
    }

    pub fn push_back(&mut self, memory: &mut Memory<Node<T>>, value: T) {
        let new_node = memory.alloc(Node::new(value));
        if let Some(end_node) = self.end {
            memory.deref_mut(new_node).prev = Some(end_node);
            memory.deref_mut(end_node).next = Some(new_node);
            self.end = Some(new_node);
        } else {
            assert!(self.begin.is_none());
            self.begin = Some(new_node);
            self.end = Some(new_node);
        }
    }

    pub fn pop_back(&mut self, memory: &mut Memory<Node<T>>) {
        if let Some(end_ref) = self.end {
            self.end = memory.deref(end_ref).prev;
            memory.dealloc(end_ref);

            if self.end.is_none() {
                self.begin = None;
            }
        }
    }

    pub fn pop_front(&mut self, memory: &mut Memory<Node<T>>) {
        if let Some(begin_ref) = self.begin {
            self.begin = memory.deref(begin_ref).next;
            memory.dealloc(begin_ref);

            if self.begin.is_none() {
                self.end = None;
            }
        }
    }
}
