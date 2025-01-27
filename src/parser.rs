use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
use std::error::Error;

use crate::ttypes::*;

#[derive(Parser)]
#[grammar = "grammar/thrift.pest"]
struct ThriftParser;

fn pair_to_ttype(pair: Pair<Rule>) -> ThriftType {
    let pair = pair.into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::tboolean => ThriftType::Bool,
        Rule::tstring => ThriftType::Str,
        Rule::tbyte => ThriftType::Byte,
        Rule::ti16 => ThriftType::I16,
        Rule::ti32 => ThriftType::I32,
        Rule::ti64 => ThriftType::I64,
        Rule::tdouble => ThriftType::Double,
        Rule::tbinary => ThriftType::Binary,
        Rule::namespace => match pair.as_str().rsplit_once(".") {
            Some((namespace, name)) => ThriftType::CustomTypeRef {
                name: name.into(),
                namespace: namespace.into(),
            },
            None => ThriftType::CustomTypeRef {
                name: pair.as_str().into(),
                namespace: "".into(),
            },
        },
        Rule::tlist => {
            let inner_ttype = pair_to_ttype(pair.into_inner().next().unwrap());
            ThriftType::List(Box::new(inner_ttype))
        }
        _ => {
            unreachable!("ttype does not exists")
        }
    }
}

fn pair_to_thrift_struct(mut pairs: Pairs<Rule>) -> ThriftStruct {
    let mut tstruct = ThriftStruct::default();

    let identifier = pairs.next().unwrap().as_str();
    tstruct.name = identifier.into();

    for field_pair in pairs.next().unwrap().into_inner() {
        let mut field_parts = field_pair.into_inner();
        let _idx = field_parts.next();
        let ttype = field_parts.next().unwrap();
        let identifier = field_parts.next().unwrap().as_str();

        tstruct.fields.push(ThriftField {
            ttype: pair_to_ttype(ttype),
            name: identifier.into(),
        });
    }

    tstruct
}

fn pair_to_thrift_function(pairs: Pairs<Rule>) -> ThriftFunction {
    let mut func = ThriftFunction::default();

    for pair in pairs {
        match pair.as_rule() {
            Rule::ttype => func.return_type_path = pair.as_str().into(),
            Rule::identifier => func.name = pair.as_str().into(),
            Rule::parameter_list => {
                for param_pair in pair.into_inner() {
                    let param_type = param_pair.into_inner().nth(1).unwrap();
                    func.parameter_type_paths.push(param_type.as_str().into());
                }
            }
            Rule::throws_clause => { /* pass */ }
            _ => unreachable!(),
        }
    }

    func
}

fn pair_to_thrift_service(pairs: Pairs<Rule>) -> ThriftService {
    let mut service = ThriftService::default();

    for pair in pairs {
        match pair.as_rule() {
            Rule::identifier => service.name = pair.as_str().into(),
            Rule::function_definition => {
                let func = pair_to_thrift_function(pair.into_inner());
                service.functions.push(func);
            }
            _ => unreachable!(),
        }
    }

    service
}

pub fn parse(content: &str, app: &mut ThriftApp) -> Result<(), Box<dyn Error>> {
    let pairs = ThriftParser::parse(Rule::thrift, &content)?;
    let mut thrift_definition = match pairs.last() {
        Some(pair) => pair.into_inner(),
        None => {
            eprintln!("warning: empty thrift file");
            return Ok(());
        }
    };

    let namespace = thrift_definition.next();
    let namespace = namespace.unwrap().into_inner().last().unwrap().as_str();

    for pair in thrift_definition {
        match pair.as_rule() {
            Rule::service_definition => {
                let mut svc = pair_to_thrift_service(pair.into_inner());
                svc.namespace = namespace.into();
                app.services.push(svc);
            }
            Rule::const_definition => {
                let mut const_def_pair = pair.into_inner();
                let ttype = const_def_pair.next().unwrap().as_str().to_string();
                let identifier = const_def_pair.next().unwrap().as_str().to_string();
                let value = const_def_pair.next().unwrap().as_str().to_string();

                app.consts.push(ThriftConst {
                    namespace: namespace.into(),
                    ttype,
                    identifier,
                    value,
                });
            }
            Rule::struct_definition => {
                let mut structdef = pair_to_thrift_struct(pair.into_inner());
                structdef.namespace = namespace.into();
                app.typedecl.push(structdef);
            }
            Rule::enum_definition => {
                // TODO
            }
            Rule::typedef => {
                //  TODO
            }
            Rule::namespace_definition | Rule::include | Rule::exception_definition => {
                //  pass
            }
            _ => {
                eprintln!("unreachable rule: {:?}", pair.as_rule());
                unreachable!()
            }
        }
    }

    Ok(())
}
