use crate::stack::Stack;

impl<T> Stack<T> {
    pub fn peek(&mut self) -> Option<&T> {
        if self.policy {
            self.stack.back()
        } else {
            self.stack.front()
        }
    }
}
