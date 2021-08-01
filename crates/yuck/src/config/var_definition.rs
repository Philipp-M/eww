use std::collections::HashMap;

use simplexpr::{dynval::DynVal, SimplExpr};

use crate::{error::{AstResult, AstResultExt}, parser::{
        ast::Ast,
        ast_iterator::AstIterator,
        from_ast::{FromAst, FromAstElementContent},
    }};
use eww_shared_util::{AttrName, Span, VarName};

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize)]
pub struct VarDefinition {
    pub name: VarName,
    pub initial_value: DynVal,
    pub span: Span,
}

impl FromAstElementContent for VarDefinition {
    fn get_element_name() -> &'static str {
        "defvar"
    }

    fn from_tail<I: Iterator<Item = Ast>>(span: Span, mut iter: AstIterator<I>) -> AstResult<Self> {
        let result: AstResult<_> = try {
            let (_, name) = iter.expect_symbol()?;
            let (_, initial_value) = iter.expect_literal()?;
            iter.expect_done()?;
            Self { name: VarName(name), initial_value, span }
        };
        result.note(r#"Expected format: `(defvar name "initial-value")`"#)
    }
}
