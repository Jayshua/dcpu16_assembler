mod ast;
mod code_generator;
mod parser {
   include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

pub use parser::ParseError as ParseError;

pub fn assemble<'a>(input: &'a str) -> Result<Vec<u16>, ParseError> {
   match parser::program(input) {
      Ok(ast) => {
         #[cfg(feature = "print-ast")]
         println!("{:?}", ast);

         Ok(code_generator::generate(ast))
      },

      Err(error) => Err(error),
   }
}