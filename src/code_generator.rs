use ::std::collections::HashMap;
use ast::*;



struct Generator {
   output: Vec<u16>,                      // The binary output so far
   pending_labels: Vec<(String, u16)>,    // The list of labels to be resolved after generation
   label_locations: HashMap<String, u16>, // The list of labels and their locations for resolution after generation
}


impl Generator {
   fn consume(&mut self, instruction: Instruction) {
      match instruction {
         Instruction::Basic(operation, destination, value) => self.generate_basic(operation, destination, value),
         Instruction::Special(operation, value)            => self.generate_special(operation, value),
         Instruction::Label(label)                         => self.append_label(label),
         Instruction::Data(data)                           => self.generate_data(data),
      }
   }



   fn finalize(&mut self) {
      for &(ref label, ref location) in &self.pending_labels {
         match self.label_locations.get(label) {
            Some(origin) => self.output[*location as usize] = *origin,
            None => panic!("Unknown label {}", label),
         }
      }
   }



   fn append_label(&mut self, label: String) {
      self.label_locations.insert(label, self.output.len() as u16);
   }



   fn generate_data(&mut self, data: Vec<DatData>) {
      for datum in data {
         match datum {
            DatData::Label(label)   => self.append_label(label),
            DatData::Number(number) => self.output.push(number),
            DatData::String(string) => self.output.extend(string.into_bytes().iter().map(|x| *x as u16)),
         }
      }
   }



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



   fn generate_special(&mut self, operation: SpecialOperation, value: Value) {
      let operation_opcode: u16 = operation.encode();
      let (value_opcode, value_literal) = value.encode();

      let opcode = (value_opcode << 10) | (operation_opcode << 5) | 0u16;
      self.output.push(opcode);

      if let Some(literal) = value_literal {
         self.generate_literal(literal);
      }
   }



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