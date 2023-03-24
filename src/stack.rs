use std::collections;
use nanoid::nanoid;

pub struct Stack<T> {
    id:             String,
    pub policy:     bool,
    pub stack:      collections::VecDeque<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            id:     nanoid!(),
            policy: true,
            stack:  collections::VecDeque::<T>::new(),
        }
    }
    pub fn init(id: String) -> Self {
        let mut res = Stack::new();
        res.id = id;
        res
    }
}
