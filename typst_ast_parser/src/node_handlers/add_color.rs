use similar::{ChangeTag, TextDiff};
use typst_syntax::{SyntaxKind, SyntaxNode};

use crate::enums::custom_enums::NodeStatus;
use crate::node_handlers::helpers_with_syntax_kind::{
    is_some_kind_of_call, is_some_kind_of_whitespace,
};

pub fn add_color_to_every_block(
    node: &SyntaxNode,
    node_status: NodeStatus,
    previous_node_kind: SyntaxKind,
) -> SyntaxNode {
    let node_kind: SyntaxKind = node.kind();

    if is_some_kind_of_whitespace(&node_kind) {
        node.clone()
    } else {
        let content: String;
        let mut fill: String;

        let color: String = match node_status {
            NodeStatus::ADDED => "green",
            NodeStatus::DELETED => "red",
            NodeStatus::MOVED => "yellow",
        }
        .to_string();

        if is_some_kind_of_call(&node_kind) {
            fill = format!("text(fill: {})", color);
            content = format!("[#{}]", node.clone().into_text());
            if previous_node_kind != SyntaxKind::Hash {
                fill = format!("#{}", fill);
            }
        } else {
            fill = format!("#text(fill: {})", color);
            content = format!("[{}]", node.clone().into_text());
        }
        SyntaxNode::leaf(node_kind, format!("{}{}", fill, content))
    }
}

pub fn create_combined_text_diff(
    child_old: &SyntaxNode,
    child_new: &SyntaxNode,
    is_an_argument_value: bool,
) -> String {
    let old_text = child_old.text().to_string();
    let new_text = child_new.text().to_string();

    let diff = TextDiff::from_lines(&old_text, &new_text);

    let mut node_res: String = "".to_string();

    for change in diff.iter_all_changes() {
        let new_text = match change.tag() {
            ChangeTag::Delete => {
                if let Some(child_str) = change.as_str() {
                    color_text(child_str, "red".to_string())
                } else {
                    "".to_string()
                }
            }
            ChangeTag::Insert => {
                if let Some(child_str) = change.as_str() {
                    color_text(child_str, "green".to_string())
                } else {
                    "".to_string()
                }
            }

            ChangeTag::Equal => child_new.text().to_string(),
        };

        node_res.push_str(&new_text);
    }
    if is_an_argument_value {
        node_res = node_res.replace('"', "");
        node_res = format!("[{}]", node_res);
    }

    node_res
}

fn color_text(child: &str, color: String) -> String {
    let combined_text = format!("#text(fill: {})[{}]", color, child);

    combined_text
}
