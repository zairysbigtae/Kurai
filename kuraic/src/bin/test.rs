use inkwell::context::Context;
use kurai_codegen::codegen::CodeGen;
use kurai_parser::parse::{parse::{parse_out_vec_expr, parse_out_vec_stmt}, parse_stmt::StmtParserStruct};
use kurai_parser_function::FunctionParserStruct;
use kurai_parser_import_decl::ImportParserStruct;
use kurai_token::token::token::Token;

fn main() {
    let code = r#"
        use print;
        fn main() {
            print::ok();
        }
        "#.to_string();

    let context = Context::create();
    let tokens = Token::tokenize(code.as_str());
    let mut discovered_modules: Vec<String> = Vec::new();
    let parsed_stmt_vec = parse_out_vec_stmt(&tokens, &mut discovered_modules, &FunctionParserStruct, &ImportParserStruct);
    let parsed_expr_vec = parse_out_vec_expr(&tokens);
    let mut codegen = CodeGen::new(&context);

    let mut discovered_modules = Vec::new();

    codegen.generate_code(parsed_stmt_vec, parsed_expr_vec.unwrap(), &mut discovered_modules, &StmtParserStruct, &FunctionParserStruct, &ImportParserStruct);

    println!("{}", codegen.show_result());
}
