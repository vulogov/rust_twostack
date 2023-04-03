use crate::ts::TS;

impl TS {
    pub fn return_to(&mut self) -> &mut TS {
        match self.pull() {
            Some(v) => {
                let _ = self.drop();
                self.push(v);

            }
            None => {},
        }
        self
    }
}
