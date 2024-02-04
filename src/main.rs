use crate::run::RuntimeContext;

pub mod local;
pub mod opcode;
pub mod run;
pub mod stack;
pub mod value;
pub mod class;

fn main() {
  let mut rt = RuntimeContext::new(&[
    opcode::GOTO,
    0x00,
    0x05,
    //
    opcode::ACONST_NULL,
    opcode::ARETURN,
    //
    opcode::ICONST_0,
    opcode::IFEQ,
    0x00,
    0x03,
  ]);
  let res = rt.run();

  println!("Hello, world! {res:?}");
}
