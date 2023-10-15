use super::inst::Inst;

mod ast;
mod error;
mod lexer;
mod parser;
mod token;
mod transformer;

pub fn compile(expr: &str) -> Vec<Inst> {
    let mut lexer = lexer::Lexer::new(expr.chars());
    let mut parser = parser::Parser::new(&mut lexer);
    let mut transformer = transformer::Transformer::default();
    let ast = parser.parse_group().unwrap();
    transformer.transform(ast)
}
