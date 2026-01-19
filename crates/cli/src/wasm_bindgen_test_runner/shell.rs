const WIDTH: usize = 50;

use std::io::{self, Write};

pub struct Shell {}

impl Shell {
    pub fn new() -> Shell {
        Shell {}
    }

    pub fn status(&self, s: &str) {
        let s = if s.len() > WIDTH { &s[..WIDTH] } else { s };
        print!("{s:<WIDTH$}\r");
        io::stdout().flush().unwrap();
    }

    pub fn clear(&self) {
        self.status("");
    }
}

impl Drop for Shell {
    fn drop(&mut self) {
        self.clear();
    }
}
