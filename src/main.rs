use std::fs::File;

use crate::run::RuntimeContext;

pub mod class;
pub mod local;
pub mod opcode;
pub mod run;
pub mod stack;
pub mod value;

fn main() {
  if let Err(e) = run() {
    eprintln!("{e}");
  }
}

fn run() -> std::io::Result<()> {
  let file = File::open("./App.class")?;
  let mut reader = class::Reader::new(file);
  let class = reader.read_class()?;
  println!("{class:?}");

  let main_method = &class.methods[1];
  let attribute_info = &main_method.attribute_info[0];

  let info = &attribute_info.info;

  let mut rt = RuntimeContext::new(info);
  let result = rt.run();
  println!("result: {result:?}");

  Ok(())
}
