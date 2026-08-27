#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    print!("$ ");
    io::stdout().flush()?;

    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    // 3. Clean and process the input
    let trimmed_input = input.trim();
    println!("{trimmed_input}: command not found");

    Ok(())
}
