// tests/list_test.rs

use C_Rust::List;

#[test]
fn test_push_front() {
    let mut list = List::new();
    list.push_front(10);
    assert_eq!(list.size(), 1);
    assert_eq!(list.front(), Some(&10));
}

#[test]
fn test_push_back() {
    let mut list = List::new();
    list.push_back(20);
    assert_eq!(list.size(), 1);
    assert_eq!(list.back(), Some(&20));
}

#[test]
fn test_pop_front() {
    let mut list = List::new();
    list.push_front(30);
    assert_eq!(list.pop_front(), Some(30));
    assert_eq!(list.size(), 0);
}

#[test]
fn test_pop_back() {
    let mut list = List::new();
    list.push_back(40);
    assert_eq!(list.pop_back(), Some(40));
    assert_eq!(list.size(), 0);
}

#[test]
fn test_is_empty() {
    let mut list = List::new();
    assert!(list.is_empty());
    list.push_back(50);
    assert!(!list.is_empty());
}

#[test]
fn test_print() {
    let mut list = List::new();
    list.push_back(60);
    list.push_back(70);
    list.push_back(80);

    // Printing is not directly tested, but you could visually check the output:
    list.print();  // Expected output: 60 70 80
}