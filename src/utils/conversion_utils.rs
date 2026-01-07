
use crate::data_types::primitive_types::DataTypes;

pub fn convert_to_xiro_dt(val: evalexpr::Value) -> DataTypes {
        match val {
            evalexpr::Value::Int(i) => DataTypes::NUMBER(i),
            evalexpr::Value::Float(f) => DataTypes::FLOAT(f),
            evalexpr::Value::Boolean(b) => DataTypes::BOOL(b),
            evalexpr::Value::String(s) => DataTypes::STR(s),
            evalexpr::Value::Tuple(v) => {
                let inner_list = v.into_iter().map(convert_to_xiro_dt).collect();
                DataTypes::LIST(inner_list)
            }
            _ => DataTypes::STR(val.to_string()),
        }
}