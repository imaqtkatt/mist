use std::ops::Neg;

use crate::value::MistValue;

pub struct MistStack {
  stack: Vec<MistValue>,
}

impl MistStack {
  pub fn new() -> Self {
    Self { stack: Vec::new() }
  }
}

impl MistStack {
  #[inline(always)]
  pub fn push(&mut self, value: MistValue) {
    self.stack.push(value);
  }

  #[inline(always)]
  pub fn pop(&mut self) -> MistValue {
    self.stack.pop().expect("Stack is not empty.")
  }
}

impl MistStack {
  /// [crate::opcode::ACONST_NULL].
  pub fn aconst_null(&mut self) {
    self.push(MistValue::ObjectReference(0));
  }

  /// [crate::opcode::D2F].
  pub fn d2f(&mut self) {
    let double: f64 = self.pop().into();
    self.push(MistValue::Float(double as f32));
  }

  /// [crate::opcode::D2I].
  pub fn d2i(&mut self) {
    let double: f64 = self.pop().into();
    self.push(MistValue::Integer(double as i32));
  }

  /// [crate::opcode::D2L].
  pub fn d2l(&mut self) {
    let double: f64 = self.pop().into();
    self.push(MistValue::Long(double as i64));
  }

  /// [crate::opcode::DADD].
  pub fn dadd(&mut self) {
    let rhs: f64 = self.pop().into();
    let lhs: f64 = self.pop().into();
    self.push(MistValue::Double(lhs + rhs));
  }

  /// [crate::opcode::DCMPG].
  pub fn dcmpg(&mut self) {
    let rhs: f64 = self.pop().into();
    let lhs: f64 = self.pop().into();
    self.push(
      lhs
        .partial_cmp(&rhs)
        .try_into()
        .unwrap_or(MistValue::Integer(1)),
    )
  }

  /// [crate::opcode::DCMPL].
  pub fn dcmpl(&mut self) {
    let rhs: f64 = self.pop().into();
    let lhs: f64 = self.pop().into();
    self.push(
      lhs
        .partial_cmp(&rhs)
        .try_into()
        .unwrap_or(MistValue::Integer(-1)),
    )
  }

  /// Push double.
  pub fn dconst(&mut self, double: f64) {
    self.push(MistValue::Double(double));
  }

  /// [crate::opcode::DDIV].
  pub fn ddiv(&mut self) {
    let rhs: f64 = self.pop().into();
    let lhs: f64 = self.pop().into();
    self.push(MistValue::Double(lhs / rhs));
  }

  /// [crate::opcode::DMUL].
  pub fn dmul(&mut self) {
    let rhs: f64 = self.pop().into();
    let lhs: f64 = self.pop().into();
    self.push(MistValue::Double(lhs * rhs));
  }

  /// [crate::opcode::DNEG].
  pub fn dneg(&mut self) {
    let double: f64 = self.pop().into();
    self.push(MistValue::Double(double.neg()));
  }

  /// [crate::opcode::DREM].
  pub fn drem(&mut self) {
    let rhs: f64 = self.pop().into();
    let lhs: f64 = self.pop().into();
    self.push(MistValue::Double(lhs % rhs));
  }

  /// [crate::opcode::DSUB].
  pub fn dsub(&mut self) {
    let rhs: f64 = self.pop().into();
    let lhs: f64 = self.pop().into();
    self.push(MistValue::Double(lhs - rhs));
  }

  /// [crate::opcode::DUP].
  pub fn dup(&mut self) {
    let value = self.pop();
    self.push(value.clone());
    self.push(value);
  }

  /// [crate::opcode::DUP_X1].
  pub fn dup_x1(&mut self) {
    let value1 = self.pop();
    let value2 = self.pop();
    self.push(value1.clone());
    self.push(value2);
    self.push(value1);
  }

  /// [crate::opcode::DUP2].
  pub fn dup2(&mut self) {
    todo!();
  }

  /// [crate::opcode::F2D].
  pub fn f2d(&mut self) {
    let float: f32 = self.pop().into();
    self.push(MistValue::Double(float as f64));
  }

  /// [crate::opcode::F2L]
  pub fn f2l(&mut self) {
    let float: f32 = self.pop().into();
    self.push(MistValue::Long(float as i64));
  }

  /// [crate::opcode::FADD].
  pub fn fadd(&mut self) {
    let rhs: f32 = self.pop().into();
    let lhs: f32 = self.pop().into();
    self.push(MistValue::Float(lhs + rhs));
  }

  /// [crate::opcode::FCMPG].
  pub fn fcmpg(&mut self) {
    let rhs: f32 = self.pop().into();
    let lhs: f32 = self.pop().into();
    self.push(
      lhs
        .partial_cmp(&rhs)
        .try_into()
        .unwrap_or(MistValue::Integer(1)),
    )
  }

  /// [crate::opcode::FCMPL].
  pub fn fcmpl(&mut self) {
    let rhs: f32 = self.pop().into();
    let lhs: f32 = self.pop().into();
    self.push(
      lhs
        .partial_cmp(&rhs)
        .try_into()
        .unwrap_or(MistValue::Integer(-1)),
    )
  }

  /// Push float.
  pub fn fconst(&mut self, float: f32) {
    self.push(MistValue::Float(float));
  }

  /// [crate::opcode::FDIV].
  pub fn fdiv(&mut self) {
    let rhs: f32 = self.pop().into();
    let lhs: f32 = self.pop().into();
    self.push(MistValue::Float(lhs / rhs));
  }

  /// [crate::opcode::FMUL].
  pub fn fmul(&mut self) {
    let rhs: f32 = self.pop().into();
    let lhs: f32 = self.pop().into();
    self.push(MistValue::Float(lhs * rhs));
  }

  /// [crate::opcode::FNEG].
  pub fn fneg(&mut self) {
    let float: f32 = self.pop().into();
    self.push(MistValue::Float(float.neg()));
  }

  /// [crate::opcode::FREM].
  pub fn frem(&mut self) {
    let rhs: f32 = self.pop().into();
    let lhs: f32 = self.pop().into();
    self.push(MistValue::Float(lhs % rhs));
  }

  /// [crate::opcode::FSUB].
  pub fn fsub(&mut self) {
    let rhs: f32 = self.pop().into();
    let lhs: f32 = self.pop().into();
    self.push(MistValue::Float(lhs - rhs));
  }

  /// [crate::opcode::I2B].
  pub fn i2b(&mut self) {
    let int: i32 = self.pop().into();
    self.push(MistValue::Byte(int as i8));
  }

  /// [crate::opcode::I2C].
  pub fn i2c(&mut self) {
    let int: i32 = self.pop().into();
    let chr = char::from_u32(int as u32).expect("Convert i32 to char");
    self.push(MistValue::Char(chr));
  }

  /// [crate::opcode::I2D].
  pub fn i2d(&mut self) {
    let int: i32 = self.pop().into();
    self.push(MistValue::Double(int as f64))
  }

  /// [crate::opcode::I2F].
  pub fn i2f(&mut self) {
    let int: i32 = self.pop().into();
    self.push(MistValue::Float(int as f32));
  }

  /// [crate::opcode::I2L].
  pub fn i2l(&mut self) {
    let int: i32 = self.pop().into();
    self.push(MistValue::Long(int as i64));
  }

  /// [crate::opcode::I2S].
  pub fn i2s(&mut self) {
    let int: i32 = self.pop().into();
    self.push(MistValue::Short(int as i16));
  }

  /// [crate::opcode::IADD].
  pub fn iadd(&mut self) {
    let rhs: i32 = self.pop().into();
    let lhs: i32 = self.pop().into();
    self.push(MistValue::Integer(lhs + rhs));
  }

  /// [crate::opcode::IAND].
  pub fn iand(&mut self) {
    let rhs: i32 = self.pop().into();
    let lhs: i32 = self.pop().into();
    self.push(MistValue::Integer(lhs & rhs));
  }

  /// Push int constant.
  pub fn iconst(&mut self, value: i32) {
    self.push(MistValue::Integer(value));
  }

  /// [crate::opcode::IDIV].
  pub fn idiv(&mut self) {
    let rhs: i32 = self.pop().into();
    let lhs: i32 = self.pop().into();
    self.push(MistValue::Integer(lhs / rhs));
  }

  /// [crate::opcode::IMUL].
  pub fn imul(&mut self) {
    let rhs: i32 = self.pop().into();
    let lhs: i32 = self.pop().into();
    self.push(MistValue::Integer(lhs * rhs));
  }

  /// [crate::opcode::INEG].
  pub fn ineg(&mut self) {
    let int: i32 = self.pop().into();
    self.push(MistValue::Integer(int.neg()));
  }

  /// [crate::opcode::IOR].
  pub fn ior(&mut self) {
    let rhs: i32 = self.pop().into();
    let lhs: i32 = self.pop().into();
    self.push(MistValue::Integer(lhs | rhs));
  }

  /// [crate::opcode::IREM].
  pub fn irem(&mut self) {
    let rhs: i32 = self.pop().into();
    let lhs: i32 = self.pop().into();
    self.push(MistValue::Integer(lhs % rhs));
  }

  /// [crate::opcode::ISHL].
  pub fn ishl(&mut self) {
    let rhs: i32 = self.pop().into();
    let lhs: i32 = self.pop().into();
    self.push(MistValue::Integer(lhs << rhs));
  }

  /// [crate::opcode::ISHR].
  pub fn ishr(&mut self) {
    let rhs: i32 = self.pop().into();
    let lhs: i32 = self.pop().into();
    self.push(MistValue::Integer(lhs >> rhs));
  }

  /// [crate::opcode::ISUB].
  pub fn isub(&mut self) {
    let rhs: i32 = self.pop().into();
    let lhs: i32 = self.pop().into();
    self.push(MistValue::Integer(lhs >> rhs));
  }

  /// [crate::opcode::IUSHR].
  pub fn iushr(&mut self) {
    let rhs: i32 = self.pop().into();
    let lhs: i32 = self.pop().into();
    let rhs = rhs as u32;
    let lhs = lhs as u32;
    self.push(MistValue::Integer((lhs >> rhs) as i32));
  }

  /// [crate::opcode::IXOR].
  pub fn ixor(&mut self) {
    let rhs: i32 = self.pop().into();
    let lhs: i32 = self.pop().into();
    self.push(MistValue::Integer(lhs ^ rhs));
  }

  /// [crate::opcode::L2D].
  pub fn l2d(&mut self) {
    let long: i64 = self.pop().into();
    self.push(MistValue::Double(long as f64));
  }

  /// [crate::opcode::L2F].
  pub fn l2f(&mut self) {
    let long: i64 = self.pop().into();
    self.push(MistValue::Float(long as f32));
  }

  /// [crate::opcode::L2F].
  pub fn l2i(&mut self) {
    let long: i64 = self.pop().into();
    self.push(MistValue::Integer(long as i32));
  }

  /// [crate::opcode::LADD].
  pub fn ladd(&mut self) {
    let rhs: i64 = self.pop().into();
    let lhs: i64 = self.pop().into();
    self.push(MistValue::Long(lhs + rhs));
  }

  /// [crate::opcode::LAND].
  pub fn land(&mut self) {
    let rhs: i64 = self.pop().into();
    let lhs: i64 = self.pop().into();
    self.push(MistValue::Long(lhs & rhs));
  }

  /// Push long.
  pub fn lconst(&mut self, long: i64) {
    self.push(MistValue::Long(long));
  }

  /// [crate::opcode::LDIV].
  pub fn ldiv(&mut self) {
    let rhs: i64 = self.pop().into();
    let lhs: i64 = self.pop().into();
    self.push(MistValue::Long(lhs / rhs));
  }

  /// [crate::opcode::LMUL].
  pub fn lmul(&mut self) {
    let rhs: i64 = self.pop().into();
    let lhs: i64 = self.pop().into();
    self.push(MistValue::Long(lhs * rhs));
  }

  /// [crate::opcode::LNEG].
  pub fn lneg(&mut self) {
    let long: i64 = self.pop().into();
    self.push(MistValue::Long(long.neg()));
  }

  /// [crate::opcode::LOR].
  pub fn lor(&mut self) {
    let rhs: i64 = self.pop().into();
    let lhs: i64 = self.pop().into();
    self.push(MistValue::Long(lhs | rhs));
  }

  /// [crate::opcode::LREM].
  pub fn lrem(&mut self) {
    let rhs: i64 = self.pop().into();
    let lhs: i64 = self.pop().into();
    self.push(MistValue::Long(lhs % rhs));
  }

  /// [crate::opcode::LSHL].
  pub fn lshl(&mut self) {
    let rhs: i64 = self.pop().into();
    let lhs: i64 = self.pop().into();
    self.push(MistValue::Long(lhs << rhs));
  }

  /// [crate::opcode::LSHR].
  pub fn lshr(&mut self) {
    let rhs: i64 = self.pop().into();
    let lhs: i64 = self.pop().into();
    self.push(MistValue::Long(lhs >> rhs));
  }

  /// [crate::opcode::LSUB].
  pub fn lsub(&mut self) {
    let rhs: i64 = self.pop().into();
    let lhs: i64 = self.pop().into();
    self.push(MistValue::Long(lhs - rhs));
  }

  /// [crate::opcode::LUSHR].
  pub fn lushr(&mut self) {
    let rhs: i64 = self.pop().into();
    let lhs: i64 = self.pop().into();
    let rhs = rhs as u64;
    let lhs = lhs as u64;
    self.push(MistValue::Long((lhs >> rhs) as i64));
  }

  /// [crate::opcode::LXOR].
  pub fn lxor(&mut self) {
    let rhs: i64 = self.pop().into();
    let lhs: i64 = self.pop().into();
    self.push(MistValue::Long(lhs ^ rhs));
  }
}
