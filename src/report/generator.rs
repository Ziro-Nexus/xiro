// src/report/generator.rs
use proc_macro2::TokenStream;
use std::fmt::Debug;
use super::variable_parser::VariableParser;

#[derive(Debug)]
pub struct GeneratorReport {
    pub is_variable_declaration: bool,
    pub is_expression: bool,
    pub tokens: Option<TokenStream>,
}  

pub fn generate_syntax_report(code: &str) -> GeneratorReport {

    let mut report = GeneratorReport {
        is_variable_declaration: false,
        is_expression: false,
        tokens: None,
    };

    if let Ok(_variable_ast) = VariableParser::translate(code) {
        println!("Variable declaration detected.");
        report.tokens = Some(_variable_ast);
        report.is_variable_declaration = true;
        return report;
    }

    return report;
    
}
