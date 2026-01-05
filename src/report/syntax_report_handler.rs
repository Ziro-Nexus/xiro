// src/report/syntax_report_handler.rs

use core::panic;

use evalexpr::eval;
use syn::Expr;
use syn::spanned::Spanned;

use crate::data_types::primitive_types::DataTypes;
use crate::utils::extraction_utils::extract_inner_tuples;

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

    fn convert_to_xiro(val: evalexpr::Value) -> DataTypes {
        match val {
            evalexpr::Value::Int(i) => DataTypes::NUMBER(i),
            evalexpr::Value::Float(f) => DataTypes::FLOAT(f),
            evalexpr::Value::Boolean(b) => DataTypes::BOOL(b),
            evalexpr::Value::String(s) => DataTypes::STR(s),
            evalexpr::Value::Tuple(v) => {
                // ¡Aquí ocurre la magia! Mapeamos cada elemento
                // llamando a esta misma función.
                let inner_list = v.into_iter().map(Self::convert_to_xiro).collect();
                DataTypes::LIST(inner_list)
            }
            _ => DataTypes::STR(val.to_string()),
        }
    }

    fn handle_variable_declaration(rp: &GeneratorReport, vmt: &mut VariableTableInMemory) {
        let rc = rp.tokens.as_ref().unwrap();
        let name = rc.clone().into_iter().nth(1).expect("No name").to_string();
        let raw_value = rc.clone().into_iter().nth(3).expect("No value").to_string();

        println!(
            "Variable Declaration - Name: {}, Value: {}",
            name, raw_value
        );

        // Evaluamos la expresión una sola vez
        match eval(&raw_value) {
            Ok(eval_value) => {
                // Usamos nuestra función recursiva
                let datatype = Self::convert_to_xiro(eval_value);

                // Aquí lo guardas en tu tabla (vmt.add o similar)
                // vmt.variable_list.push(Variable { name, value: datatype, ... });
                println!("Variable añadida con éxito: {:#?}", datatype);
            }
            Err(e) => panic!("Error evaluando '{}': {}", raw_value, e),
        }
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
