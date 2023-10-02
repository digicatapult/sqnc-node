#[derive(pest_derive::Parser)]
#[grammar = "dscp.pest"]
pub(crate) struct DscpParser;
