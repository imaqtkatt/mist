use std::{env, fs::File};

use crate::run::RuntimeContext;

pub mod class;
pub mod local;
pub mod opcode;
pub mod run;
pub mod stack;
pub mod value;

fn main() {
  let args = env::args().collect::<Vec<String>>();

  if let Err(e) = run(&args) {
    eprintln!("{e}");
  }
}

fn run(args: &[String]) -> std::io::Result<()> {
  let file = File::open(&args[1])?;
  let mut reader = class::Reader::new(file);
  let class = reader.read_class()?;
  let class_name = class.this_class.clone();
  // println!("{class:?}");

  let mut ctx = class::context::Context::new();
  ctx.add_class(class);

  let result = RuntimeContext::boot(&ctx, &class_name);
  println!("result: {result:?}");

  Ok(())
}
