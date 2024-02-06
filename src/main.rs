use std::{collections::HashMap, fs::File};

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

  let mut ctx = class::context::Context::new(HashMap::new());
  ctx.add_class(class);

  let result = RuntimeContext::boot(&ctx, "App", "main");
  println!("result: {result:?}");

  Ok(())
}
