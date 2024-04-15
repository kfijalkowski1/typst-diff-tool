use clap::error::Result;
use typst_syntax::{ SyntaxNode};
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;


pub(crate) fn create_typst_file(root: SyntaxNode, file_path: &str) {
    let mut file = File::create(file_path).expect("Failed to create file");

    file.write_all(root.into_text().as_bytes()).expect("Failed to write to file");

}