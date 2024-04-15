use clap::{Command, Arg};
use clap::error::Result;
use std::fs;
use typst_syntax::{parse, SyntaxNode};
use serde_json;
use serde_json::{json, Value};
use std::io::Write;

fn serialize_syntax_node(node: &SyntaxNode) -> Value {
    // Assuming 'SyntaxNode' has fields like 'kind' and 'children'
    // You'll need to adjust these field names and structure based on the actual 'SyntaxNode' structure
    json!({
        "kind": format!("{:?}", node.kind()),  // Assuming there's a 'kind' method or field
        "children": node.children().map(serialize_syntax_node).collect::<Vec<_>>()  // Recursively serialize children
    })
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up command line argument parsing with Command instead of App
    let matches = Command::new("Typst AST Parser")
        .version("0.1.0")
        .author("Krzys")
        .about("Parses a Typst file and prints the AST")
        .arg(
            Arg::new("FILE")
                .help("First file")
                .required(true)
                .index(1))
        .arg( Arg::new("FILE2")
            .help("Second file")
            .required(true)
            .index(2))
        .get_matches();

    // Get the file path from the command line arguments
    let file_path1 = matches.get_one::<String>("FILE").unwrap();
    let file_path2 = matches.get_one::<String>("FILE2").unwrap();

    // Read the Typst file content
    let content1 = fs::read_to_string(file_path1)?;
    let content2 = fs::read_to_string(file_path2)?;

    // Parse the Typst file content into an AST
    let ast_tree1: SyntaxNode = parse(&content1);
    let ast_tree2: SyntaxNode = parse(&content2);

    // for child in ast_tree1.children() {
    //     println!("Child: {:?}", child);
    // }

    for it in ast_tree1.children().zip(ast_tree2.children()) {
        let (child1, child2) = it;
        if (child1 == child2) {
            println!("Equal children");
        } else {
            // println!("Child 1: {:?}", child1);
            // println!("\nChild 2: {:?}", child2);
            for child in child1.children() {
                    println!("Child: {:?}", child);
                }
        }
    }

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

    Ok(())
}