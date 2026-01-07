// src/memory_table/vartable.rs

use std::collections::HashMap;
use evalexpr::*;

use super::super::data_types::primitive_types;
use super::super::data_types::primitive_types::evaluate;
use super::super::data_types::primitive_types::retrieve_number;
use super::super::data_types::primitive_types::retrieve_bool;
use super::super::data_types::primitive_types::retrieve_string;
use super::super::data_types::primitive_types::retrieve_list;

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    value: primitive_types::DataTypes,
    var_type_number: i32
}

impl Variable {
    pub fn new(name: String, value: primitive_types::DataTypes) -> Self {
        let var_type_number = 0;
        Variable {
            name,
            value,
            var_type_number
        }
    }

    pub fn get_value(&self) -> &primitive_types::DataTypes {
        &self.value
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn resolve_value_type(&mut self) {

        self.var_type_number = match evaluate(&self.value) {
            primitive_types::DataTypesIds::ISNUMBER => 1,
            primitive_types::DataTypesIds::ISBOOL => 2,
            primitive_types::DataTypesIds::ISSTR => 3,
            primitive_types::DataTypesIds::ISFLOAT => 4,
            primitive_types::DataTypesIds::ISLIST => 5,
        };
    }

    pub fn get_value_type_id(&self) -> i32 {
        self.var_type_number
    }

    pub fn try_get_number(&self) -> &i64 {
        retrieve_number(&self.value).unwrap().unwrap()
    }

    pub fn try_get_bool(&self) -> &bool {
        retrieve_bool(&self.value).unwrap().unwrap()
    }

    pub fn try_get_string(&self) -> &String {
        retrieve_string(&self.value).unwrap().unwrap()
    }

    pub fn try_get_list(&self) -> &Vec<primitive_types::DataTypes> {
        retrieve_list(&self.value).unwrap().unwrap()
    }

    
}

#[derive(Debug)]
pub struct VariableTableInMemory {
    variable_list: Vec<Variable>
}



impl VariableTableInMemory {
    pub fn new() -> Self {
        VariableTableInMemory { variable_list: Vec::new() }
    }

    pub fn resolve_existing_identificators(&self, raw_value: String) -> String {
        // if raw value for example (a + 1) where a is a existing variable in the table, should be replaced with the value in the table
        let mut resolved_value = raw_value.clone();
        for var in self.variable_list.iter() {
            let var_name = var.name.clone();
            let var_value = match &var.value {
                primitive_types::DataTypes::NUMBER(n) => n.to_string(),
                primitive_types::DataTypes::FLOAT(f) => f.to_string(),
                primitive_types::DataTypes::BOOL(b) => b.to_string(),
                primitive_types::DataTypes::LIST(l) => {
                    let elements: Vec<String> = l.iter().map(|dt| dt.to_string()).collect();
                    format!("({})", elements.join(","))
                }
                primitive_types::DataTypes::STR(s) => format!("\"{}\"", s),
                _ => var.value.to_string(),
            };
            resolved_value = resolved_value.replace(&var_name, &var_value);
        }
        resolved_value
    }

    pub fn get_table(&self) -> &Vec<Variable> {
        self.variable_list.as_ref()
    }

    pub fn push_var(&mut self, var: Variable) {
        if self.variable_list.iter().any(|v| v.name == var.name) {
            panic!("Variable with name '{}' already exist", var.name)
        }
        self.variable_list.push(var);
    }

    pub fn set_var(&mut self, var: Variable) {
        for v in self.variable_list.iter_mut() {
            if v.name == var.name {
                *v = var;
                return;
            }
        }
        panic!("Variable with name '{}' does not exist", var.name)
    }
}
