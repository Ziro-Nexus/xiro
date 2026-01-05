// src/main.rs
// Xiro scripting interpreted language

mod utils;
mod report;
mod data_types;
mod memory_table;
mod plugins;

use report::generator::generate_syntax_report;
use report::syntax_report_handler::ReportHandler;
use memory_table::vartable::VariableTableInMemory;

fn main() {

    let var0 = generate_syntax_report("def a = (2 + 2, 2, 2, (3, 3 + 2))");
    let var1 = generate_syntax_report("def a = 56");
    let mut vtm = VariableTableInMemory::new();
    ReportHandler::handle_report(&var0, &mut vtm);
    ReportHandler::handle_report(&var1, &mut vtm);
    ReportHandler::print_status_report(&var0);
    ReportHandler::print_status_report(&var1);

}
