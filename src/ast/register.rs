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
