// pub mod nom_parser;
pub mod pest_parser;

#[derive(pest_derive::Parser)]
#[grammar = "rbs.pest"]
pub struct RbsParser;
