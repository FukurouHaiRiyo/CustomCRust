pub mod List;
mod Vector;

use List::deque::Deque;
use List::mem::Memory;
use List::node::Node;
use Vector::vec::Vec;


fn main() {
    let mut memory = Memory::<Node<i32>>::new();
    let mut deque = Deque::<i32>::new();

    for i in 0..10 {
        deque.push_back(&mut memory, i);
    }
    println!();
    deque.debug_print(&memory);

    // Create a new vector
    let mut vec = Vec::new();

    // Push elements into the vector
    vec.push(10);
    vec.push(20);
    vec.push(30);

    // Print the vector's length and capacity
    println!("Length of vector: {}", vec.len());
    println!("Capacity of vector: {}", vec.capacity());

}