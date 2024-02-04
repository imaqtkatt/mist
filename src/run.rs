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
          if value <= 0 {
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

        opcode::IINC => unimplemented!(),

        opcode::ILOAD => {
          let load = self.local.load(self.fetch(&mut ip) as usize);
          stack.push(load);
        }
        opcode::ILOAD_0 => stack.push(self.local.load(0)),
        opcode::ILOAD_1 => stack.push(self.local.load(1)),
        opcode::ILOAD_2 => stack.push(self.local.load(2)),
        opcode::ILOAD_3 => stack.push(self.local.load(3)),

        opcode::IMUL => stack.imul(),

        opcode::INEG => stack.ineg(),

        opcode::INSTANCEOF => unimplemented!(),

        opcode::INVOKEDYNAMIC => unimplemented!(),

        opcode::INVOKEINTERFACE => unimplemented!(),

        opcode::INVOKESPECIAL => unimplemented!(),

        opcode::INVOKESTATIC => unimplemented!(),

        opcode::INVOKEVIRTUAL => unimplemented!(),

        opcode::IOR => stack.ior(),

        opcode::IREM => stack.irem(),

        // The current method must have return type boolean, byte, short, char, or int.
        opcode::IRETURN => break Some(stack.pop()),

        opcode::ISHL => stack.ishl(),

        opcode::ISHR => stack.ishr(),

        opcode::ISTORE => {
          let index = self.fetch(&mut ip) as usize;
          self.local.store(index, stack.pop());
        }
        opcode::ISTORE_0 => self.local.store(0, stack.pop()),
        opcode::ISTORE_1 => self.local.store(1, stack.pop()),
        opcode::ISTORE_2 => self.local.store(2, stack.pop()),
        opcode::ISTORE_3 => self.local.store(3, stack.pop()),

        opcode::ISUB => stack.isub(),

        opcode::IUSHR => stack.iushr(),

        opcode::JSR => unimplemented!(),

        opcode::JSR_W => unimplemented!(),

        opcode::L2D => stack.l2d(),
        opcode::L2F => stack.l2f(),
        opcode::L2I => stack.l2i(),

        opcode::LADD => stack.ladd(),

        opcode::LALOAD => unimplemented!(),

        opcode::LAND => stack.land(),

        opcode::LASTORE => unimplemented!(),

        opcode::LCMP => stack.lcmp(),

        opcode::LCONST_0 => stack.lconst(0i64),
        opcode::LCONST_1 => stack.lconst(1i64),

        opcode::LDC => unimplemented!(),
        opcode::LDC_W => unimplemented!(),
        opcode::LDC2_W => unimplemented!(),

        opcode::LDIV => stack.ldiv(),

        opcode::LLOAD => {
          let load = self.local.load(self.fetch(&mut ip) as usize);
          stack.push(load);
        }
        opcode::LLOAD_0 => stack.push(self.local.load(0)),
        opcode::LLOAD_1 => stack.push(self.local.load(1)),
        opcode::LLOAD_2 => stack.push(self.local.load(2)),
        opcode::LLOAD_3 => stack.push(self.local.load(3)),

        opcode::LMUL => stack.lmul(),

        opcode::LNEG => stack.lneg(),

        opcode::LOOKUPSWTICH => unimplemented!(),

        opcode::LOR => stack.lor(),

        opcode::LREM => stack.lrem(),

        opcode::LRETURN => break Some(stack.pop()),

        opcode::LSHL => stack.lshl(),

        opcode::LSHR => stack.lshr(),

        opcode::LSTORE => {
          let index = self.fetch(&mut ip) as usize;
          self.local.store(index, stack.pop());
        }
        opcode::LSTORE_0 => self.local.store(0, stack.pop()),
        opcode::LSTORE_1 => self.local.store(1, stack.pop()),
        opcode::LSTORE_2 => self.local.store(2, stack.pop()),
        opcode::LSTORE_3 => self.local.store(3, stack.pop()),

        opcode::LSUB => stack.lsub(),

        opcode::LXOR => stack.lxor(),

        opcode::MONITORENTER => unimplemented!(),
        opcode::MONITOREXIT => unimplemented!(),

        opcode::MULTIANEWARRAY => unimplemented!(),

        opcode::NEW => unimplemented!(),

        opcode::NOP => {}

        opcode::POP => _ = stack.pop(),

        opcode::POP2 => {
          // TODO: where each of value1 and value2 is a value of a category 1 computational type
          _ = stack.pop();
          _ = stack.pop();
        }

        opcode::PUTFIELD => unimplemented!(),

        opcode::PUTSTATIC => unimplemented!(),

        opcode::RET => {
          let _index = self.fetch(&mut ip);
          unimplemented!();
        }

        opcode::SALOAD => unimplemented!(),

        opcode::SASTORE => unimplemented!(),

        opcode::SIPUSH => {
          let byte1 = self.program[ip] as u32;
          let byte2 = self.program[ip + 1] as u32;
          let value = (byte1 << 8 | byte2) as i16;
          stack.push(MistValue::Short(value));
        }

        opcode::SWAP => {
          let value1 = stack.pop();
          let value2 = stack.pop();
          stack.push(value1);
          stack.push(value2);
        }

        opcode::TABLESWITCH => unimplemented!(),

        opcode::WIDE => unimplemented!(),

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
