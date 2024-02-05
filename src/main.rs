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
  println!("{:?}", class);

  let result = RuntimeContext::boot_class(&class, "main");
  println!("result: {result:?}");

  Ok(())
}
