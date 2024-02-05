use crate::opcode::Opcode;

#[derive(Clone, Debug)]
pub struct AttributeInfo {
  pub attribute_name_index: u16,
  // pub attribute_length: u32,
  // pub info: Vec<u8>,
  pub info: Info,
}

#[derive(Clone, Debug)]
pub enum Info {
  Code(Code),
  LineNumberTable(LineNumberTable),
  Bytes(Vec<u8>),
}

#[derive(Clone, Debug)]
pub struct Code {
  pub max_stack: u16,
  pub max_local: u16,
  pub code_length: u32,
  pub code: Vec<Opcode>,
  pub exception_table: Vec<ExceptionTableInfo>,
  pub attributes: Vec<AttributeInfo>,
}

#[derive(Clone, Debug)]
pub struct ExceptionTableInfo {
  pub start_pc: u16,
  pub end_pc: u16,
  pub handler_pc: u16,
  pub catch_type: u16,
}

#[derive(Clone, Debug)]
pub struct LineNumberTable {
  pub attribute_name_index: u16,
  pub attribute_length: u32,
  pub line_number_table: Vec<LineNumberTableInfo>,
}

#[derive(Clone, Debug)]
pub struct LineNumberTableInfo {
  pub start_pc: u16,
  pub line_number: u16,
}
