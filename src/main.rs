// src/main.rs
// Xiro DSL language

mod report;
mod data_types;
mod memory_table;
mod plugins;

use report::generator::generate_syntax_report;
use report::syntax_report_handler::ReportHandler;
use memory_table::vartable::VariableTableInMemory;

fn main() {
    let var1 = generate_syntax_report("def a = (\"hello\" + \" world\")");
    let var2 = generate_syntax_report("def b = (1 + 2, 3 * 4, 500 - 200)");
    let mut vtm = VariableTableInMemory::new();
    ReportHandler::handle_report(&var1, &mut vtm);
    ReportHandler::handle_report(&var2, &mut vtm);
}
