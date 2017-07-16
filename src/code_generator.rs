/*!
Module for generating a raw DCPU binary program from
an Abstract Syntax Tree using the types in the ast module.

Call the function `generate` to generate a Vec<u16> from a Vec<ast::Instruction>
*/


/*
The code generator works in two passes. First, it loops over every instruction
converting it into its corresponding opcode, inserting 0x0000 into memory for
in place of every label reference, and storing the memory location of each label
instantiation as it is encountered. Second, it loops over every 0x0000 that was
inserted in place of a label and replaces it with the actual memory location that
was stored during the first pass. This allows forward referencing label locations.
*/


use ::std::collections::HashMap;
use ast::*;


/// Generate raw binary data from a Vec<ast::Instruction>
///
/// Loops over each instruction generating the opcode, then
/// resolves all label references with their actual locations
/// in the output data
pub fn generate(instructions: Vec<Instruction>) -> Vec<u16> {
   let mut generator = Generator {
      output: vec![],
      pending_labels: vec![],
      label_locations: HashMap::new(),
   };

   for inst in instructions {
      generator.consume(inst);
   }

   generator.finalize();

   generator.output
}



/// Generates a DCPU-16 program from an AST
struct Generator {
   /// The binary output so far
   output: Vec<u16>,
   /// The list of label references to be resolved after generation
   pending_labels: Vec<(String, u16)>,
   /// The list of labels and their locations for resolution after generation
   label_locations: HashMap<String, u16>,
}


impl Generator {
   /// Consume an instruction, storing its opcode into the output data
   fn consume(&mut self, instruction: Instruction) {
      match instruction {
         Instruction::Basic(operation, destination, value) => self.generate_basic(operation, destination, value),
         Instruction::Special(operation, value)            => self.generate_special(operation, value),
         Instruction::Label(label)                         => self.append_label(label),
         Instruction::Data(data)                           => self.generate_data(data),
      }
   }


   /**
   Iterate over every label reference, replacing the `0x0000`
   placeholder with the label's actual location.
   */
   fn finalize(&mut self) {
      for &(ref label, ref location) in &self.pending_labels {
         match self.label_locations.get(label) {
            Some(origin) => self.output[*location as usize] = *origin,
            None => panic!("Unknown label {}", label),
         }
      }
   }


   /// Add a label and its location to the map of labels for resolution after code generation completes
   fn append_label(&mut self, label: String) {
      self.label_locations.insert(label, self.output.len() as u16);
   }


   /// Add a list of data declarations to the binary data
   fn generate_data(&mut self, data: Vec<DatData>) {
      for datum in data {
         match datum {
            DatData::Label(label)   => self.append_label(label),
            DatData::Number(number) => self.output.push(number),
            DatData::String(string) => self.output.extend(string.into_bytes().iter().map(|x| *x as u16)),
         }
      }
   }


   /// Generate the opcode for a basic instruction, adding it to the output data
   fn generate_basic(&mut self, operation: BasicOperation, destination: Destination, value: Value) {
      let operation_opcode: u16 = operation.encode();
      let (destination_opcode, destination_literal) = destination.encode();
      let (value_opcode, value_literal) = value.encode();

      let opcode = (value_opcode << 10) | (destination_opcode << 5) | operation_opcode;

      self.output.push(opcode);

      if let Some(literal) = value_literal {
         self.generate_literal(literal);
      }

      if let Some(literal) = destination_literal {
         self.generate_literal(literal);
      }
   }


   /// Generate the opcode for a special instruction, adding it to the output data
   fn generate_special(&mut self, operation: SpecialOperation, value: Value) {
      let operation_opcode: u16 = operation.encode();
      let (value_opcode, value_literal) = value.encode();

      let opcode = (value_opcode << 10) | (operation_opcode << 5) | 0u16;
      self.output.push(opcode);

      if let Some(literal) = value_literal {
         self.generate_literal(literal);
      }
   }


   /**
   Generate the binary representation of a literal, adding it to the output data.
   (Either a number or a label reference)
   */
   fn generate_literal(&mut self, literal: Literal) {
      match literal {
         Literal::Label(label) => {
            self.pending_labels.push((label, self.output.len() as u16));
            self.output.push(0x0);
         },

         Literal::Number(number) => {
            self.output.push(number);
         },
      }
   }
}
