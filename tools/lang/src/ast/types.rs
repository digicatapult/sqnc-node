use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

use pest::Span;

#[derive(Clone, PartialEq)]
pub struct AstNode<'a, V>
where
    V: 'a,
{
    pub(crate) value: V,
    pub(crate) span: Span<'a>,
}

impl<'a, V> Display for AstNode<'a, V>
where
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'a, V> Debug for AstNode<'a, V>
where
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AstNode")
            .field("value", &self.value)
            .field("span", &self.span)
            .finish()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenFieldType<'a> {
    None,
    File,
    Role,
    Literal,
    LiteralValue(AstNode<'a, &'a str>),
    Token(AstNode<'a, &'a str>),
}

impl<'a> Display for TokenFieldType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenFieldType::None => write!(f, "None"),
            TokenFieldType::File => write!(f, "File"),
            TokenFieldType::Role => write!(f, "Role"),
            TokenFieldType::Literal => write!(f, "Literal"),
            TokenFieldType::LiteralValue(s) => write!(f, "\"{}\"", s.value),
            TokenFieldType::Token(s) => write!(f, "{}", s.value),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TokenPropDecl<'a> {
    pub(crate) name: AstNode<'a, &'a str>,
    pub(crate) types: Vec<AstNode<'a, TokenFieldType<'a>>>,
}

impl<'a> Display for TokenPropDecl<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.name,
            self.types
                .iter()
                .map(|t| format!("{}", t))
                .collect::<Vec<_>>()
                .join(" | ")
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TokenDecl<'a> {
    pub(crate) name: AstNode<'a, &'a str>,
    pub(crate) props: AstNode<'a, Vec<AstNode<'a, TokenPropDecl<'a>>>>,
}

impl<'a> Display for TokenDecl<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "token {} {{\n{}\n}}",
            self.name,
            self.props
                .value
                .iter()
                .map(|p| format!("\t{}", p))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FnVis {
    Private,
    Public,
}

impl Display for FnVis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &FnVis::Public => write!(f, "pub"),
            &FnVis::Private => write!(f, "priv"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnArg<'a> {
    pub(crate) name: AstNode<'a, &'a str>,
    pub(crate) token_type: AstNode<'a, &'a str>,
}

impl<'a> Display for FnArg<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.token_type)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BoolOp {
    And,
    Or,
    Xor,
}
#[derive(Clone, Debug, PartialEq)]
pub enum BoolCmp {
    Eq,
    Neq,
}
#[derive(Clone, Debug, PartialEq)]
pub enum TypeCmp {
    Is,
    Isnt,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TokenProp<'a> {
    pub(crate) token: AstNode<'a, &'a str>,
    pub(crate) prop: AstNode<'a, &'a str>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Comparison<'a> {
    Fn {
        name: AstNode<'a, &'a str>,
        inputs: AstNode<'a, Vec<AstNode<'a, &'a str>>>,
        outputs: AstNode<'a, Vec<AstNode<'a, &'a str>>>,
    },
    PropLit {
        left: AstNode<'a, TokenProp<'a>>,
        op: AstNode<'a, BoolCmp>,
        right: AstNode<'a, &'a str>,
    },
    PropSender {
        left: AstNode<'a, TokenProp<'a>>,
        op: AstNode<'a, BoolCmp>,
    },
    TokenToken {
        left: AstNode<'a, &'a str>,
        op: AstNode<'a, BoolCmp>,
        right: AstNode<'a, &'a str>,
    },
    PropToken {
        left: AstNode<'a, TokenProp<'a>>,
        op: AstNode<'a, BoolCmp>,
        right: AstNode<'a, &'a str>,
    },
    PropProp {
        left: AstNode<'a, TokenProp<'a>>,
        op: AstNode<'a, BoolCmp>,
        right: AstNode<'a, TokenProp<'a>>,
    },
    PropType {
        left: AstNode<'a, TokenProp<'a>>,
        op: AstNode<'a, TypeCmp>,
        right: AstNode<'a, &'a str>,
    },
}

impl<'a> Display for Comparison<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Comparison::Fn { name, inputs, outputs } => {
                let inputs = inputs.value.iter().map(|i| i.value).collect::<Vec<_>>().join(", ");
                let outputs = outputs.value.iter().map(|i| i.value).collect::<Vec<_>>().join(", ");
                write!(f, "{} |{}| => |{}|", name, inputs, outputs)
            }
            Comparison::PropLit { left, op, right } => {
                let op = match op.value {
                    BoolCmp::Eq => "==",
                    BoolCmp::Neq => "!=",
                };
                write!(f, "{}.{} {} \"{}\"", left.value.token, left.value.prop, op, right)
            }
            Comparison::PropSender { left, op } => {
                let op = match op.value {
                    BoolCmp::Eq => "==",
                    BoolCmp::Neq => "!=",
                };
                write!(f, "{}.{} {} sender", left.value.token, left.value.prop, op)
            }
            Comparison::TokenToken { left, op, right } => {
                let op = match op.value {
                    BoolCmp::Eq => "==",
                    BoolCmp::Neq => "!=",
                };
                write!(f, "{} {} {}", left, op, right)
            }
            Comparison::PropToken { left, op, right } => {
                let op = match op.value {
                    BoolCmp::Eq => "==",
                    BoolCmp::Neq => "!=",
                };
                write!(f, "{}.{} {} {}", left.value.token, left.value.prop, op, right)
            }
            Comparison::PropProp { left, op, right } => {
                let op = match op.value {
                    BoolCmp::Eq => "==",
                    BoolCmp::Neq => "!=",
                };
                write!(
                    f,
                    "{}.{} {} {}.{}",
                    left.value.token, left.value.prop, op, right.value.token, right.value.prop
                )
            }
            Comparison::PropType { left, op, right } => {
                let op = match op.value {
                    TypeCmp::Is => ":",
                    TypeCmp::Isnt => "!:",
                };
                write!(f, "{}.{}{} {}", left.value.token, left.value.prop, op, right)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionTree<'a> {
    Leaf(AstNode<'a, Comparison<'a>>),
    Not(Box<ExpressionTree<'a>>),
    Node {
        left: Box<ExpressionTree<'a>>,
        op: BoolOp,
        right: Box<ExpressionTree<'a>>,
    },
}

impl<'a> Display for ExpressionTree<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionTree::Leaf(c) => write!(f, "{}", c),
            ExpressionTree::Not(e) => write!(f, "!({})", e),
            ExpressionTree::Node { left, op, right } => {
                let op_symbol = match op {
                    BoolOp::And => "&",
                    BoolOp::Xor => "^",
                    BoolOp::Or => "|",
                };
                write!(f, "({} {} {})", left, op_symbol, right)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnDecl<'a> {
    pub(crate) visibility: AstNode<'a, FnVis>,
    pub(crate) name: AstNode<'a, &'a str>,
    pub(crate) inputs: AstNode<'a, Arc<[AstNode<'a, FnArg<'a>>]>>,
    pub(crate) outputs: AstNode<'a, Arc<[AstNode<'a, FnArg<'a>>]>>,
    pub(crate) conditions: AstNode<'a, Vec<ExpressionTree<'a>>>,
}

fn format_fn_args(args: Vec<&FnArg>) -> String {
    match args.len() < 3 {
        true => format!(
            "|{}|",
            args.iter().map(|v| format!("{}", v)).collect::<Vec<_>>().join(", ")
        ),
        false => format!(
            "|\n{}\n|",
            args.iter().map(|v| format!("\t{}", v)).collect::<Vec<_>>().join("\n")
        ),
    }
}

fn format_conditions_list(conditions: Vec<&ExpressionTree>) -> String {
    format!(
        "{{\n{}\n}}",
        conditions
            .iter()
            .map(|v| format!("\t{},", v))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

impl<'a> Display for FnDecl<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} fn {} {} => {} where {}",
            self.visibility,
            self.name,
            format_fn_args(self.inputs.value.iter().map(|v| &v.value).collect()),
            format_fn_args(self.outputs.value.iter().map(|v| &v.value).collect()),
            format_conditions_list(self.conditions.value.iter().collect::<Vec<_>>())
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AstRoot<'a> {
    TokenDecl(AstNode<'a, TokenDecl<'a>>),
    FnDecl(AstNode<'a, FnDecl<'a>>),
}

impl<'a> Display for AstRoot<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AstRoot::TokenDecl(d) => write!(f, "{}", d),
            AstRoot::FnDecl(d) => write!(f, "{}", d),
        }
    }
}

pub type Ast<'a> = Vec<AstNode<'a, AstRoot<'a>>>;
