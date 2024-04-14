use clap::{Command, Arg};
use clap::error::Result;
use std::fs;
use typst_syntax::{parse, SyntaxNode};
use serde_json;
use serde_json::{json, Value};

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
        .author("Your Name")
        .about("Parses a Typst file and prints the AST")
        .arg(
            Arg::new("FILE")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Get the file path from the command line arguments
    let file_path = matches.get_one::<String>("FILE").unwrap();

    // Read the Typst file content
    let content = fs::read_to_string(file_path)?;

    // Parse the Typst file content into an AST
    let ast_tree: SyntaxNode = parse(&content);
    let serialized_ast = serialize_syntax_node(&ast_tree);

    let serialized = serde_json::to_string_pretty(&serialized_ast)?;
    // Write the serialized string to a file
    let mut file = fs::File::create("result.json")?;
    file.write_all(serialized.as_bytes())?;
    println!("AST saved to 'result.ast'.");

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