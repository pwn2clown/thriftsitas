fn _parse_param() {
    let content = "1:com.app.thrift.data.LoginRequest request";

    ThriftParser::parse(Rule::parameter, content).unwrap();
}

fn _parse_function() {
    let content =
        r#"com.app.thrift.data.LoginResponse login(1:1:com.app.thrift.data.LoginRequest request)"#;

    ThriftParser::parse(Rule::function_definition, content).unwrap();
}

fn _parse_function_throw() {
    let content = r#"com.app.thrift.data.LoginResponse login(1:1:com.app.thrift.data.LoginRequest request) throws (1:com.app.exception.thrift.data.TechnicalException technicalException, 2:com.app.exception.thrift.data.FunctionnalException functionnalException)"#;

    ThriftParser::parse(Rule::function_definition, content).unwrap();
}

fn _parse_service() {
    let content = r#"service Acces {
	/**
	 * lol
	 */	
	com.app.thrift.data.LoginResponse login(1:1:com.app.thrift.data.LoginRequest request)throws (1:com.app.exception.thrift.data.TechnicalException technicalException, 2:com.app.exception.thrift.data.FunctionnalException functionnalException)
    }"#;

    ThriftParser::parse(Rule::service_definition, content).unwrap();
}

fn _parse_nested_ttype() {
    println!(
        "{:#?}",
        ThriftParser::parse(Rule::ttype, "list<string>")
            .unwrap()
            .next()
            .unwrap()
            .into_inner()
    );
}

fn _parse_enum_value() {
    ThriftParser::parse(Rule::enum_value, "ENUM_VALUE_01= 01").unwrap();
}

fn _parse_enum_def() {
    let content = r#"enum SomeEnum {	
	ENUM_VALUE_01 = 01,
	ENUM_VALUE_02 = 02,
    }"#;

    ThriftParser::parse(Rule::enum_definition, content).unwrap();
}
