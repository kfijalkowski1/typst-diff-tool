// mod create_typst;
// mod create_result_ast;
//
// use clap::{Command, Arg};
// use clap::error::Result;
// use create_result_ast::create_ast_tree;
// use create_typst::create_typst_file;
//
//
// /// Reads arguments from command line and creates the result file
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Set up command line argument parsing with Command instead of App
//     let matches = Command::new("Typst AST Parser")
//         .version("0.1.0")
//         .author("Krzysztof Fijałkowski i Rafał Szczepaniak")
//         .about("Parses a Typst file and prints the AST")
//         .arg(
//             Arg::new("FILE")
//                 .help("First file")
//                 .required(true)
//                 .index(1))
//         .arg( Arg::new("FILE2")
//             .help("Second file")
//             .required(true)
//             .index(2))
//         .get_matches();
//     // Get the file path from the command line arguments
//     let file_path1 = matches.get_one::<String>("FILE").unwrap();
//     let file_path2 = matches.get_one::<String>("FILE2").unwrap();
//
//     let result_ast_tree = create_ast_tree(file_path1, file_path2);
//
//     create_typst_file(result_ast_tree, "result.typ");
//
//     Ok(())
// }


use std::collections::HashSet;

fn compare_nodes(list1: &[&str], list2: &[&str]) -> (Vec<String>, Vec<String>) {
    let added_in1st_nodes: HashSet<&str> = list1.iter().copied().filter(|&node| !list2.contains(&node)).collect();
    let added_in2nd_nodes: HashSet<&str> = list2.iter().copied().filter(|&node| !list1.contains(&node)).collect();

    let mut result = Vec::new();
    let mut result_flags = Vec::new();

    let len1 = list1.len();
    let len2 = list2.len();
    let mut i = 0;
    let mut j = 0;

    while i < len1 || j < len2 {
        if i < len1 && result.contains(&list1[i].to_string()) {
            i += 1;
        } else if i < len1 && j < len2 && added_in1st_nodes.contains(list1[i]) && added_in2nd_nodes.contains(list2[j]) {
            result.push((list2[j]).to_string()); // TODO not two pushes should be
            result.push((list1[i]).to_string());
            result_flags.push("modified".to_string());
            i += 1;
            j += 1;
        } else if i < len1 && added_in1st_nodes.contains(list1[i]) {
            result.push((list1[i]).to_string());
            result_flags.push("deleted".to_string());
            i += 1;
        } else if j < len2 && added_in2nd_nodes.contains(list2[j]) {
            result.push((list2[j]).to_string());
            result_flags.push("added".to_string());
            j += 1;
        } else if i < len1 && j < len2 && list1[i] == list2[j] {
            result.push((list1[i]).to_string());
            result_flags.push("same".to_string());
            i += 1;
            j += 1;
        } else if i < len1 && j < len2 && list1[i] != list2[j] {
            result.push((list2[j]).to_string());
            result_flags.push("moved".to_string());
            j += 1;
        }
    }

    (result, result_flags)
}

fn main() {
    let list1 = ["BOX0", "BOX1", "BOX2", "BOX3", "BOX99", "BOX4", "BOX5"];
    let list2 = ["BOX0", "BOX1", "BOX2", "BOX3", "BOX98", "BOX4", "BOX5"];

    let (result, result_flags) = compare_nodes(&list1, &list2);

    for (node, flag) in result.iter().zip(result_flags.iter()) {
        println!("Node: {}, Status: {}", node, flag);
    }
}

