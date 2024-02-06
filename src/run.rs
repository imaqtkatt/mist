use crate::{
  class::{
    self, attribute_info,
    pool::{self, Entry},
  },
  local::Local,
  opcode,
  stack::MistStack,
  value::MistValue,
};

const MAIN: &str = "main";
const MAIN_DESCRIPTOR: &str = "([Ljava/lang/String;)J";

pub struct RuntimeContext<'bytecode> {
  local: Local,
  program: &'bytecode [u8],
  context: &'bytecode class::context::Context,
}

impl<'bytecode> RuntimeContext<'bytecode> {
  pub fn new(
    program: &'bytecode [u8],
    context: &'bytecode class::context::Context,
    local: Local,
  ) -> Self {
    Self {
      program,
      context,
      local,
    }
  }

  pub fn boot(
    context: &'bytecode class::context::Context,
    main_class: &str,
  ) -> Option<MistValue> {
    let this_class = context.lookup_class(main_class)?;
    let main =
      this_class.lookup_method_with_descriptor(MAIN, MAIN_DESCRIPTOR)?;

    let attribute_info::AttributeInfo::Code(code) = &main.attributes[0] else {
      panic!()
    };
    Self::run_code(code, context, &this_class.constant_pool)
  }

  pub fn run_code(
    code: &'bytecode attribute_info::Code,
    context: &'bytecode class::context::Context,
    constant_pool: &[pool::Entry],
  ) -> Option<MistValue> {
    let local = Local::new(code.max_local as usize);
    let stack = MistStack::new(code.max_stack as usize);

    if code.is_native() {
      return code.native.clone().unwrap()(&local);
    } else {
      let mut rt = Self::new(&code.code, &context, local);
      rt.run(stack, constant_pool)
    }
  }
}

impl<'bytecode> RuntimeContext<'bytecode> {
  fn run(
    &mut self,
    mut stack: MistStack,
    constant_pool: &[pool::Entry],
  ) -> Option<MistValue> {
    let mut ip = 0;

    loop {
      let instruction = self.fetch(&mut ip);

      println!("{instruction:x}");
      match instruction {
        opcode::ACONST_NULL => stack.aconst_null(),

        opcode::ALOAD => {
          let index = self.fetch(&mut ip) as usize;
          stack.push(self.local.load(index));
        }
        opcode::ALOAD_0 => stack.push(self.local.load(0)),
        opcode::ALOAD_1 => stack.push(self.local.load(1)),
        opcode::ALOAD_2 => stack.push(self.local.load(2)),
        opcode::ALOAD_3 => stack.push(self.local.load(3)),

        opcode::ARETURN => break Some(stack.pop()),

        opcode::ASTORE => {
          let index = self.fetch(&mut ip);
          self.local.store(index as usize, stack.pop());
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
          let index = self.fetch(&mut ip) as usize;
          stack.push(self.local.load(index));
        }
        opcode::DLOAD_0 => stack.push(self.local.load(0)),
        opcode::DLOAD_1 => stack.push(self.local.load(1)),
        opcode::DLOAD_2 => stack.push(self.local.load(2)),
        opcode::DLOAD_3 => stack.push(self.local.load(3)),

        opcode::DMUL => stack.dmul(),

        opcode::DNEG => stack.dneg(),

        opcode::DREM => stack.drem(),

        opcode::DRETURN => break Some(stack.pop()),

        opcode::DSTORE => {
          let index = self.fetch(&mut ip);
          self.local.store(index as usize, stack.pop());
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
          let index = self.fetch(&mut ip) as usize;
          stack.push(self.local.load(index));
        }
        opcode::FLOAD_0 => stack.push(self.local.load(0)),
        opcode::FLOAD_1 => stack.push(self.local.load(1)),
        opcode::FLOAD_2 => stack.push(self.local.load(2)),
        opcode::FLOAD_3 => stack.push(self.local.load(3)),

        opcode::FMUL => stack.fmul(),

        opcode::FNEG => stack.fneg(),

        opcode::FREM => stack.frem(),

        opcode::FRETURN => break Some(stack.pop()),

        opcode::FSTORE => {
          let index = self.fetch(&mut ip);
          self.local.store(index as usize, stack.pop());
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

        opcode::IINC => {
          let index = self.fetch(&mut ip) as usize;
          let r#const = self.fetch(&mut ip) as i32;
          self.local.iinc(index, r#const);
        }

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

        opcode::INVOKESTATIC => {
          let indexbyte1 = self.fetch(&mut ip) as u16;
          let indexbyte2 = self.fetch(&mut ip) as u16;
          let class_index = indexbyte1 << 8 | indexbyte2;
          let pool::Entry::MethodRefInfo {
            class_index,
            name_and_type_index,
          } = &constant_pool[class_index as usize]
          else {
            panic!()
          };

          let pool::Entry::ClassInfo { name_index } =
            &constant_pool[*class_index as usize]
          else {
            unreachable!()
          };
          let pool::Entry::NameAndTypeInfo {
            index,
            descriptor_index,
          } = &constant_pool[*name_and_type_index as usize]
          else {
            unreachable!()
          };

          let Entry::Utf8Info { bytes: class_name } =
            &constant_pool[*name_index as usize]
          else {
            panic!();
          };
          let Entry::Utf8Info { bytes: method_name } =
            &constant_pool[*index as usize]
          else {
            panic!();
          };
          let Entry::Utf8Info { bytes: descriptor } =
            &constant_pool[*descriptor_index as usize]
          else {
            panic!();
          };

          let method =
            self
              .context
              .lookup_method(class_name, method_name, descriptor);

          if let Some(method) = method {
            let attribute_info::AttributeInfo::Code(code) =
              &method.attributes[0]
            else {
              panic!();
            };

            if let Some(ret) =
              RuntimeContext::run_code(code, self.context, constant_pool)
            {
              stack.push(ret);
            }
          } else {
            panic!("Could not find method '{method_name:?}'");
          }
        }

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

        opcode::LDC => {
          let index = self.fetch(&mut ip);
          match &constant_pool[index as usize] {
            Entry::IntegerInfo { bytes } => stack.iconst(*bytes as i32),
            _ => unimplemented!(),
          }
        }
        opcode::LDC_W => unimplemented!(),
        opcode::LDC2_W => unimplemented!(),

        opcode::LDIV => stack.ldiv(),

        opcode::LLOAD => {
          let index = self.fetch(&mut ip) as usize;
          stack.push(self.local.load(index));
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

        opcode::RETURN => break None,

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
