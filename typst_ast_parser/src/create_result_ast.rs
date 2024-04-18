use std::fs;
use std::slice::Iter;
use typst_syntax::{parse, SyntaxNode, SyntaxKind};


fn _node_already_exists(node: &SyntaxNode, parent: Iter<SyntaxNode>) -> bool {
    for iter_node in parent {
        if iter_node == node {
            return true
        }
    }
    false
}


fn add_color_to_every_block(node: &SyntaxNode, color: &String) -> SyntaxNode {
    let node_kind = node.kind();
    if node.children().count() == 0 {
        if node.text().len() > 2 {
            let signs: String = if color == "green" { "+++".to_string() } else { "---".to_string() };
            let combined_text = format!("#block(fill: {}.transparentize(50%))[{}({})]", color, signs, node.text());
            SyntaxNode::leaf(SyntaxKind::Text, combined_text)
        } else {
            SyntaxNode::leaf(SyntaxKind::Text, node.text().to_string())
        }
    } else {
        let mut inner_nodes: Vec<SyntaxNode> = Vec::new();
        for iter in node.children() {
            let colored_child = add_color_to_every_block(iter, color);
            inner_nodes.push(colored_child);
        }
        SyntaxNode::inner(node_kind, inner_nodes)
    }
}

fn find_difference_in_children(node1: Option<&SyntaxNode>, node2: Option<&SyntaxNode>) -> SyntaxNode {
    // Check if both node1 and node2 are leaves
    let combined_text: String;
    if (node1.is_none() && node2.is_some()) || (node1.is_some() && node2.is_none()){
        let colored_node: SyntaxNode;
        if node1.is_none() {
            colored_node = add_color_to_every_block(node2.unwrap(), &"green".to_string());
        } else {
            colored_node = add_color_to_every_block(node1.unwrap(), &"red".to_string());
        }
        colored_node
    } else {
        let child_old = node1.expect("The other file has different node");
        let child_new = node2.expect("The other file has different node");
        let node_kind = child_new.kind();

        if child_old.children().count() == 0 && child_new.children().count() == 0 {
            // Check if the text of node1 and node2 are different
            combined_text = format!("#block(fill: red.transparentize(50%))[---({})] #block(fill: green.transparentize(50%))[+++({})]", child_old.text(), child_new.text());

            return if node1 != node2 {
                // Create a new leaf node combining the text of node1 and node2
                SyntaxNode::leaf(SyntaxKind::Text, combined_text)
            } else {
                SyntaxNode::leaf(SyntaxKind::Text, child_new.text().to_string())
            }
        } else {
            let mut leaves: Vec<SyntaxNode> = Vec::new();
            let mut iter1 = child_old.children();
            let mut iter2 = child_new.children();

            loop {
                match (iter1.next(), iter2.next()) {
                    (Some(child1), Some(child2)) => {
                        // if _node_already_exists(child2, child_old.children()) && _node_already_exists(child1, child_new.children()){
                        //     leaves.push(child2.clone());
                        // } else
                        if _node_already_exists(child2, child_old.children()) && !_node_already_exists(child1, child_new.children()) {
                            let combined_child = find_difference_in_children(Some(child1), None);
                            leaves.push(combined_child);
                        } else if !_node_already_exists(child2, child_old.children()) && _node_already_exists(child1, child_new.children()) {
                            let combined_child = find_difference_in_children(None, Some(child2));
                            leaves.push(combined_child);
                        } else {
                            let combined_child = find_difference_in_children(Some(child1), Some(child2));
                            leaves.push(combined_child);
                        }
                    }
                    (Some(child1), None) => {
                        if _node_already_exists(child1, child_new.children()) && !_node_already_exists(child1, leaves.iter()) {
                            leaves.push(child1.clone());
                        } else {
                            let combined_child = find_difference_in_children(Some(child1), None);
                            leaves.push(combined_child);
                        }
                    }
                    (None, Some(child2)) => {
                        if _node_already_exists(child2, child_old.children()) && !_node_already_exists(child2, leaves.iter()) {
                            leaves.push(child2.clone());
                        } else {
                            let combined_child = find_difference_in_children(None, Some(child2));
                            leaves.push(combined_child);
                        }
                    }
                    (None, None) => {
                        break;
                    }
                }
            }
            SyntaxNode::inner(node_kind, leaves)
        }
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
            let combined_node = find_difference_in_children(Option::from(child1), Option::from(child2));
            nodes.push(combined_node);
        } else {
            nodes.push(child1.clone());
        }
    }
    SyntaxNode::inner(SyntaxKind::Auto, nodes)
}
