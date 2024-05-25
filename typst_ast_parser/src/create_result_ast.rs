use std::fs;
use std::slice::Iter;
use typst_syntax::{parse, SyntaxNode, SyntaxKind};


fn node_already_exists(node: &SyntaxNode, parent: Iter<SyntaxNode>) -> bool {
    for iter_node in parent {
        if iter_node == node {
            return true;
        }
    }
    false
}


fn is_some_kind_of_whitespace(node_kind: &SyntaxKind) -> bool {
    [SyntaxKind::Linebreak, SyntaxKind::Parbreak, SyntaxKind::Break, SyntaxKind::Hash, SyntaxKind::Space, SyntaxKind::None].contains(node_kind)
}


fn add_color_to_every_block(node: &SyntaxNode, color: &String) -> SyntaxNode {
    let node_kind: SyntaxKind = node.kind();
    let combined_text: String;

    if is_some_kind_of_whitespace(&node_kind) {
        node.clone()
    } else {
        if node_kind == SyntaxKind::FuncCall {
            combined_text = format!("text(fill: {})[#{}]", color, node.clone().into_text());
        } else {
            combined_text = format!("#text(fill: {})[{}]", color, node.clone().into_text());
        }
        SyntaxNode::leaf(node_kind, combined_text)
    }
}

fn find_difference_in_children(node1: Option<&SyntaxNode>, node2: Option<&SyntaxNode>, is_an_argument_value: bool) -> SyntaxNode {
    if node1.is_none() || node2.is_none() {
        let colored_node: SyntaxNode;
        if node1.is_none() {
            colored_node = add_color_to_every_block(node2.unwrap(), &"green".to_string());
        }
        else {
            colored_node = add_color_to_every_block(node1.unwrap(), &"red".to_string());
        }
        colored_node
    }
    else {
        let child_old: &SyntaxNode = node1.expect("The other file has different node");
        let child_new: &SyntaxNode = node2.expect("The other file has different node");
        let node_kind: SyntaxKind = child_new.kind();

        if child_old.children().count() == 0 && child_new.children().count() == 0 {
            if child_old.text() == child_new.text() || is_some_kind_of_whitespace(&child_new.kind()) {
                SyntaxNode::leaf(node_kind, child_new.text().to_string())
            } else {
                let mut combined_text = format!("#text(fill: red)[{}]#text(fill: green)[{}]", child_old.text(), child_new.text());

                if is_an_argument_value {
                    combined_text = combined_text.replace("\"", "");
                    combined_text = format!("[{}]", combined_text);
                }
                SyntaxNode::leaf(node_kind, combined_text)
            }
        } else {
            let mut leaves: Vec<SyntaxNode> = Vec::new();
            let mut iter1: Iter<'_, SyntaxNode> = child_old.children();
            let mut iter2: Iter<'_, SyntaxNode> = child_new.children();
            let is_function_argument: bool;
            if !is_an_argument_value {
                is_function_argument = child_new.kind() == SyntaxKind::Args;
            } else {
                is_function_argument = child_new.kind() != SyntaxKind::Markup;
            }

            loop {
                match (iter1.next(), iter2.next()) {
                    (Some(child1), Some(child2)) => {
                        // if _node_already_exists(child2, child_old.children()) && _node_already_exists(child1, child_new.children()){
                        //     leaves.push(child2.clone());
                        // } else
                        if node_already_exists(child2, child_old.children()) && !node_already_exists(child1, child_new.children()) {
                            let combined_child = find_difference_in_children(Some(child1), None, is_function_argument);
                            leaves.push(combined_child);
                        } else if !node_already_exists(child2, child_old.children()) && node_already_exists(child1, child_new.children()) {
                            let combined_child = find_difference_in_children(None, Some(child2), is_function_argument);
                            leaves.push(combined_child);
                        } else {
                            let combined_child = find_difference_in_children(Some(child1), Some(child2), is_function_argument);
                            leaves.push(combined_child);
                        }
                    }
                    (Some(child1), None) => {
                        if node_already_exists(child1, child_new.children()) && !node_already_exists(child1, leaves.iter()) {
                            leaves.push(child1.clone());
                        } else {
                            let combined_child = find_difference_in_children(Some(child1), None, is_function_argument);
                            leaves.push(combined_child);
                        }
                    }
                    (None, Some(child2)) => {
                        if node_already_exists(child2, child_old.children()) && !node_already_exists(child2, leaves.iter()) {
                            leaves.push(child2.clone());
                        } else {
                            let combined_child = find_difference_in_children(None, Some(child2), is_function_argument);
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
    let content1: String = fs::read_to_string(file_path1).expect("Couldn't read file");
    let content2: String = fs::read_to_string(file_path2).expect("Couldn't read file");

    // Parse the Typst file content into an AST
    let ast_tree1: SyntaxNode = parse(&content1);
    let ast_tree2: SyntaxNode = parse(&content2);

    let mut nodes: Vec<SyntaxNode> = Vec::new();

    let mut iter1: Iter<'_, SyntaxNode> = ast_tree1.children();
    let mut iter2: Iter<'_, SyntaxNode> = ast_tree2.children();
    // Use loop with match and next() to iterate through both trees
    loop {
        match (iter1.next(), iter2.next()) {
            // If both trees have a child node
            (Some(child1), Some(child2)) => {
                if child1 != child2 {
                    let combined_node: SyntaxNode = find_difference_in_children(Some(child1), Some(child2), false);
                    nodes.push(combined_node);
                } else {
                    nodes.push(child2.clone());
                }
            }
            (Some(child1), None) => {
                let combined_child: SyntaxNode = find_difference_in_children(Some(child1), None, false);
                nodes.push(combined_child);
            }
            (None, Some(child2)) => {
                let combined_child: SyntaxNode = find_difference_in_children(None, Some(child2), false);
                nodes.push(combined_child);
            }
            // If neither tree has a child node, exit the loop
            (None, None) => break,
        }
    }
    SyntaxNode::inner(SyntaxKind::Markup, nodes)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::create_ast_tree;

    #[test]
    fn test_add_bullet_point() {
        let path_to_old: String = "data/1_additional_bullet_point/no_bullet_point.typ".to_string();
        let path_to_new: String = "data/1_additional_bullet_point/added_bullet_point.typ".to_string();

        let result_ast_tree = create_ast_tree(&path_to_old, &path_to_new).into_text();

        let expected_content: String = fs::read_to_string("data/1_additional_bullet_point/expected_added_bullet_point.typ".to_string()).expect("Couldn't read file");

        assert_eq!(result_ast_tree.to_string(), expected_content);
    }

    #[test]
    fn test_remove_bullet_point() {
        let path_to_old: String = "data/2_delete_bullet_point/bullet_point.typ".to_string();
        let path_to_new: String = "data/2_delete_bullet_point/removed_bullet_point.typ".to_string();

        let result_ast_tree = create_ast_tree(&path_to_old, &path_to_new).into_text();

        let expected_content: String = fs::read_to_string("data/2_delete_bullet_point/expected_removed_bullet_point.typ".to_string()).expect("Couldn't read file");

        assert_eq!(result_ast_tree.to_string(), expected_content);
    }

    #[test]
    fn test_add_and_remove_bullet_point() {
        let path_to_old: String = "data/2_5_add_and_delete_bullet_point/bullet_point.typ".to_string();
        let path_to_new: String = "data/2_5_add_and_delete_bullet_point/added_and_deleted_bullet_points.typ".to_string();

        let result_ast_tree = create_ast_tree(&path_to_old, &path_to_new).into_text();

        let expected_content: String = fs::read_to_string("data/2_5_add_and_delete_bullet_point/expected_added_and_removed_bullet_point.typ".to_string()).expect("Couldn't read file");

        assert_eq!(result_ast_tree.to_string(), expected_content);
    }

    #[test]
    fn test_modify_bullet_point() {
        let path_to_old: String = "data/3_modify_bullet_point/bullet_point.typ".to_string();
        let path_to_new: String = "data/3_modify_bullet_point/modified_bullet_point.typ".to_string();

        let result_ast_tree = create_ast_tree(&path_to_old, &path_to_new).into_text();

        let expected_content: String = fs::read_to_string("data/3_modify_bullet_point/expected_modified_bullet_point.typ".to_string()).expect("Couldn't read file");

        assert_eq!(result_ast_tree.to_string(), expected_content);
    }

    #[test]
    fn test_bullet_point_switched_places() {
        let path_to_old: String = "data/4_bullet_points_switch_places/bullet_point.typ".to_string();
        let path_to_new: String = "data/4_bullet_points_switch_places/switched_bullet_point.typ".to_string();

        let result_ast_tree = create_ast_tree(&path_to_old, &path_to_new).into_text();

        let expected_content: String = fs::read_to_string("data/4_bullet_points_switch_places/expected_switched_bullet_point.typ".to_string()).expect("Couldn't read file");

        assert_eq!(result_ast_tree.to_string(), expected_content);
    }

    #[test]
    fn test_added_whole_paragraph() {
        let path_to_old: String = "data/5_add_paragraphs/one_paragraph.typ".to_string();
        let path_to_new: String = "data/5_add_paragraphs/add_paragraphs.typ".to_string();

        let result_ast_tree = create_ast_tree(&path_to_old, &path_to_new).into_text();

        let expected_content: String = fs::read_to_string("data/5_add_paragraphs/expected_added_paragraphs.typ".to_string()).expect("Couldn't read file");

        assert_eq!(result_ast_tree.to_string(), expected_content);
    }

    #[test]
    fn test_removed_whole_paragraph() {
        let path_to_old: String = "data/6_deleted_paragraph/two_paragraphs.typ".to_string();
        let path_to_new: String = "data/6_deleted_paragraph/deleted_paragraph.typ".to_string();

        let result_ast_tree = create_ast_tree(&path_to_old, &path_to_new).into_text();

        let expected_content: String = fs::read_to_string("data/6_deleted_paragraph/expected_deleted_paragraph.typ".to_string()).expect("Couldn't read file");

        assert_eq!(result_ast_tree.to_string(), expected_content);
    }
}