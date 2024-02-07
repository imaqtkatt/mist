use core::fmt;
use std::rc::Rc;

use crate::{local::Local, opcode::Opcode, value::MistValue};

// #[derive(Clone, Debug)]
// pub struct AttributeInfo {
//   // pub attribute_name_index: u16,
//   // pub attribute_name: String,
//   // pub attribute_length: u32,
//   // pub info: Vec<u8>,
//   pub info: Info,
// }

#[derive(Clone, Debug)]
pub enum AttributeInfo {
  Code(Code),
  LineNumberTable(LineNumberTable),
  Bytes(Vec<u8>),
}

pub type NativeCode = Option<Rc<dyn Fn(&Local) -> Option<MistValue>>>;

#[derive(Clone)]
pub struct Code {
  pub native: NativeCode,

  pub max_stack: u16,
  pub max_local: u16,
  pub code: Vec<Opcode>,
  pub exception_table: Vec<ExceptionTableInfo>,
  pub attributes: Vec<AttributeInfo>,
}

impl Code {
  pub fn is_native(&self) -> bool {
    self.native.is_some()
  }

  pub fn native(
    f: impl Fn(&Local) -> Option<MistValue> + 'static,
    max_local: u16,
  ) -> Self {
    Self {
      native: Some(Rc::new(f)),
      max_local,
      max_stack: 0,
      code: Vec::new(),
      exception_table: Vec::new(),
      attributes: Vec::new(),
    }
  }
}

impl fmt::Debug for Code {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Code")
      .field("native", &self.native.is_some())
      .field("max_stack", &self.max_stack)
      .field("max_local", &self.max_local)
      .field("code", &self.code)
      .field("exception_table", &self.exception_table)
      .field("attributes", &self.attributes)
      .finish()
  }
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
