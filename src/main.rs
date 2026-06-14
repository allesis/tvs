mod parser;

use parser::Parser;

use std::env;

fn main() {
    let p = Parser::new(env::args().collect());

    println!("{}", p);
}
