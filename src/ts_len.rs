use crate::ts::TS;

impl TS {
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn stack_len(&mut self) -> usize {
        match self.stack.peek() {
            Some(curr) => {
                return curr.len();
            }
            None => {
                self.add_stack();
            }
        }
        0
    }
}
