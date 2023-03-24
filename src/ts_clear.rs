use crate::ts::TS;

impl TS {
    pub fn clear(&mut self) -> &mut TS {
        self.stack.clear();
        self.ensure()
    }
}
