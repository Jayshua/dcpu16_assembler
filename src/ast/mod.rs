pub mod destination;
pub mod indirection;
pub mod register;
pub mod value;
pub mod basic_operation;
pub mod special_operation;


pub use self::destination::Destination;
pub use self::indirection::Indirection;
pub use self::register::Register;
pub use self::value::Value;
pub use self::basic_operation::BasicOperation;
pub use self::special_operation::SpecialOperation;


#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
   Basic(BasicOperation, Destination, Value),
   Special(SpecialOperation, Value),
   Label(String),
   Data(Vec<DatData>)
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
