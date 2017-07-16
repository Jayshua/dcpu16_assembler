use super::Literal;
use super::Indirection;
use super::Register;



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
         Value::Pop => (0x18, None),

         Value::Literal(Literal::Number(num))
            if num <= 30 || num == 0xffff
            => (num.wrapping_add(0x21), None),

         Value::Literal(lit) => (0x1f, Some(lit)),

         Value::Register(reg) => (reg.encode(), None),

         Value::Indirection(ind) => ind.encode(),
      }
   }
}