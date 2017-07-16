mod ast;
mod code_generator;
mod parser {
   include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

pub use parser::ParseError as ParseError;

pub fn assemble<'a>(input: &'a str, print_ast: bool) -> Result<Vec<u16>, ParseError> {
   match parser::program(input) {
      Ok(ast) => {
         if print_ast {
            println!("{:?}", ast);
         }

         Ok(code_generator::generate(ast))
      },

      Err(error) => Err(error),
   }
}