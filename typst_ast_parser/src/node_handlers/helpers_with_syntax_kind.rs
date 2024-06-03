use typst_syntax::SyntaxKind;

pub fn is_some_kind_of_whitespace(node_kind: &SyntaxKind) -> bool {
    [SyntaxKind::Linebreak, SyntaxKind::Parbreak, SyntaxKind::Break,
        SyntaxKind::Hash, SyntaxKind::Space, SyntaxKind::None,
        SyntaxKind::BlockComment, SyntaxKind::LineComment,
        SyntaxKind::Import, SyntaxKind::ImportItems, SyntaxKind::ModuleImport,
        SyntaxKind::RenamedImportItem, SyntaxKind::Ident].contains(node_kind)
}

pub fn is_some_kind_of_call(node_kind: &SyntaxKind) -> bool {
    [SyntaxKind::FuncCall, SyntaxKind::ShowRule, SyntaxKind::Show].contains(node_kind)
}


pub fn skip_syntax_kinds(node_kind: &SyntaxKind) -> bool {
    [SyntaxKind::LetBinding, SyntaxKind::Let].contains(node_kind)
}


pub fn is_function_argument_other_then_text(node_kind: SyntaxKind, previous_node_kind: SyntaxKind) -> bool {
    ![SyntaxKind::Str, SyntaxKind::ContentBlock].contains(&node_kind) && previous_node_kind == SyntaxKind::Named
}