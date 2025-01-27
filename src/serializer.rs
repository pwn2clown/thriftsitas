use crate::{ThriftApp, ThriftStruct, ThriftType};

pub fn flatten_struct(app: &ThriftApp, ttype: &ThriftStruct) -> Vec<u8> {
    let mut buf = Vec::new();

    //  Flatten struct
    for field in &ttype.fields {
        match &field.ttype {
            ThriftType::CustomTypeRef { name, namespace } => {
                let custom_ttype = app.search_ttype(&name, &namespace);
                let custom_ttype = custom_ttype.first().unwrap();

                println!("{custom_ttype:#?}");
            }
            _ => println!(""),
        }
    }

    buf
}
