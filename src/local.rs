use crate::value::MistValue;

pub struct Local {
  variables: Vec<MistValue>,
}

impl Local {
  pub fn new(capacity: usize) -> Self {
    let variables = vec![MistValue::default(); capacity];
    Self { variables }
  }

  #[inline(always)]
  pub fn store(&mut self, index: usize, variable: MistValue) {
    self.variables[index] = variable;
  }

  #[inline(always)]
  pub fn load(&self, index: usize) -> MistValue {
    self.variables[index]
  }

  /// [crate::opcode::IINC].
  #[inline(always)]
  pub fn iinc(&mut self, index: usize, r#const: i32) {
    let int: i32 = self.variables[index].into();
    self.variables[index] = MistValue::Integer(int + r#const);
  }
}
