use crate::List::node::Node;
use crate::List::mem::Memory;
use std::fmt::Debug;
use std::marker::PhantomData;

/// A doubly linked list implementation that uses the custom `Memory` manager.
pub struct List<T> {
    begin: Option<usize>, // Index of the first node in the list
    end: Option<usize>,   // Index of the last node in the list
    phantom: PhantomData<T>, // Marker to tie `List` to type `T`
}

impl<T: Clone + Debug> List<T> {
    /// Creates a new, empty list.
    pub fn new() -> Self {
        Self {
            begin: None,
            end: None,
            phantom: PhantomData,
        }
    }

    /// Debugs and prints the contents of the list.
    pub fn debug_print(&self, memory: &Memory<Node<T>>) {
        let mut iter = self.begin;
        while let Some(node_ref) = iter {
            print!("{:?} ", memory.deref(node_ref).value);
            iter = memory.deref(node_ref).next;
        }
        println!();
    }

    /// Adds a value to the front of the list.
    pub fn push_front(&mut self, memory: &mut Memory<Node<T>>, value: T) {
        let new_node = memory.alloc(Node::new(value));
        if let Some(begin_node) = self.begin {
            memory.deref_mut(new_node).next = Some(begin_node);
            memory.deref_mut(begin_node).prev = Some(new_node);
        } else {
            self.end = Some(new_node);
        }
        self.begin = Some(new_node);
    }

    /// Adds a value to the back of the list.
    pub fn push_back(&mut self, memory: &mut Memory<Node<T>>, value: T) {
        let new_node = memory.alloc(Node::new(value));
        if let Some(end_node) = self.end {
            memory.deref_mut(new_node).prev = Some(end_node);
            memory.deref_mut(end_node).next = Some(new_node);
        } else {
            self.begin = Some(new_node);
        }
        self.end = Some(new_node);
    }

    /// Removes and discards the value at the back of the list.
    pub fn pop_back(&mut self, memory: &mut Memory<Node<T>>) {
        if let Some(end_ref) = self.end {
            self.end = memory.deref(end_ref).prev;
            memory.dealloc(end_ref);
            if self.end.is_none() {
                self.begin = None;
            }
        }
    }

    /// Removes and discards the value at the front of the list.
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
