const WIDTH: usize = 50;

use std::io::{self, IsTerminal, Write};

pub struct Shell {
    is_tty: bool,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            is_tty: io::stdout().is_terminal(),
        }
    }

    pub fn status(&self, s: &str) {
        if !self.is_tty {
            return;
        }
        let s = if s.len() > WIDTH { &s[..WIDTH] } else { s };
        print!("{s:<WIDTH$}\r");
        io::stdout().flush().unwrap();
    }

    pub fn clear(&self) {
        if !self.is_tty {
            return;
        }
        self.status("");
    }
}

impl Drop for Shell {
    fn drop(&mut self) {
        self.clear();
    }
}
