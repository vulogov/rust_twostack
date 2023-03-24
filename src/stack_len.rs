use crate::stack::Stack;

impl<T> Stack<T> {
    pub fn len(&self) -> usize {
        self.stack.len()
    }
}
