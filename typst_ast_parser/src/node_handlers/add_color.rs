use typst_syntax::{SyntaxKind, SyntaxNode};

use crate::enums::custom_enums::NodeStatus;
use crate::node_handlers::helpers_with_syntax_kind::{is_some_kind_of_call, is_some_kind_of_whitespace};

pub fn add_color_to_every_block(node: &SyntaxNode, node_status: NodeStatus, previous_node_kind: SyntaxKind) -> SyntaxNode {
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
        }.to_string();

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

pub fn create_combined_text_diff(child_old: &SyntaxNode, child_new: &SyntaxNode, is_an_argument_value: bool) -> String {
    let mut combined_text: String = format!("#text(fill: red)[{}]#text(fill: green)[{}]", child_old.text(), child_new.text());

    if is_an_argument_value {
        combined_text = combined_text.replace("\"", "");
        combined_text = format!("[{}]", combined_text);
    }

    combined_text
}