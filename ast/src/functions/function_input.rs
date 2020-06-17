use crate::{
    ast::Rule,
    common::{Identifier, Mutable},
    types::Type,
};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::function_input))]
pub struct FunctionInput<'ast> {
    pub mutable: Option<Mutable>,
    pub identifier: Identifier<'ast>,
    pub _type: Type<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
