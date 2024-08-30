use anyhow::{Context, Result};
use std::{io, io::Write};

pub fn read_input(prompt: &str) -> Result<String> {
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .context("Failed to read input")?;

    Ok(input.trim().to_string())
}
