#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SpecialOperation {
   Jsr,
   Int,
   Iag,
   Ias,
   Rfi,
   Iaq,
   Hwn,
   Hwq,
   Hwi,
}



impl SpecialOperation {
   pub fn encode(self) -> u16 {
      match self {
         SpecialOperation::Jsr => 0x01,
         SpecialOperation::Int => 0x08,
         SpecialOperation::Iag => 0x09,
         SpecialOperation::Ias => 0x0a,
         SpecialOperation::Rfi => 0x0b,
         SpecialOperation::Iaq => 0x0c,
         SpecialOperation::Hwn => 0x10,
         SpecialOperation::Hwq => 0x11,
         SpecialOperation::Hwi => 0x12,
      }
   }
}
