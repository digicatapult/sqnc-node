use pest::{Parser, Span};
use std::{env, fmt, fs, io};

#[derive(pest_derive::Parser)]
#[grammar = "dscp.pest"]
pub struct DscpParser;

#[derive(Debug)]
pub struct AstNode<'a, V> {
    value: V,
    span: Span<'a>,
}

impl<'a, V> AstNode<'a, V> {
    fn into(self) -> V {
        self.value
    }
}

pub struct AstBuildError<'a> {
    message: &'a str,
    span: Span<'a>,
}

impl<'a> fmt::Display for AstBuildError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

impl<'a> fmt::Debug for AstBuildError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

#[derive(Debug)]
pub enum TokenFieldType<'a> {
    None,
    File,
    Role,
    Literal,
    LiteralValue(AstNode<'a, &'a str>),
    Token(AstNode<'a, &'a str>),
}

#[derive(Debug)]
pub struct TokenProp<'a> {
    name: AstNode<'a, &'a str>,
    types: Vec<AstNode<'a, TokenFieldType<'a>>>,
}

#[derive(Debug)]
pub struct TokenDecl<'a> {
    name: AstNode<'a, &'a str>,
    props: AstNode<'a, Vec<AstNode<'a, TokenProp<'a>>>>,
}

#[derive(Debug)]
pub enum FnVis {
    private,
    public,
}

#[derive(Debug)]
pub struct FnDecl<'a> {
    visibility: AstNode<'a, FnVis>,
    name: AstNode<'a, &'a str>,
}

#[derive(Debug)]
pub enum AstRoot<'a> {
    TokenDecl(AstNode<'a, TokenDecl<'a>>),
    FnDecl(AstNode<'a, FnDecl<'a>>),
}

fn parse_token_prop_type(pair: pest::iterators::Pair<Rule>) -> AstNode<TokenFieldType> {
    AstNode {
        span: pair.as_span(),
        value: match pair.as_rule() {
            Rule::file => TokenFieldType::File,
            Rule::literal => TokenFieldType::Literal,
            Rule::role => TokenFieldType::Role,
            Rule::none => TokenFieldType::None,
            Rule::literal_value => {
                let literal_string = pair.into_inner().next().unwrap();
                let span = literal_string.as_span();
                let literal_val = literal_string.into_inner();
                TokenFieldType::LiteralValue(AstNode {
                    value: literal_val.as_str(),
                    span,
                })
            }
            Rule::ident => TokenFieldType::Token(AstNode {
                span: pair.as_span(),
                value: pair.into_inner().as_str(),
            }),
            _ => panic!(),
        },
    }
}

fn parse_token_prop_field(pair: pest::iterators::Pair<Rule>) -> AstNode<TokenProp> {
    AstNode {
        span: pair.as_span(),
        value: match pair.as_rule() {
            Rule::field => {
                let mut pair = pair.into_inner();
                let name = pair.next().unwrap();
                let valid_types = pair
                    .next()
                    .unwrap()
                    .into_inner()
                    .into_iter()
                    .map(parse_token_prop_type)
                    .collect::<Vec<_>>();
                TokenProp {
                    name: AstNode {
                        value: name.as_str(),
                        span: name.as_span(),
                    },
                    types: valid_types,
                }
            }
            _ => panic!(),
        },
    }
}

fn parse_token_props(pair: pest::iterators::Pair<Rule>) -> AstNode<Vec<AstNode<TokenProp>>> {
    AstNode {
        span: pair.as_span(),
        value: match pair.as_rule() {
            Rule::properties => pair
                .into_inner()
                .into_iter()
                .map(parse_token_prop_field)
                .collect::<Vec<_>>(),
            _ => panic!("Unexpected rule {:?}", pair),
        },
    }
}

fn parse_token_decl(pair: pest::iterators::Pair<Rule>) -> AstNode<TokenDecl> {
    match pair.as_rule() {
        Rule::struct_decl => {
            let span = pair.as_span();
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap();
            let props = parse_token_props(pair.next().unwrap());

            AstNode {
                span,
                value: TokenDecl {
                    name: AstNode {
                        value: name.as_str(),
                        span: name.as_span(),
                    },
                    props,
                },
            }
        }
        _ => panic!("Unexpected rule {}", pair.as_str()),
    }
}

fn parse_fn_decl(pair: pest::iterators::Pair<Rule>) -> AstNode<FnDecl> {
    let span = pair.as_span();
    match pair.as_rule() {
        Rule::fn_decl => {
            let mut pair = pair.into_inner();
            let vis = pair.next().unwrap();
            let name = pair.next().unwrap();
            AstNode {
                value: FnDecl {
                    visibility: AstNode {
                        value: match vis.as_str() {
                            "" => FnVis::private,
                            "priv" => FnVis::private,
                            "pub" => FnVis::public,
                            _ => panic!(),
                        },
                        span: vis.as_span(),
                    },
                    name: AstNode {
                        value: name.as_str(),
                        span: name.as_span(),
                    },
                },
                span,
            }
        }
        _ => panic!("Unexpected rule {}", pair.as_str()),
    }
}

fn main() -> io::Result<()> {
    let file_path = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(file_path)?;

    let pairs = DscpParser::parse(Rule::main, &contents);
    if let Err(e) = pairs {
        eprintln!("Parse failed: {:?}", e);
        return Ok(());
    }
    let pairs = pairs.unwrap();

    let ast: Vec<AstNode<AstRoot>> = pairs
        .into_iter()
        .filter_map(|pair| match pair.as_rule() {
            Rule::struct_decl => {
                let span = pair.as_span();
                Some(AstNode {
                    span,
                    value: AstRoot::TokenDecl(parse_token_decl(pair)),
                })
            }
            Rule::fn_decl => {
                let fn_span = pair.as_span();
                Some(AstNode {
                    span: fn_span,
                    value: AstRoot::FnDecl(parse_fn_decl(pair)),
                })
            }
            Rule::EOI => None,
            _ => panic!(),
        })
        .collect();

    println!("{:?}", ast);

    Ok(())
}
