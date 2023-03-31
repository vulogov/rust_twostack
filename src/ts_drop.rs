use crate::ts::TS;

impl TS {
    pub fn drop(&mut self) -> &mut TS {
        let _ = self.stack.pull();
        self.ensure()
    }
}
