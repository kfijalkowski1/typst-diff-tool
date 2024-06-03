use std::slice::Iter;

use typst_syntax::{SyntaxKind, SyntaxNode};

use crate::enums::custom_enums::NodeStatus;
use crate::node_handlers::add_color::{add_color_to_every_block, create_combined_text_diff};
use crate::node_handlers::helpers_with_syntax_kind::{is_some_kind_of_whitespace, skip_syntax_kinds};

pub fn find_difference_in_children(child_old: &SyntaxNode, child_new: &SyntaxNode, is_an_argument_value: bool) -> SyntaxNode {
    let node_kind: SyntaxKind = child_new.kind();

    if is_some_kind_of_whitespace(&node_kind) {
        child_new.clone()
    } else {
        if child_old.children().count() == 0 && child_new.children().count() == 0 {
            if child_old.text() == child_new.text() {
                SyntaxNode::leaf(node_kind, child_new.text().to_string())
            } else {
                let combined_text: String = create_combined_text_diff(child_old, child_new, is_an_argument_value);
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
}