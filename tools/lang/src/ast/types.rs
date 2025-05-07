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
    Integer,
    LiteralValue(AstNode<'a, &'a str>),
    IntegerValue(AstNode<'a, i128>),
    Token(AstNode<'a, &'a str>),
}

impl<'a> Display for TokenFieldType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenFieldType::None => write!(f, "None"),
            TokenFieldType::File => write!(f, "File"),
            TokenFieldType::Role => write!(f, "Role"),
            TokenFieldType::Literal => write!(f, "Literal"),
            TokenFieldType::Integer => write!(f, "Integer"),
            TokenFieldType::LiteralValue(s) => write!(f, "\"{}\"", s.value),
            TokenFieldType::IntegerValue(s) => write!(f, "{}", s.value),
            TokenFieldType::Token(s) => write!(f, "{}", s.value),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TokenPropDecl<'a> {
    pub(crate) name: AstNode<'a, &'a str>,
    pub(crate) types: Arc<[AstNode<'a, TokenFieldType<'a>>]>,
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
    pub(crate) props: AstNode<'a, Arc<[AstNode<'a, TokenPropDecl<'a>>]>>,
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
    pub(crate) is_reference: bool,
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

impl Display for BoolCmp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            BoolCmp::Eq => "==",
            BoolCmp::Neq => "!=",
        };
        write!(f, "{}", op)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeCmp {
    Is,
    Isnt,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeCmpType {
    None,
    File,
    Role,
    Literal,
    Integer,
    Token,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RoleLit {
    Root,
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
        inputs: AstNode<'a, Arc<[AstNode<'a, &'a str>]>>,
        outputs: AstNode<'a, Arc<[AstNode<'a, &'a str>]>>,
    },
    PropLit {
        left: AstNode<'a, TokenProp<'a>>,
        op: BoolCmp,
        right: AstNode<'a, &'a str>,
    },
    PropInt {
        left: AstNode<'a, TokenProp<'a>>,
        op: BoolCmp,
        right: AstNode<'a, i128>,
    },
    PropSender {
        left: AstNode<'a, TokenProp<'a>>,
        op: BoolCmp,
    },
    SenderRole {
        op: BoolCmp,
        right: AstNode<'a, RoleLit>,
    },
    TokenToken {
        left: AstNode<'a, &'a str>,
        op: BoolCmp,
        right: AstNode<'a, &'a str>,
    },
    PropToken {
        left: AstNode<'a, TokenProp<'a>>,
        op: BoolCmp,
        right: AstNode<'a, &'a str>,
    },
    PropProp {
        left: AstNode<'a, TokenProp<'a>>,
        op: BoolCmp,
        right: AstNode<'a, TokenProp<'a>>,
    },
    PropType {
        left: AstNode<'a, TokenProp<'a>>,
        op: TypeCmp,
        right: AstNode<'a, TypeCmpType>,
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
                write!(f, "{}.{} {} \"{}\"", left.value.token, left.value.prop, op, right)
            }
            Comparison::PropInt { left, op, right } => {
                write!(f, "{}.{} {} {}", left.value.token, left.value.prop, op, right)
            }
            Comparison::PropSender { left, op } => {
                write!(f, "{}.{} {} sender", left.value.token, left.value.prop, op)
            }
            Comparison::TokenToken { left, op, right } => {
                write!(f, "{} {} {}", left, op, right)
            }
            Comparison::PropToken { left, op, right } => {
                write!(f, "{}.{} {} {}", left.value.token, left.value.prop, op, right)
            }
            Comparison::PropProp { left, op, right } => {
                write!(
                    f,
                    "{}.{} {} {}.{}",
                    left.value.token, left.value.prop, op, right.value.token, right.value.prop
                )
            }
            Comparison::PropType { left, op, right } => {
                let op = match op {
                    TypeCmp::Is => ":",
                    TypeCmp::Isnt => "!:",
                };
                let right = match right.value {
                    TypeCmpType::None => "None",
                    TypeCmpType::File => "File",
                    TypeCmpType::Role => "Role",
                    TypeCmpType::Literal => "Literal",
                    TypeCmpType::Integer => "Integer",
                    TypeCmpType::Token => "Token",
                };
                write!(f, "{}.{}{} {}", left.value.token, left.value.prop, op, right)
            }
            Comparison::SenderRole { op, right } => {
                let right = match right.value {
                    RoleLit::Root => "root",
                };
                write!(f, "sender {} {}", op, right)
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

fn format_fn_args(args: Arc<[&FnArg]>) -> String {
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
