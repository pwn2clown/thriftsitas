#[derive(Default, Debug)]
pub struct ThriftApp {
    pub services: Vec<ThriftService>,
    pub typedecl: Vec<ThriftStruct>,
    pub consts: Vec<ThriftConst>,
}

impl ThriftApp {
    pub fn search_service(&self, name_pattern: &str, namespace: &str) -> Vec<ThriftService> {
        self.services
            .iter()
            .filter(|t| t.name.contains(name_pattern) && t.namespace.eq(namespace))
            .map(|t| t.clone())
            .collect()
    }

    pub fn search_ttype(&self, name_pattern: &str, namespace: &str) -> Vec<ThriftStruct> {
        self.typedecl
            .iter()
            .filter(|t| t.name.contains(name_pattern) && t.namespace.eq(namespace))
            .map(|t| t.clone())
            .collect()
    }
}

#[derive(Default, Debug, Clone)]
pub struct ThriftConst {
    pub value: String,
    pub namespace: String,
    pub identifier: String,
    pub ttype: String,
}

#[derive(Default, Debug, Clone)]
pub struct ThriftService {
    pub name: String,
    pub namespace: String,
    pub functions: Vec<ThriftFunction>,
}

#[derive(Debug, Default, Clone)]
pub struct ThriftStruct {
    pub name: String,
    pub namespace: String,
    pub fields: Vec<ThriftField>, //  CHANGEME
}

#[derive(Debug, Clone)]
pub enum ThriftType {
    Bool,
    Byte,
    I16,
    I32,
    I64,
    Double,
    Binary,
    Str,
    List(Box<Self>),
    CustomTypeRef { name: String, namespace: String },
}

#[derive(Debug, Clone)]
pub struct ThriftField {
    pub ttype: ThriftType,
    pub name: String,
}

#[derive(Default, Debug, Clone)]
pub struct ThriftFunction {
    pub name: String,
    pub return_type_path: String,
    pub parameter_type_paths: Vec<String>,
}
