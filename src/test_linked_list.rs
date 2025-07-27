use std::cell::RefCell;

#[derive(Debug)]
pub struct TestLinkedList {
    pub data: String,
    pub pointer: Box<RefCell<Option<TestLinkedList>>>,
}

pub fn create_test_linked_list(data: &str, pointer: Option<TestLinkedList>) -> TestLinkedList {
    TestLinkedList {
        data: String::from(data),
        pointer: Box::new(RefCell::new(pointer)),
    }
}
