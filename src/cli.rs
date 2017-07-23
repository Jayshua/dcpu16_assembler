/*!
A basic CLI wrapper for the DCPU-16 assembler
Usage:
   dasm input_file [-o output_file]
*/

extern crate dcpu16_assembler;
use dcpu16_assembler::assemble;

extern crate argparse;
use argparse::{ArgumentParser, Store, StoreOption};

use std::fs::File;
use std::path::Path;
use std::io::{Write, Read};



// Get the input and output files from the cli arguments,
// read the file, assemble it, and write the output to a file.
fn main() {
   let (output_file_path, input_file_path) = get_arguments();
   let input_program = read_program(input_file_path);

   let assembled_program = match assemble(input_program.as_str()) {
      Ok(program) => program,

      Err(err) => {
         println!("Error assembling program: {}", err);
         std::process::exit(-1);
      },
   };

   write_bits(output_file_path, assembled_program);
}



// Get the input file path and output file path from the command line arguments
fn get_arguments() -> (String, String) {
   // Get the arguments from the command line
   let mut input_argument: String = String::new();
   let mut output_argument: Option<String> = None;

   {
      let mut ap = ArgumentParser::new();
      ap.set_description("DCPU-16 Assembler");
      ap.refer(&mut input_argument).add_argument("file", Store, "file to compile").required();
      ap.refer(&mut output_argument).add_option(&["-o", "--out"], StoreOption, "set the output file");
      ap.parse_args_or_exit();
   }


   // Return the arguments with a default output file filled in if necessary
   match output_argument {
      Some(output_path) => (output_path, input_argument),
      None => (Path::new(&input_argument).with_extension("bin").to_str().unwrap().to_string(), input_argument),
   }
}



// Read the file at the provided path into a String
fn read_program(file_path: String) -> String {
   let mut in_file = match File::open(file_path) {
      Ok(file) => file,
      Err(err) => {
         println!("Unable to open input file: {}", err);
         std::process::exit(-1);
      },
   };

   let mut program: String = String::new();

   match in_file.read_to_string(&mut program) {
      Ok(_) => program,
      Err(err) => {
         println!("Unable to read input file: {}", err);
         std::process::exit(-1);
      },
   }
}



// Write the provided data to a file at the provided path
fn write_bits(file_path: String, data: Vec<u16>) {
   let mut out_file = match File::create(file_path) {
      Ok(file) => file,
      Err(err) => {
         println!("Unable to create output file: {}", err);
         std::process::exit(-1);
      },
   };


   for word in data {
      let bytes: [u8; 2] = [(word >> 8) as u8, (word & 0xff) as u8];

      match out_file.write(&bytes[..]) {
         Ok(_) => (),
         Err(err) => {
            println!("Unable to write to output file: {}", err);
            std::process::exit(-1);
         }
      }
   }
}

