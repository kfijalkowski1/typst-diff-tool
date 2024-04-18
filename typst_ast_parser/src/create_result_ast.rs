use std::fs;
use typst_syntax::{parse, SyntaxNode, SyntaxKind};


fn find_difference_in_children(node1: &SyntaxNode, node2: &SyntaxNode) -> SyntaxNode {

    // Check if both node1 and node2 are leaves
    if node1.children().count() == 0 && node2.children().count() == 0 {
        // Check if the text of node1 and node2 are different
        return if node1.text() != node2.text() {
            // Create a new leaf node combining the text of node1 and node2
            let combined_text = format!("#block(fill: red.transparentize(50%))[---({})] #block(fill: green.transparentize(50%))[+++({})]", node1.text(), node2.text());
            SyntaxNode::leaf(SyntaxKind::Auto, combined_text)
        } else {
            SyntaxNode::leaf(SyntaxKind::Auto, node1.text().to_string())
        }
    } else {
        let mut leaves: Vec<SyntaxNode> = Vec::new();
        // Iterate over children if nodes are not leaves
        for child_iter in node1.children().zip(node2.children()) {
            let (child1, child2) = child_iter;
            // Recursively call the function for children and get combined node
            let combined_child = find_difference_in_children(child1, child2);
            leaves.push(combined_child);
        }
        SyntaxNode::inner(SyntaxKind::Auto, leaves)
    }
}


pub(crate) fn create_ast_tree(file_path1: &String, file_path2: &String) -> SyntaxNode {
    let content1 = fs::read_to_string(file_path1).expect("Couldn't read file");
    let content2 = fs::read_to_string(file_path2).expect("Couldn't read file");

    // Parse the Typst file content into an AST
    let ast_tree1: SyntaxNode = parse(&content1);
    let ast_tree2: SyntaxNode = parse(&content2);

    let mut nodes: Vec<SyntaxNode> = Vec::new();

    for main_iter in ast_tree1.children().zip(ast_tree2.children()) {
        let (child1,child2) = main_iter;
        if child1 != child2 {
            let combined_node = find_difference_in_children(child1, child2);
            nodes.push(combined_node);
        } else {
            nodes.push(child1.clone());
        }
    }
    SyntaxNode::inner(SyntaxKind::Auto, nodes)
}
