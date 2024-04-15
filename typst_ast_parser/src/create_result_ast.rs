use clap::{Command, Arg};
use clap::error::Result;
use std::fs;
use typst_syntax::{parse, SyntaxNode};
use std::io::Write;


pub(crate) fn create_ast_tree(file_path1: &String, file_path2: &String) -> SyntaxNode {

    let content1 = fs::read_to_string(file_path1).expect("Couldn't read file");
    let content2 = fs::read_to_string(file_path2).expect("Couldn't read file");

    // Parse the Typst file content into an AST
    let ast_tree1: SyntaxNode = parse(&content1);
    let ast_tree2: SyntaxNode = parse(&content2);

    return ast_tree1;

    // for child in ast_tree1.children() {
    //     println!("Child: {:?}", child);
    // }

    // for it in ast_tree1.children().zip(ast_tree2.children()) {
    //     let (child1, child2) = it;
    //     if (child1 == child2) {
    //         println!("Equal children");
    //     } else {
    //         // println!("Child 1: {:?}", child1);
    //         // println!("\nChild 2: {:?}", child2);
    //         for child in child1.children() {
    //                 println!("{:?}", child.into_text());
    //             }
    //     }
    // }

    //
    // let serialized_ast = serialize_syntax_node(&ast_tree);
    //
    // let serialized = serde_json::to_string_pretty(&serialized_ast)?;
    // // Write the serialized string to a file
    // let mut file = fs::File::create("result.json")?;
    // file.write_all(serialized.as_bytes())?;
    // println!("AST saved to 'result.ast'.");

    // match astTree {
    //     Ok(tree) => {
    //         println!("AST: {:?}", tree);
    //     }
    //     Err(e) => {
    //         println!("Failed to parse Typst file: {:?}", e);
    //     }
    // }


}