use std::fs;
use std::fs::File;
use std::io::prelude::*;

use typst_syntax::{parse, SyntaxNode};

pub fn create_typst_file(root: SyntaxNode, file_path: &str) {
    let mut file = File::create(file_path).expect("Failed to create file");

    file.write_all(root.into_text().as_bytes()).expect("Failed to write to file");
}

pub fn read_typst_file(file_path: &str) -> SyntaxNode{
    let content: String = fs::read_to_string(file_path).expect("Couldn't read file");
    parse(&content)
}
