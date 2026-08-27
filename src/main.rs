#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush()?;

        let mut input = String::new();

        io::stdin().read_line(&mut input)?;

        let trimmed_input = input.trim();
        println!("{trimmed_input}: command not found");
    }
}
