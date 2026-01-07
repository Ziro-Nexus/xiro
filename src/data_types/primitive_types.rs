use syn::Data;

#[derive(Debug)]
pub enum DataTypes {
    STR(String),
    NUMBER(i64),
    FLOAT(f64),
    BOOL(bool),
    LIST(Vec<DataTypes>),
}

impl Clone for DataTypes {
    fn clone(&self) -> Self {
        match self {
            DataTypes::STR(s) => DataTypes::STR(s.clone()),
            DataTypes::NUMBER(n) => DataTypes::NUMBER(*n),
            DataTypes::FLOAT(f) => DataTypes::FLOAT(*f),
            DataTypes::BOOL(b) => DataTypes::BOOL(*b),
            DataTypes::LIST(l) => DataTypes::LIST(l.clone()),
        }
    }
}

impl ToString for DataTypes {
    fn to_string(&self) -> String {
        match self {
            DataTypes::STR(s) => s.clone(),
            DataTypes::NUMBER(n) => n.to_string(),
            DataTypes::FLOAT(f) => f.to_string(),
            DataTypes::BOOL(b) => b.to_string(),
            DataTypes::LIST(l) => {
                let elements: Vec<String> = l.iter().map(|dt| dt.to_string()).collect();
                format!("({})", elements.join(","))
            }
        }
    }
}

pub enum DataTypesIds {
    ISSTR,
    ISNUMBER,
    ISFLOAT,
    ISBOOL,
    ISLIST,
}

pub fn evaluate(dt: &DataTypes) -> DataTypesIds {
    let eval: DataTypesIds = match dt {
        DataTypes::STR(_) => DataTypesIds::ISSTR,
        DataTypes::NUMBER(_) => DataTypesIds::ISNUMBER,
        DataTypes::BOOL(_) => DataTypesIds::ISBOOL,
        DataTypes::FLOAT(_) => DataTypesIds::ISFLOAT,
        DataTypes::LIST(_) => DataTypesIds::ISLIST,
    };
    return eval;
}

pub fn retrieve_number(dt: &DataTypes) -> Option<Box<Option<&i64>>> {
    let retrieved = match dt {
        DataTypes::STR(_) => None,
        DataTypes::NUMBER(n) => Some(Box::new(Some(n))),
        DataTypes::BOOL(_) => None,
        DataTypes::FLOAT(_) => None,
        DataTypes::LIST(_) => None,
    };

    retrieved
}

pub fn retrieve_bool(dt: &DataTypes) -> Option<Box<Option<&bool>>> {
    let retrieved = match dt {
        DataTypes::STR(_) => None,
        DataTypes::NUMBER(_) => None,
        DataTypes::BOOL(b) => Some(Box::new(Some(b))),
        DataTypes::FLOAT(_) => None,
        DataTypes::LIST(_) => None,
    };

    retrieved
}

pub fn retrieve_string(dt: &DataTypes) -> Option<Box<Option<&String>>> {
    let retrieved = match dt {
        DataTypes::STR(s) => Some(Box::new(Some(s))),
        DataTypes::NUMBER(n) => None,
        DataTypes::BOOL(b) => None,
        DataTypes::FLOAT(f) => None,
        DataTypes::LIST(l) => None,
    };

    retrieved
}

pub fn retrieve_list(dt: &DataTypes) -> Option<Box<Option<&Vec<DataTypes>>>> {
    let retrieved = match dt {
        DataTypes::STR(s) => None,
        DataTypes::NUMBER(n) => None,
        DataTypes::BOOL(b) => None,
        DataTypes::FLOAT(f) => None,
        DataTypes::LIST(l) => Some(Box::new(Some(l))),
    };

    retrieved
}
