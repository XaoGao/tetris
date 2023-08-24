use crate::engine::{Engine, self};

pub struct Interface {
  engine: Engine
}

impl Interface {
    pub fn run(engine: Engine) {
      let interface = Self {
        engine
      };
      todo!("Run game")
    }
}
