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

        if buffer.trim().is_empty() {
            continue;
        }

        if buffer.trim() == "print_vars()" {
            for r in vtm.get_table().iter() {
                match r.get_value() {
                    data_types::primitive_types::DataTypes::NUMBER(n) => println!("{} = {:?}", r.name, n),
                    data_types::primitive_types::DataTypes::BOOL(b) => println!("{} = {:?}", r.name, b),
                    data_types::primitive_types::DataTypes::FLOAT(f) => println!("{} = {:?}", r.name, f),
                    data_types::primitive_types::DataTypes::STR(s) => println!("{} = {:?}", r.name, s),
                    data_types::primitive_types::DataTypes::LIST(l) => println!("{} = {:?}", r.name, l),
                }
            }
            continue;
        }

        let syntax_report = generate_syntax_report(&buffer.trim());
        ReportHandler::handle_report(&syntax_report, &mut vtm);
        //ReportHandler::print_status_report(&syntax_report);

        
        buffer.clear();
    }
}
