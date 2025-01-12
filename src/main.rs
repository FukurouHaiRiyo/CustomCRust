mod List;

use List::deque::Deque;
use List::mem::Memory;
use List::node::Node;


fn main() {
    let mut memory = Memory::<Node<i32>>::new();
    let mut deque = Deque::<i32>::new();

    for i in 0..10 {
        deque.push_back(&mut memory, i);
    }
    println!();
    memory.debug_print();
    deque.debug_print(&memory);

    for i in 10..20 {
        deque.push_front(&mut memory, i);
    }
    println!();
    memory.debug_print();
    deque.debug_print(&memory);

    for _ in 0..20 {
        deque.pop_front(&mut memory);
    }
    println!();
    memory.debug_print();
    deque.debug_print(&memory);

    for i in 0..10 {
        deque.push_back(&mut memory, i);
    }
    println!();
    memory.debug_print();
    deque.debug_print(&memory);

    for i in 0..10 {
        deque.push_front(&mut memory, i);
    }
    println!();
    memory.debug_print();
    deque.debug_print(&memory);

    for i in 0..10 {
        deque.push_front(&mut memory, i);
    }
    println!();
    memory.debug_print();
    deque.debug_print(&memory);
}