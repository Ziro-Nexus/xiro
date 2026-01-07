// src/report/syntax_report_handler.rs

use std::process::exit;
use evalexpr::eval;
use colored::*;
use tracing::{info, error, warn};

use crate::data_types::primitive_types;
use crate::utils::conversion_utils::convert_to_xiro_dt;
use super::super::memory_table::vartable::{Variable, VariableTableInMemory};
use super::generator::GeneratorReport;

pub struct ReportHandler;

impl ReportHandler {
    pub fn handle_report(rp: &GeneratorReport, vtm: &mut VariableTableInMemory) {
        if rp.is_variable_declaration {
            Self::handle_variable_declaration(rp, vtm);
        } else if rp.is_set_variable {
            Self::handle_set_variable(rp, vtm);
        }
    }

    fn handle_variable_declaration(rp: &GeneratorReport, vtm: &mut VariableTableInMemory) {
        let rc = rp.tokens.as_ref().unwrap();
        let name = rc.clone().into_iter().nth(0).expect("ERR: No name").to_string();
        let raw_value = rc.clone().into_iter().nth(3).expect("ERR: No value").to_string();

        let resolved_value = vtm.resolve_existing_identificators(raw_value.clone());

        match eval(&resolved_value) {
            Ok(eval_value) => {
                let datatype = convert_to_xiro_dt(eval_value);
                let mut new_var = Variable::new(name, datatype);
                new_var.resolve_value_type();
                vtm.push_var(new_var);
            }
            Err(e) => {
                error!("🛑 Evaluation Fault: Cannot declare variable '{}'", name);
                error!("📟 Source: {}", raw_value.bright_black());
                error!("⚠️  Reason: {}", e);
                //exit(1);
            }
        }
    }

    fn handle_set_variable(rp: &GeneratorReport, vtm: &mut VariableTableInMemory) {
        let rc = rp.tokens.as_ref().unwrap();
        let name = rc.clone().into_iter().nth(0).expect("ERR: No name").to_string();
        let raw_value = rc.clone().into_iter().nth(2).expect("ERR: No value").to_string();

        let resolved_value = vtm.resolve_existing_identificators(raw_value.clone());

        match eval(&resolved_value) {
            Ok(eval_value) => {
                let datatype = convert_to_xiro_dt(eval_value);
                let mut new_var = Variable::new(name, datatype);
                new_var.resolve_value_type();
                vtm.set_var(new_var);
            }
            Err(e) => {
                error!("💀 Memory Write Collision: Failed to set '{}'", name);
                error!("📟 Expression: {}", raw_value);
                //error!("⚠️ Reason: Tried to evaluate but got error: {}", e);
                //exit(1);
            }
        }
    }

    pub fn print_status_report(rp: &GeneratorReport) {
        let header = " XIRO SYNTAX REPORT ".bold().on_bright_black().white();
        let border = "═".repeat(40).bright_black();
        
        println!("\n{}", border);
        println!("{}", header);
        
        let status = if rp.is_variable_declaration { "DECLARATION" } 
                    else if rp.is_set_variable { "ASSIGNMENT" } 
                    else if rp.is_expression { "EXPRESSION" } 
                    else { "UNKNOWN" };

        println!(" 🧩 Type:      {}", status.cyan().bold());
        
        if let Some(tokens) = &rp.tokens {
            println!(" 📜 Buffer:    {}", tokens.to_string().italic().bright_white());
        }

        let mut flags = Vec::new();
        if rp.is_variable_declaration { flags.push("INIT"); }
        if rp.is_set_variable { flags.push("MUTATE"); }
        if rp.is_expression { flags.push("EVAL"); }
        
        println!(" ⚙️  Flags:     {}", flags.join(" | ").yellow());
        println!("{}\n", border);
    }
}