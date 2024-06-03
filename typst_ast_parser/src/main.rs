use clap::{Arg, Command};
use clap::error::Result;

use crate::node_handlers::create_result_ast::create_diff_ast_tree;
use crate::typst_handlers::typst_parser::create_typst_file;

mod node_handlers;
mod typst_handlers;
mod enums;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up command line argument parsing with Command instead of App
    let matches = Command::new("Typst AST Parser")
        .version("0.1.0")
        .author("Krzysztof Fijałkowski i Rafał Szczepaniak")
        .about("Parses a Typst file and prints the AST")
        .arg(
            Arg::new("FILE")
                .help("First file")
                .required(true)
                .index(1))
        .arg(Arg::new("FILE2")
            .help("Second file")
            .required(true)
            .index(2))
        .get_matches();
    // Get the file path from the command line arguments
    let file_path1 = matches.get_one::<String>("FILE").unwrap();
    let file_path2 = matches.get_one::<String>("FILE2").unwrap();

    let result_ast_tree = create_diff_ast_tree(file_path1, file_path2);

    create_typst_file(result_ast_tree, "result.typ");

    Ok(())
}
