use std::fs::File;

use crate::run::RuntimeContext;

pub mod class;
pub mod local;
pub mod opcode;
pub mod run;
pub mod stack;
pub mod value;

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

  let file = File::open("./App.class").unwrap();
  let mut reader = class::Reader::new(file);
  let (magic, minor_version, major_version, constant_pool_count) =
    reader.read_header().unwrap();
  println!(
    "{magic:x}, {minor_version:x}, {major_version:x}, {constant_pool_count:x}"
  );
  reader.read_constant_pool(constant_pool_count).unwrap();
}
