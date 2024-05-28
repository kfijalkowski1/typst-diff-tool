use std::fs;
use std::slice::Iter;
use typst_syntax::{parse, SyntaxNode, SyntaxKind};
use std::collections::HashSet;
use crate::custom_enums::NodeStatus;

fn _node_exists(node: &SyntaxNode, parent: Iter<SyntaxNode>) -> bool {
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


fn skip_syntax_kinds(node_kind: &SyntaxKind) -> bool {
    [SyntaxKind::LetBinding, SyntaxKind::Let].contains(node_kind)
}


fn add_color_to_every_block(node: &SyntaxNode, node_status: NodeStatus, previous_node_kind: SyntaxKind) -> SyntaxNode {
    let node_kind: SyntaxKind = node.kind();

    if is_some_kind_of_whitespace(&node_kind) {
        node.clone()
    } else {
        let color: String;
        let content: String;
        let mut fill: String;

        if node_status == NodeStatus::ADDED {
            color = "green".to_string();
        } else if node_status == NodeStatus::DELETED {
            color = "red".to_string();
        } else if node_status == NodeStatus::MOVED {
            color = "yellow".to_string();
        }
        else {
            panic!("Invalid node_status passed. Allowed values: ADDED, DELETED, MOVED");
        }

        if node_kind == SyntaxKind::FuncCall {
            fill = format!("text(fill: {})", color);
            content = format!("[#{}]", node.clone().into_text());
            if previous_node_kind != SyntaxKind::Hash {
                fill = format!("#{}", fill);
            }
        }
        else {
            fill = format!("#text(fill: {})", color);
            content = format!("[{}]", node.clone().into_text());
        }
        SyntaxNode::leaf(node_kind, format!("{}{}", fill, content))
    }
}

fn find_difference_in_children(child_old: &SyntaxNode, child_new: &SyntaxNode, is_an_argument_value: bool) -> SyntaxNode {
    let node_kind: SyntaxKind = child_new.kind();

    if child_old.children().count() == 0 && child_new.children().count() == 0 {
        if child_old.text() == child_new.text() || is_some_kind_of_whitespace(&node_kind) || node_kind == SyntaxKind::Ident {
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

        if skip_syntax_kinds(&node_kind) {
            child_new.clone()
        } else {
            let mut prev_node_kind: SyntaxKind = SyntaxKind::None;
            loop {
                let combined_child: SyntaxNode;

                match (iter1.next(), iter2.next()) {
                    (Some(child1), Some(child2)) => {
                        if child1 != child2 {
                            combined_child = find_difference_in_children(child1, child2, is_function_argument);
                            prev_node_kind = combined_child.kind();
                            leaves.push(combined_child);
                        } else {
                            prev_node_kind = child2.kind();
                            leaves.push(child2.clone());
                        }
                    }
                    (Some(child1), None) => {
                        combined_child = add_color_to_every_block(child1, NodeStatus::DELETED, prev_node_kind);
                        leaves.push(combined_child);
                    }
                    (None, Some(child2)) => {
                        combined_child = add_color_to_every_block(child2, NodeStatus::ADDED, prev_node_kind);
                        leaves.push(combined_child);
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

    let nodes1: Vec<SyntaxNode> = ast_tree1.children().cloned().collect();
    let nodes2: Vec<SyntaxNode> = ast_tree2.children().cloned().collect();

    let added_in1st_nodes: HashSet<SyntaxNode> = nodes1.iter().cloned().filter(|node| !nodes2.contains(node)).collect();
    let added_in2nd_nodes: HashSet<SyntaxNode> = nodes2.iter().cloned().filter(|node| !nodes1.contains(node)).collect();

    let mut nodes: Vec<SyntaxNode> = Vec::new();

    let len1 = nodes1.len();
    let len2 = nodes2.len();
    let mut i = 0;
    let mut j = 0;

    let mut new_node: SyntaxNode;
    let mut prev_node_king: SyntaxKind = SyntaxKind::None;
    while i < len1 || j < len2 {
        if i < len1 && j < len2 && added_in1st_nodes.contains(&nodes1[i]) && added_in2nd_nodes.contains(&nodes2[j]) {
            new_node = find_difference_in_children(&nodes1[i], &nodes2[j], false);
            prev_node_king = new_node.kind();
            nodes.push(new_node); // Modified case
            if i < len1 { i += 1; }
            if j < len2 { j += 1; }
        } else if i < len1 && added_in1st_nodes.contains(&nodes1[i]) {
            new_node = add_color_to_every_block(&nodes1[i], NodeStatus::DELETED, prev_node_king); // Deleted case
            prev_node_king = new_node.kind();
            nodes.push(new_node);
            if i < len1 { i += 1; }
        } else if j < len2 && added_in2nd_nodes.contains(&nodes2[j]) {
            new_node = add_color_to_every_block(&nodes2[j], NodeStatus::ADDED, prev_node_king); // Added case
            prev_node_king = new_node.kind();
            nodes.push(new_node);
            if j < len2 { j += 1; }
        } else if i < len1 && j < len2 && nodes1[i] == nodes2[j] {
            prev_node_king = nodes2[j].kind();
            nodes.push(nodes2[j].clone()); // Same case
            if i < len1 { i += 1; }
            if j < len2 { j += 1; }
        } else if i < len1 && j < len2 && nodes1[i] != nodes2[j] {
            new_node = add_color_to_every_block(&nodes1[i], NodeStatus::MOVED, prev_node_king); // Moved case
            prev_node_king = new_node.kind();
            nodes.push(new_node);
            if i < len1 { i += 1; }
        } else {
            // Ensure at least one of i or j is incremented to avoid infinite loop
            if i < len1 { i += 1; }
            if j < len2 { j += 1; }
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