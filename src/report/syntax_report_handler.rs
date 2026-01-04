// src/report/syntax_report_handler.rs

use core::panic;

use evalexpr::eval;
use syn::Expr;
use syn::spanned::Spanned;

use crate::data_types::primitive_types::DataTypes;

use super::super::memory_table::vartable::Variable;
use super::super::memory_table::vartable::VariableTableInMemory;
use super::super::plugins::primitive_plugins::PRIMITIVEPLUGINS;
use super::generator::GeneratorReport;

pub struct ReportHandler;

impl ReportHandler {
    pub fn handle_report(rp: &GeneratorReport, vtm: &mut VariableTableInMemory) {
        if rp.is_variable_declaration {
            println!("Handling variable declaration report...");
            Self::handle_variable_declaration(rp, vtm);
        }
    }

    fn handle_variable_declaration(rp: &GeneratorReport, vmt: &mut VariableTableInMemory) {
        let rc = rp.tokens.as_ref().unwrap();
        let name = rc.clone().into_iter().nth(1).unwrap().to_string();
        let type_value = rc.clone().into_iter().nth(3).unwrap().to_string();
        let arguments_value = type_value.clone();
        println!(
            "Variable Declaration - Name: {}, Value: {}",
            name, type_value
        );

        let datatype: DataTypes;
        // resolve possible expr
        let type_value = eval(&type_value);

        match type_value {
            Ok(matched_type_value) => match matched_type_value {
                evalexpr::Value::Int(i) => {
                    datatype = DataTypes::NUMBER(i);
                }
                evalexpr::Value::Float(f) => {
                    datatype = DataTypes::FLOAT(f);
                }
                evalexpr::Value::Boolean(b) => {
                    datatype = DataTypes::BOOL(b);
                }
                evalexpr::Value::String(s) => {
                    datatype = DataTypes::STR(s);
                }
                evalexpr::Value::Tuple(_) => {
                    let arguments_value = arguments_value.replace("(", "").replace(")", "");
                    let items: Vec<&str> = arguments_value.split(',').collect();
                    let mut data_list: Vec<DataTypes> = Vec::new();
                    for item in items {
                        let trimmed_item = item.trim();
                        let eval_item = eval(trimmed_item);
                        match eval_item {
                            Ok(ev) => match ev {
                                evalexpr::Value::Int(i) => {
                                    data_list.push(DataTypes::NUMBER(i));
                                }
                                evalexpr::Value::Float(f) => {
                                    data_list.push(DataTypes::FLOAT(f));
                                }
                                evalexpr::Value::Boolean(b) => {
                                    data_list.push(DataTypes::BOOL(b));
                                }
                                evalexpr::Value::String(s) => {
                                    data_list.push(DataTypes::STR(s));
                                }
                                _ => {
                                    data_list.push(DataTypes::STR(ev.to_string()));
                                }
                            },
                            Err(_) => {
                                data_list.push(DataTypes::STR(trimmed_item.to_string()));
                            }
                        }
                    }
                    datatype = DataTypes::LIST(data_list);
                }
                _ => {
                    panic!("Syntax Error");
                }
            },
            Err(_) => {
                panic!("Error evaluating expression for variable declaration");
            }
        }

        let mut new_var = Variable::new(name, datatype);
        new_var.resolve_value_type();
        vmt.push_var(new_var);
        println!(
            "Variable added to Variable Table in Memory. {}",
            format!("{:#?}", vmt)
        );
    }

    pub fn print_status_report(rp: &GeneratorReport) {
        println!("--- Syntax Report ---");
        println!("Is Variable Declaration: {}", rp.is_variable_declaration);
        println!("Is Expression: {}", rp.is_expression);
        match &rp.tokens {
            Some(tokens) => println!("Tokens: {}", tokens.to_string()),
            None => println!("Tokens: None"),
        }
        println!("---------------------");
    }
}
