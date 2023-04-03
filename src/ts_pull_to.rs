use crate::ts::TS;

impl TS {
    pub fn pull_to(&mut self) -> &mut TS {
        self.stack.left();
        match self.pull() {
            Some(v) => {
                self.stack.right();
                self.push(v);

            }
            None => {
                self.stack.right();
            }
        }
        self
    }
}
