use anyhow::{Context, Result};
use std::fs;
use std::fs::File;
use std::io::prelude::*;

use typst_syntax::{parse, SyntaxNode};

pub fn create_typst_file(root: SyntaxNode, file_path: &str) -> Result<()> {
    let mut file = File::create(file_path).context("Failed to create file")?;
    file.write_all(root.into_text().as_bytes())
        .context("Failed to write to file")?;
    Ok(())
}

pub fn read_typst_file(file_path: &str) -> Result<SyntaxNode> {
    let content = fs::read_to_string(file_path).context("Couldn't read file")?;
    Ok(parse(&content))
}
