use pest::Span;

use super::*;

#[derive(Debug)]
pub struct AstNode<'a, V> {
    pub(crate) value: V,
    pub(crate) span: Span<'a>
}

#[derive(Debug)]
pub enum TokenFieldType<'a> {
    None,
    File,
    Role,
    Literal,
    LiteralValue(AstNode<'a, &'a str>),
    Token(AstNode<'a, &'a str>)
}

#[derive(Debug)]
pub struct TokenProp<'a> {
    pub(crate) name: AstNode<'a, &'a str>,
    pub(crate) types: Vec<AstNode<'a, TokenFieldType<'a>>>
}

#[derive(Debug)]
pub struct TokenDecl<'a> {
    pub(crate) name: AstNode<'a, &'a str>,
    pub(crate) props: AstNode<'a, Vec<AstNode<'a, TokenProp<'a>>>>
}

#[derive(Debug)]
pub enum FnVis {
    Private,
    Public
}

#[derive(Debug)]
pub struct FnDecl<'a> {
    pub(crate) visibility: AstNode<'a, FnVis>,
    pub(crate) name: AstNode<'a, &'a str>
}

#[derive(Debug)]
pub enum AstRoot<'a> {
    TokenDecl(AstNode<'a, TokenDecl<'a>>),
    FnDecl(AstNode<'a, FnDecl<'a>>)
}
