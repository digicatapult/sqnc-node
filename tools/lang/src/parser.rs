#[derive(pest_derive::Parser)]
#[grammar = "sqnc.pest"]
pub(crate) struct DscpParser;
