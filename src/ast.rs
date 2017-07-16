use dcpu16_types::{BasicOperation, SpecialOperation};



#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
   Basic(BasicOperation, Destination, Value),
   Special(SpecialOperation, Value),
   Label(String),
   Data(Vec<DatData>)
}




#[derive(Debug, PartialEq, Clone)]
pub enum Destination {
   Register(Register),
   Indirection(Indirection),
   Push
}


impl Destination {
   pub fn encode(self) -> (u16, Option<Literal>) {
      match self {
         Destination::Push             => (0x18, None),
         Destination::Indirection(ind) => ind.encode(),
         Destination::Register(reg)    => (reg.encode(), None),
      }
   }
}




#[derive(Debug, PartialEq, Clone)]
pub enum Value {
   Literal(Literal),
   Indirection(Indirection),
   Register(Register),
   Pop
}


impl Value {
   pub fn encode(self) -> (u16, Option<Literal>) {
      match self {
         Value::Pop              => (0x18, None),

         Value::Literal(Literal::Number(num))
            if num <= 30 || num == 0xffff
            => (num.wrapping_add(0x21), None),

         Value::Literal(lit) => (0x1f, Some(lit)),

         Value::Register(reg)    => (reg.encode(), None),
         Value::Indirection(ind) => ind.encode(),
      }
   }
}




#[derive(Debug, PartialEq, Clone)]
pub enum Indirection {
   Register(Register),
   RegisterPlusLiteral(Register, Literal),
   Literal(Literal),
}


impl Indirection {
   pub fn encode(self) -> (u16, Option<Literal>) {
      match self {
         Indirection::Literal(lit) => (0x1e, Some(lit)),

         Indirection::Register(reg) => match reg {
            Register::A  => (0x8, None),
            Register::B  => (0x9, None),
            Register::C  => (0xa, None),
            Register::X  => (0xb, None),
            Register::Y  => (0xc, None),
            Register::Z  => (0xd, None),
            Register::I  => (0xe, None),
            Register::J  => (0xf, None),
            Register::SP => (0x19, None),
            Register::PC => panic!("Indirect PC access is not possible on the DCPU-16"),
            Register::EX => panic!("Indirect EX access is not possible on the DCPU-16"),
         },

         Indirection::RegisterPlusLiteral(reg, lit) => match reg {
            Register::A  => (0x10, Some(lit)),
            Register::B  => (0x11, Some(lit)),
            Register::C  => (0x12, Some(lit)),
            Register::X  => (0x13, Some(lit)),
            Register::Y  => (0x14, Some(lit)),
            Register::Z  => (0x15, Some(lit)),
            Register::I  => (0x16, Some(lit)),
            Register::J  => (0x17, Some(lit)),
            Register::SP => (0x1a, Some(lit)),
            Register::PC => panic!("Indirect PC access is not possible on the DCPU-16"),
            Register::EX => panic!("Indirect EX access is not possible on the DCPU-16"),
         },
      }
   }
}




#[derive(Debug, PartialEq, Clone)]
pub enum DatData {
   Label(String),
   Number(u16),
   String(String),
}




#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
   Label(String),
   Number(u16),
}




#[derive(Debug, PartialEq, Clone)]
pub enum Register {
   A,
   B,
   C,
   X,
   Y,
   Z,
   I,
   J,
   PC,
   SP,
   EX
}


impl Register {
   pub fn encode(self) -> u16 {
      match self {
         Register::A  => 0x0,
         Register::B  => 0x1,
         Register::C  => 0x2,
         Register::X  => 0x3,
         Register::Y  => 0x4,
         Register::Z  => 0x5,
         Register::I  => 0x6,
         Register::J  => 0x7,
         Register::PC => 0x1c,
         Register::SP => 0x1b,
         Register::EX => 0x1d,
      }
   }
}