use core::fmt;

pub struct Parser {
    args: Vec<String>,
}

impl Parser {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

impl fmt::Display for Parser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.args.len();
        self.args.iter().enumerate().for_each(|(v, e)| {
            let _ = write!(f, "{}", e);
            if v < len - 1 {
                let _ = write!(f, ", ");
            }
        });
        Ok(())
    }
}
