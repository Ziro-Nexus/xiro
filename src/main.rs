// src/main.rs
// Xiro DSL

mod data_types;
mod memory_table;
mod plugins;
mod report;
mod utils;

use memory_table::vartable::VariableTableInMemory;
use report::generator::generate_syntax_report;
use report::syntax_report_handler::ReportHandler;

use crate::utils::telemetry::init_xiro_telemetry;

fn main() {
    init_xiro_telemetry();
    let mut vtm = VariableTableInMemory::new();

    let mut buffer = String::new();

    loop {
        print!("XS>> ");
        std::io::Write::flush(&mut std::io::stdout()).expect("Flush failed!");
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        let syntax_report = generate_syntax_report(&buffer.trim());
        ReportHandler::handle_report(&syntax_report, &mut vtm);
        ReportHandler::print_status_report(&syntax_report);

        println!("Output:");

        for r in vtm.get_table().iter() {
            match r.get_value() {
                data_types::primitive_types::DataTypes::NUMBER(n) => println!("{:?}", n),
                data_types::primitive_types::DataTypes::BOOL(b) => println!("{:?}", b),
                data_types::primitive_types::DataTypes::FLOAT(f) => println!("{:?}", f),
                data_types::primitive_types::DataTypes::STR(s) => println!("{:?}", s),
                data_types::primitive_types::DataTypes::LIST(l) => println!("{:?}", l),
            }
        }

        print!("\n");
        buffer.clear();
    }
}
