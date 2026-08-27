#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    print!("$ ");
    io::stdout().flush()?;

    Ok(())
}
