#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BasicOperation {
   Set,
   Add,
   Sub,
   Mul,
   Mli,
   Div,
   Dvi,
   Mod,
   Mdi,
   And,
   Bor,
   Xor,
   Shr,
   Asr,
   Shl,
   Ifb,
   Ifc,
   Ife,
   Ifn,
   Ifg,
   Ifa,
   Ifl,
   Ifu,
   Adx,
   Sbx,
   Sti,
   Std,
}



impl BasicOperation {
   pub fn encode(self) -> u16 {
      match self {
         BasicOperation::Set => 0x01,
         BasicOperation::Add => 0x02,
         BasicOperation::Sub => 0x03,
         BasicOperation::Mul => 0x04,
         BasicOperation::Mli => 0x05,
         BasicOperation::Div => 0x06,
         BasicOperation::Dvi => 0x07,
         BasicOperation::Mod => 0x08,
         BasicOperation::Mdi => 0x09,
         BasicOperation::And => 0x0a,
         BasicOperation::Bor => 0x0b,
         BasicOperation::Xor => 0x0c,
         BasicOperation::Shr => 0x0d,
         BasicOperation::Asr => 0x0e,
         BasicOperation::Shl => 0x0f,
         BasicOperation::Ifb => 0x10,
         BasicOperation::Ifc => 0x11,
         BasicOperation::Ife => 0x12,
         BasicOperation::Ifn => 0x13,
         BasicOperation::Ifg => 0x14,
         BasicOperation::Ifa => 0x15,
         BasicOperation::Ifl => 0x16,
         BasicOperation::Ifu => 0x17,
         BasicOperation::Adx => 0x1a,
         BasicOperation::Sbx => 0x1b,
         BasicOperation::Sti => 0x1e,
         BasicOperation::Std => 0x1f,
      }
   }
}