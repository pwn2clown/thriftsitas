use std::{
    error::Error,
    fs::{self, read_to_string},
};

mod parser;
mod serializer;
mod ttypes;

use ttypes::*;

fn main() -> Result<(), Box<dyn Error>> {
    let base_path = "../data";

    let thrift_files: Vec<String> = fs::read_dir(base_path)?
        .map(|p| p.unwrap().file_name().to_str().unwrap().to_string())
        .filter(|s| s.ends_with(".thrift"))
        .collect();

    let mut app = ThriftApp::default();

    for file in thrift_files.iter() {
        let thrift_content = read_to_string(format!("{base_path}/{file}"))?;
        parser::parse(&thrift_content, &mut app)?;
    }

    /*
    let params = ttypes.first().unwrap();

    serializer::flatten_struct(&app, params);
    */

    Ok(())
}
