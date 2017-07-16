use super::Register;
use super::Indirection;
use super::Literal;



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
