    use std::collections::HashSet;

use typst_syntax::{SyntaxKind, SyntaxNode};

use crate::enums::custom_enums::NodeStatus;
use crate::node_handlers::add_color::add_color_to_every_block;
use crate::node_handlers::node_comparer::find_difference_in_children;
use crate::typst_handlers::typst_parser::read_typst_file;
use anyhow::{Result};

pub(crate) fn create_diff_ast_tree(file_path1: &String, file_path2: &String) -> Result<SyntaxNode> {
    let ast_tree1: SyntaxNode = read_typst_file(file_path1)?;
    let ast_tree2: SyntaxNode = read_typst_file(file_path2)?;

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
    let mut prev_node_kind: SyntaxKind = SyntaxKind::None;
    while i < len1 || j < len2 {
        if i < len1 && j < len2 && added_in1st_nodes.contains(&nodes1[i]) && added_in2nd_nodes.contains(&nodes2[j]) {
            if nodes1[i].kind() == nodes2[j].kind() {
                new_node = find_difference_in_children(&nodes1[i], &nodes2[j], false, prev_node_kind);
                prev_node_kind = new_node.kind();
                nodes.push(new_node); // Modified case
            } else {
                let deleted_node = add_color_to_every_block(&nodes1[i], NodeStatus::DELETED, prev_node_kind);
                let added_node = add_color_to_every_block(&nodes2[j], NodeStatus::ADDED, prev_node_kind);
                nodes.push(deleted_node);
                nodes.push(added_node);
            }
            if i < len1 { i += 1; }
            if j < len2 { j += 1; }
        } else if i < len1 && added_in1st_nodes.contains(&nodes1[i]) {
            new_node = add_color_to_every_block(&nodes1[i], NodeStatus::DELETED, prev_node_kind); // Deleted case
            prev_node_kind = new_node.kind();
            nodes.push(new_node);
            if i < len1 { i += 1; }
        } else if j < len2 && added_in2nd_nodes.contains(&nodes2[j]) {
            new_node = add_color_to_every_block(&nodes2[j], NodeStatus::ADDED, prev_node_kind); // Added case
            prev_node_kind = new_node.kind();
            nodes.push(new_node);
            if j < len2 { j += 1; }
        } else if i < len1 && j < len2 && nodes1[i] == nodes2[j] {
            prev_node_kind = nodes2[j].kind();
            nodes.push(nodes2[j].clone()); // Same case
            if i < len1 { i += 1; }
            if j < len2 { j += 1; }
        } else if i < len1 && j < len2 && nodes1[i] != nodes2[j] {
            new_node = add_color_to_every_block(&nodes1[i], NodeStatus::MOVED, prev_node_kind); // Moved case
            prev_node_kind = new_node.kind();
            nodes.push(new_node);
            if i < len1 { i += 1; }
        } else {
            // Ensure at least one of i or j is incremented to avoid infinite loop
            if i < len1 { i += 1; }
            if j < len2 { j += 1; }
        }
    }

    Ok(SyntaxNode::inner(SyntaxKind::Markup, nodes))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::create_diff_ast_tree;

    #[test]
    fn test_add_bullet_point() {
        let path_to_old: String = "data/1_additional_bullet_point/no_bullet_point.typ".to_string();
        let path_to_new: String = "data/1_additional_bullet_point/added_bullet_point.typ".to_string();

        let result_ast_tree = create_diff_ast_tree(&path_to_old, &path_to_new).expect("EXPECT").into_text();

        let expected_content: String = fs::read_to_string("data/1_additional_bullet_point/expected_added_bullet_point.typ".to_string()).expect("Couldn't read file");

        assert_eq!(result_ast_tree.to_string(), expected_content);
    }

    #[test]
    fn test_remove_bullet_point() {
        let path_to_old: String = "data/2_delete_bullet_point/bullet_point.typ".to_string();
        let path_to_new: String = "data/2_delete_bullet_point/removed_bullet_point.typ".to_string();

        let result_ast_tree = create_diff_ast_tree(&path_to_old, &path_to_new).expect("").into_text();

        let expected_content: String = fs::read_to_string("data/2_delete_bullet_point/expected_removed_bullet_point.typ".to_string()).expect("Couldn't read file");

        assert_eq!(result_ast_tree.to_string(), expected_content);
    }

    #[test]
    fn test_add_and_remove_bullet_point() {
        let path_to_old: String = "data/2_5_add_and_delete_bullet_point/bullet_point.typ".to_string();
        let path_to_new: String = "data/2_5_add_and_delete_bullet_point/added_and_deleted_bullet_points.typ".to_string();

        let result_ast_tree = create_diff_ast_tree(&path_to_old, &path_to_new).expect("").into_text();

        let expected_content: String = fs::read_to_string("data/2_5_add_and_delete_bullet_point/expected_added_and_removed_bullet_point.typ".to_string()).expect("Couldn't read file");

        assert_eq!(result_ast_tree.to_string(), expected_content);
    }

    #[test]
    fn test_modify_bullet_point() {
        let path_to_old: String = "data/3_modify_bullet_point/bullet_point.typ".to_string();
        let path_to_new: String = "data/3_modify_bullet_point/modified_bullet_point.typ".to_string();

        let result_ast_tree = create_diff_ast_tree(&path_to_old, &path_to_new).expect("").into_text();

        let expected_content: String = fs::read_to_string("data/3_modify_bullet_point/expected_modified_bullet_point.typ".to_string()).expect("Couldn't read file");

        assert_eq!(result_ast_tree.to_string(), expected_content);
    }

    #[test]
    fn test_bullet_point_switched_places() {
        let path_to_old: String = "data/4_bullet_points_switch_places/bullet_point.typ".to_string();
        let path_to_new: String = "data/4_bullet_points_switch_places/switched_bullet_point.typ".to_string();

        let result_ast_tree = create_diff_ast_tree(&path_to_old, &path_to_new).expect("").into_text();

        let expected_content: String = fs::read_to_string("data/4_bullet_points_switch_places/expected_switched_bullet_point.typ".to_string()).expect("Couldn't read file");

        assert_eq!(result_ast_tree.to_string(), expected_content);
    }

    #[test]
    fn test_added_whole_paragraph() {
        let path_to_old: String = "data/5_add_paragraphs/one_paragraph.typ".to_string();
        let path_to_new: String = "data/5_add_paragraphs/add_paragraphs.typ".to_string();

        let result_ast_tree = create_diff_ast_tree(&path_to_old, &path_to_new).expect("").into_text();

        let expected_content: String = fs::read_to_string("data/5_add_paragraphs/expected_added_paragraphs.typ".to_string()).expect("Couldn't read file");

        assert_eq!(result_ast_tree.to_string(), expected_content);
    }

    #[test]
    fn test_removed_whole_paragraph() {
        let path_to_old: String = "data/6_deleted_paragraph/two_paragraphs.typ".to_string();
        let path_to_new: String = "data/6_deleted_paragraph/deleted_paragraph.typ".to_string();

        let result_ast_tree = create_diff_ast_tree(&path_to_old, &path_to_new).expect("").into_text();

        let expected_content: String = fs::read_to_string("data/6_deleted_paragraph/expected_deleted_paragraph.typ".to_string()).expect("Couldn't read file");

        assert_eq!(result_ast_tree.to_string(), expected_content);
    }
}