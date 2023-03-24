use crate::ts::TS;

impl TS {
    pub fn position(&mut self, name: String) -> &mut TS {
        let size = self.len();
        let mut c = 0;
        while c < size {
            match self.current() {
                Some(s) => {
                    if s.stack_id() == name {
                        return self;
                    }
                }
                None => return self,
            }
            self.stack.left();
            c += 1
        }
        self
    }
}
