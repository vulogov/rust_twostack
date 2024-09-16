use crate::ts::TS;

impl TS {
    pub fn workbench_len(&self) -> usize {
        self.workbench.len()
    }
    pub fn return_to(&mut self) -> &mut TS {
        match self.pull() {
            Some(v) => {
                self.workbench.push(v);

            }
            None => {
                return self;
            }
        }
        self
    }
    pub fn return_from(&mut self) -> &mut TS {
        match self.workbench.pull() {
            Some(v) => {
                println!("{} {}", &v, self.len());
                self.push(v);
            }
            None => {
                return self;
            }
        }
        self
    }
}
