// src/report/generator.rs
use proc_macro2::TokenStream;
use tracing::error;
use std::{fmt::Debug, process::exit};
use crate::report::set_variable_parser::SetVariableParser;

use super::variable_parser::VariableParser;

#[derive(Debug)]
pub struct GeneratorReport {
    pub is_variable_declaration: bool,
    pub is_set_variable: bool,
    pub is_expression: bool,
    pub tokens: Option<TokenStream>,
}  

pub fn generate_syntax_report(code: &str) -> GeneratorReport {

    let mut report = GeneratorReport {
        is_variable_declaration: false,
        is_set_variable: false,
        is_expression: false,
        tokens: None,
    };

    if let Ok(_variable_ast) = VariableParser::translate(code) {
        report.tokens = Some(_variable_ast);
        report.is_variable_declaration = true;
        return report;
    }

    if let Ok(_set_variable_ast) = SetVariableParser::translate(code) {
        report.tokens = Some(_set_variable_ast);
        report.is_set_variable = true;
        return  report;
    }

    error!("⚠️  Unrecognized syntax: '{}'", code);
    exit(1);
    
}
