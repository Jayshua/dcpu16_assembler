extern crate dcpu16_types;

mod parser {
   include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

mod ast;
mod code_generator;

pub fn generate<'a>(input: &'a str) -> Vec<u16> {
   let parsed = parser::program(input).unwrap();
   let code = code_generator::Generator::generate(parsed);
   code
}