use typst_syntax::{SyntaxNode};
use std::fs::File;
use std::io::prelude::*;

/// Creates typst file from Syntax tree
pub(crate) fn create_typst_file(root: SyntaxNode, file_path: &str) {
    let mut file = File::create(file_path).expect("Failed to create file");

    file.write_all(root.into_text().as_bytes()).expect("Failed to write to file");
}
