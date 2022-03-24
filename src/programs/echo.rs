
// echo clone

// This program should pass argument as string with " "
// For new line, don't end the quote and hit enter.
// eg. cargo run -- "hello world, -----> and hit enter
// john doe" <---- then end with quote.
// ">>" operator also work.

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pattern: String, 
}  

pub fn echo() {
    let args = Cli::parse();
    println!("{}", args.pattern);
}
