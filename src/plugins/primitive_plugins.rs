use crate::data_types::primitive_types::DataTypes;

pub enum IOPLUGINS {
    WRITE(DataTypes),
}

#[derive(Debug)]
pub enum PRIMITIVEPLUGINS {
    TOARRAY,
    LEN
}

impl PRIMITIVEPLUGINS {
    pub fn from_str(input: &str) -> Option<PRIMITIVEPLUGINS> {
        match input {
            "to_array" => Some(PRIMITIVEPLUGINS::TOARRAY),
            "len" => Some(PRIMITIVEPLUGINS::LEN),
            _ => None,
        }
    }
}
