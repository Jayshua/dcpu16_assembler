/*!
The types for building an Abstract Syntax Tree of a DCPU-16 assembly program.

The AST will be a Vec&lt;ast::Instruction&gt;, where each instruction will be one of:

+ Basic(Operation, Destination, Value) - A basic DCPU-16 instruction such as Set or Add
+ Special(Operation, Value)            - A special DCPU-16 instruction such as Jsr or Hwi
+ Label(String)                        - A label in the program such as :loop_begin
+ Data(Vec<Data>)                      - A data declaration for embedding raw data directly in the program


The Basic and Special instructions contain a Value component, which represents
the source of the data for the operation. (A.K.A., the `a` argument of the instruction.)
This value can be one of:

+ Literal(number)          - A literal number
+ Indirection(indirection) - An indirect access to memory, i.e. [a] or [a + 0xf] or [0x93f]
+ Register(register)       - A register
+ Pop                      - The value top value on the stack, incrementing `SP` as a side effect


The Basic instruction also contains a Destination component which represents the `b` argument of the instruction.
This destination can be one of:

+ Register(register)       - The register to store the value in
+ Indirection(indirection) - The location in memory indicated by an indirect access, i.e. [a] or [a + 0xf]
+ Push                     - The top of the stack, decrementing `SP` as a side effect

Indirect memory access in both the Value and Destination components can have three forms:

+ Register - The location in memory pointed to by a register. i.e. [a]
+ RegisterPlusLiteral - The location in memory pointed to by a register, offset by the next word. i.e. [a + 0x20]
+ Literal - The location in memory pointed to by the next word. i.e. [0xffff]

A completed AST might look like this:

```
[
   Basic(Set, Register(X), Literal(Number(33800))),
   Basic(Set, Register(Y), Literal(Label("data"))),
   Basic(Set, Register(I), Literal(Number(79))),
   Special(Jsr, Literal(Label("CRC_Calculate"))),
   Data([Number(0)]),
   Label("CRC_Calculate"),
   Basic(Set, Push, Register(I)),
   Basic(Set, Push, Register(J)),
   Basic(Set, Push, Register(X)),
   Basic(Set, Push, Register(Y)),
   Basic(Set, Push, Register(A)),
   Basic(Set, Push, Register(B)),
   Basic(Set, Push, Register(C)),
   Basic(Set, Register(B), Literal(Number(1))),
   Basic(Set, Register(Z), Indirection(Register(Y))),
   Basic(Set, Register(J), Register(Y)),
   Basic(Add, Register(J), Register(I)),
   Basic(Set, Register(Z), Indirection(Register(J))),
   Label("CRC_Calculate_loop"),
   Special(Jsr, Literal(Label("CRC_Calculate_loop_perform_calc"))),
   Basic(Shr, Register(Z), Literal(Number(1))),
   Basic(Set, Register(C), Indirection(Register(J))),
   Basic(And, Register(C), Register(B)),
   Basic(Ifg, Register(C), Literal(Number(0))),
   Special(Jsr, Literal(Label("CRC_Calculate_loop_shift"))),
   Basic(Xor, Register(Z), Register(C)),
   Basic(Shl, Register(B), Literal(Number(1))),
   Basic(Ifn, Register(B), Literal(Number(0))),
   Basic(Set, Register(PC), Literal(Label("CRC_Calculate_loop"))),
   Basic(Set, Register(B), Literal(Number(1))),
   Basic(Sub, Register(J), Literal(Number(1))),
   Basic(Ifl, Register(J), Register(Y)),
   Basic(Set, Register(PC), Literal(Label("CRC_Calculate_loop_end"))),
   Basic(Set, Register(PC), Literal(Label("CRC_Calculate_loop"))),
   Label("CRC_Calculate_loop_shift"),
   Basic(Ife, Register(C), Literal(Number(1))),
   Basic(Set, Register(PC), Pop),
   Basic(Shr, Register(C), Literal(Number(1))),
   Basic(Set, Register(PC), Literal(Label("CRC_Calculate_loop_shift"))),
   Label("CRC_Calculate_loop_perform_calc"),
   Basic(Set, Register(A), Register(Z)),
   Basic(And, Register(A), Literal(Number(1))),
   Basic(Ifg, Register(A), Literal(Number(0))),
   Basic(Xor, Register(Z), Register(X)),
   Basic(Set, Register(PC), Pop),
   Label("CRC_Calculate_loop_end"),
   Basic(Set, Register(C), Pop),
   Basic(Set, Register(B), Pop),
   Basic(Set, Register(A), Pop),
   Basic(Set, Register(Y), Pop),
   Basic(Set, Register(X), Pop),
   Basic(Set, Register(J), Pop),
   Basic(Set, Register(I), Pop),
   Basic(Set, Register(PC), Pop),
   Label("data"),
   Data([String("Test tseT Test tseT Test tseT tseT tseT Test tseT Test tseT Test tseT Test tseT")])
]
```

*/

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
