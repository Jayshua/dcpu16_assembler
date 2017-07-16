use super::Register;
use super::Literal;



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
