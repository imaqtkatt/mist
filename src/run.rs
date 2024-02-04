use crate::{local::Local, opcode, stack::MistStack, value::MistValue};

pub struct RuntimeContext<'bytecode> {
  local: Local,
  program: &'bytecode [u8],
}

impl<'bytecode> RuntimeContext<'bytecode> {
  pub fn new(program: &'bytecode [u8]) -> Self {
    Self {
      program,
      local: Local::new(1 << 10),
    }
  }
}

impl<'bytecode> RuntimeContext<'bytecode> {
  pub fn run(&mut self) -> Option<MistValue> {
    let mut ip = 0;
    let mut stack = MistStack::new();

    loop {
      let instruction = self.fetch(&mut ip);

      println!("{instruction:x}");
      match instruction {
        opcode::ACONST_NULL => stack.aconst_null(),

        opcode::ALOAD => {
          let load = self.local.load(self.fetch(&mut ip) as usize);
          stack.push(load);
        }

        opcode::ALOAD_0 => {
          let load = self.local.load(0);
          stack.push(load);
        }

        opcode::ALOAD_1 => {
          let load = self.local.load(1);
          stack.push(load);
        }

        opcode::ALOAD_2 => {
          let load = self.local.load(2);
          stack.push(load);
        }

        opcode::ALOAD_3 => {
          let load = self.local.load(3);
          stack.push(load);
        }

        opcode::ARETURN => break Some(stack.pop()),

        opcode::ASTORE => {
          let store = self.fetch(&mut ip);
          self.local.store(store as usize, stack.pop());
        }
        opcode::ASTORE_0 => self.local.store(0, stack.pop()),
        opcode::ASTORE_1 => self.local.store(1, stack.pop()),
        opcode::ASTORE_2 => self.local.store(2, stack.pop()),
        opcode::ASTORE_3 => self.local.store(3, stack.pop()),

        opcode::ATHROW => unimplemented!(),

        opcode::BALOAD => unimplemented!(),

        opcode::BASTORE => unimplemented!(),

        opcode::BIPUSH => stack.iconst(self.fetch(&mut ip) as i32),

        opcode::CALOAD => unimplemented!(),

        opcode::CASTORE => unimplemented!(),

        opcode::CHECKCAST => unimplemented!(),

        opcode::D2F => stack.d2f(),
        opcode::D2I => stack.d2i(),
        opcode::D2L => stack.d2l(),

        opcode::DADD => stack.dadd(),

        opcode::DALOAD => unimplemented!(),

        opcode::DASTORE => unimplemented!(),

        opcode::DCMPG => stack.dcmpg(),
        opcode::DCMPL => stack.dcmpl(),

        opcode::DCONST_0 => stack.dconst(0f64),
        opcode::DCONST_1 => stack.dconst(1f64),

        opcode::DDIV => stack.ddiv(),

        opcode::DLOAD => {
          let load = self.local.load(self.fetch(&mut ip) as usize);
          stack.push(load);
        }
        opcode::DLOAD_0 => {
          let load = self.local.load(0);
          stack.push(load);
        }
        opcode::DLOAD_1 => {
          let load = self.local.load(1);
          stack.push(load);
        }
        opcode::DLOAD_2 => {
          let load = self.local.load(2);
          stack.push(load);
        }
        opcode::DLOAD_3 => {
          let load = self.local.load(3);
          stack.push(load);
        }

        opcode::DMUL => stack.dmul(),

        opcode::DNEG => stack.dneg(),

        opcode::DREM => stack.drem(),

        opcode::DRETURN => break Some(stack.pop()),

        opcode::DSTORE => {
          let store = self.fetch(&mut ip);
          self.local.store(store as usize, stack.pop());
        }
        opcode::DSTORE_0 => self.local.store(0, stack.pop()),
        opcode::DSTORE_1 => self.local.store(1, stack.pop()),
        opcode::DSTORE_2 => self.local.store(2, stack.pop()),
        opcode::DSTORE_3 => self.local.store(3, stack.pop()),

        opcode::DSUB => stack.dsub(),

        opcode::DUP => stack.dup(),
        opcode::DUP_X1 => stack.dup(),
        opcode::DUP_X2 => unimplemented!(),
        opcode::DUP2_X1 => unimplemented!(),
        opcode::DUP2_X2 => unimplemented!(),

        opcode::F2D => stack.f2d(),
        opcode::F2I => stack.f2i(),
        opcode::F2L => stack.f2l(),

        opcode::FADD => stack.fadd(),

        opcode::FALOAD => unimplemented!(),

        opcode::FASTORE => unimplemented!(),

        opcode::FCMPG => stack.fcmpg(),
        opcode::FCMPL => stack.fcmpl(),

        opcode::FCONSTF_0 => stack.fconst(0f32),
        opcode::FCONSTF_1 => stack.fconst(1f32),
        opcode::FCONSTF_2 => stack.fconst(2f32),

        opcode::FDIV => stack.fdiv(),

        opcode::FLOAD => {
          let load = self.local.load(self.fetch(&mut ip) as usize);
          stack.push(load);
        }
        opcode::FLOAD_0 => {
          let float = self.local.load(0);
          stack.push(float);
        }
        opcode::FLOAD_1 => {
          let float = self.local.load(1);
          stack.push(float);
        }
        opcode::FLOAD_2 => {
          let float = self.local.load(2);
          stack.push(float);
        }
        opcode::FLOAD_3 => {
          let float = self.local.load(3);
          stack.push(float);
        }

        opcode::FMUL => stack.fmul(),

        opcode::FNEG => stack.fneg(),

        opcode::FREM => stack.frem(),

        opcode::FRETURN => break Some(stack.pop()),

        opcode::FSTORE => {
          let store = self.fetch(&mut ip);
          self.local.store(store as usize, stack.pop());
        }
        opcode::FSTORE_0 => self.local.store(0, stack.pop()),
        opcode::FSTORE_1 => self.local.store(1, stack.pop()),
        opcode::FSTORE_2 => self.local.store(2, stack.pop()),
        opcode::FSTORE_3 => self.local.store(3, stack.pop()),

        opcode::FSUB => stack.fsub(),

        opcode::GETFIELD => unimplemented!(),

        opcode::GETSTATIC => unimplemented!(),

        // 16-bit branchoffset.
        opcode::GOTO => {
          let branchbyte1 = self.program[ip] as usize;
          let branchbyte2 = self.program[ip + 1] as usize;
          ip = branchbyte1 << 8 | branchbyte2;
        }

        // 32-bit branchoffset.
        opcode::GOTO_W => {
          let branchbyte1 = self.program[ip] as usize;
          let branchbyte2 = self.program[ip + 1] as usize;
          let branchbyte3 = self.program[ip + 2] as usize;
          let branchbyte4 = self.program[ip + 3] as usize;
          ip = branchbyte1 << 24
            | branchbyte2 << 16
            | branchbyte3 << 8
            | branchbyte4;
        }

        opcode::I2B => stack.i2b(),
        opcode::I2C => stack.i2c(),
        opcode::I2D => stack.i2d(),
        opcode::I2F => stack.i2f(),
        opcode::I2L => stack.i2l(),
        opcode::I2S => stack.i2s(),

        opcode::IADD => stack.iadd(),

        opcode::IADLOAD => unimplemented!(),

        opcode::IAND => stack.iand(),

        opcode::IASTORE => unimplemented!(),

        opcode::ICONST_M1 => stack.iconst(-1),
        opcode::ICONST_0 => stack.iconst(0),
        opcode::ICONST_1 => stack.iconst(1),
        opcode::ICONST_2 => stack.iconst(2),
        opcode::ICONST_3 => stack.iconst(3),
        opcode::ICONST_4 => stack.iconst(4),
        opcode::ICONST_5 => stack.iconst(5),

        opcode::IDIV => stack.idiv(),

        // Both values needs to be of type reference.
        opcode::IF_ACMPEQ => {
          let value1: usize = stack.pop().into();
          let value2: usize = stack.pop().into();

          if value1 == value2 {
            let branchbyte1 = self.program[ip] as usize;
            let branchbyte2 = self.program[ip + 1] as usize;
            ip = branchbyte1 << 8 | branchbyte2;
          } else {
            ip += 2;
          }
        }
        // Both values needs to be of type reference.
        opcode::IF_ACMPNE => {
          let value1: usize = stack.pop().into();
          let value2: usize = stack.pop().into();

          if value1 != value2 {
            let branchbyte1 = self.program[ip] as usize;
            let branchbyte2 = self.program[ip + 1] as usize;
            ip = branchbyte1 << 8 | branchbyte2;
          } else {
            ip += 2;
          }
        }

        // Both values needs to be of type int.
        opcode::IF_ICMPEQ => {
          let value1: i32 = stack.pop().into();
          let value2: i32 = stack.pop().into();

          if value1 == value2 {
            let branchbyte1 = self.program[ip] as usize;
            let branchbyte2 = self.program[ip + 1] as usize;
            ip = branchbyte1 << 8 | branchbyte2;
          } else {
            ip += 2;
          }
        }
        // Both values needs to be of type int.
        opcode::IF_ICMPNE => {
          let value1: i32 = stack.pop().into();
          let value2: i32 = stack.pop().into();

          if value1 != value2 {
            let branchbyte1 = self.program[ip] as usize;
            let branchbyte2 = self.program[ip + 1] as usize;
            ip = branchbyte1 << 8 | branchbyte2;
          } else {
            ip += 2;
          }
        }
        // Both values needs to be of type int.
        opcode::IF_ICMPLT => {
          let value1: i32 = stack.pop().into();
          let value2: i32 = stack.pop().into();

          if value1 < value2 {
            let branchbyte1 = self.program[ip] as usize;
            let branchbyte2 = self.program[ip + 1] as usize;
            ip = branchbyte1 << 8 | branchbyte2;
          } else {
            ip += 2;
          }
        }
        // Both values needs to be of type int.
        opcode::IF_ICMPGE => {
          let value1: i32 = stack.pop().into();
          let value2: i32 = stack.pop().into();
          if value1 >= value2 {
            let branchbyte1 = self.program[ip] as usize;
            let branchbyte2 = self.program[ip + 1] as usize;
            ip = branchbyte1 << 8 | branchbyte2;
          } else {
            ip += 2;
          }
        }
        // Both values needs to be of type int.
        opcode::IF_ICMPGT => {
          let value1: i32 = stack.pop().into();
          let value2: i32 = stack.pop().into();

          if value1 > value2 {
            let branchbyte1 = self.program[ip] as usize;
            let branchbyte2 = self.program[ip + 1] as usize;
            ip = branchbyte1 << 8 | branchbyte2;
          } else {
            ip += 2;
          }
        }

        // Both values needs to be of type int.
        opcode::IF_ICMPLE => {
          let value1: i32 = stack.pop().into();
          let value2: i32 = stack.pop().into();

          if value1 <= value2 {
            let branchbyte1 = self.program[ip] as usize;
            let branchbyte2 = self.program[ip + 1] as usize;
            ip = branchbyte1 << 8 | branchbyte2;
          } else {
            ip += 2;
          }
        }

        // Value needs to be of type int.
        opcode::IFEQ => {
          let value: i32 = stack.pop().into();
          if value == 0 {
            let branchbyte1 = self.program[ip] as usize;
            let branchbyte2 = self.program[ip + 1] as usize;
            ip = branchbyte1 << 8 | branchbyte2;
          } else {
            ip += 2;
          }
        }
        // Value needs to be of type int.
        opcode::IFNE => {
          let value: i32 = stack.pop().into();
          if value != 0 {
            let branchbyte1 = self.program[ip] as usize;
            let branchbyte2 = self.program[ip + 1] as usize;
            ip = branchbyte1 << 8 | branchbyte2;
          } else {
            ip += 2;
          }
        }
        // Value needs to be of type int.
        opcode::IFLT => {
          let value: i32 = stack.pop().into();
          if value < 0 {
            let branchbyte1 = self.program[ip] as usize;
            let branchbyte2 = self.program[ip + 1] as usize;
            ip = branchbyte1 << 8 | branchbyte2;
          } else {
            ip += 2;
          }
        }
        // Value needs to be of type int.
        opcode::IFGE => {
          let value: i32 = stack.pop().into();
          if value >= 0 {
            let branchbyte1 = self.program[ip] as usize;
            let branchbyte2 = self.program[ip + 1] as usize;
            ip = branchbyte1 << 8 | branchbyte2;
          } else {
            ip += 2;
          }
        }
        // Value needs to be of type int.
        opcode::IFGT => {
          let value: i32 = stack.pop().into();
          if value > 0 {
            let branchbyte1 = self.program[ip] as usize;
            let branchbyte2 = self.program[ip + 1] as usize;
            ip = branchbyte1 << 8 | branchbyte2;
          } else {
            ip += 2;
          }
        }
        // Value needs to be of type int.
        opcode::IFLE => {
          let value: i32 = stack.pop().into();
          if value > 0 {
            let branchbyte1 = self.program[ip] as usize;
            let branchbyte2 = self.program[ip + 1] as usize;
            ip = branchbyte1 << 8 | branchbyte2;
          } else {
            ip += 2;
          }
        }

        // Value needs to be of type reference.
        opcode::IFNONNULL => {
          let value: usize = stack.pop().into();
          if value > 0 {
            let branchbyte1 = self.program[ip] as usize;
            let branchbyte2 = self.program[ip + 1] as usize;
            ip = branchbyte1 << 8 | branchbyte2;
          } else {
            ip += 2;
          }
        }

        // Value needs to be of type reference.
        opcode::IFNULL => {
          let value: usize = stack.pop().into();
          if value == 0 {
            let branchbyte1 = self.program[ip] as usize;
            let branchbyte2 = self.program[ip + 1] as usize;
            ip = branchbyte1 << 8 | branchbyte2;
          } else {
            ip += 2;
          }
        }

        other => panic!("Found illegal bytecode '{other:x}'."),
      }
    }
  }

  #[inline(always)]
  fn fetch(&self, ip: &mut usize) -> u8 {
    let instruction = self.program[*ip];
    *ip += 1;
    instruction
  }
}
