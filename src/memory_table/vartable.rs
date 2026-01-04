// src/memory_table/vartable.rs

use super::super::data_types::primitive_types;
use super::super::data_types::primitive_types::evaluate;
use super::super::data_types::primitive_types::retrieve_number;
use super::super::data_types::primitive_types::retrieve_bool;
use super::super::data_types::primitive_types::retrieve_string;
use super::super::data_types::primitive_types::retrieve_list;

#[derive(Debug)]
pub struct Variable {
    name: String,
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

    pub fn push_var(&mut self, var: Variable) {
        self.variable_list.push(var);
    }
}
