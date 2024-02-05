use std::fs::File;

use crate::{class::attribute_info, run::RuntimeContext};

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
  // println!("{class:?}");

  let main_method = class.find_method("main").unwrap();
  let info = &main_method.attributes[0].info;

  match info {
    attribute_info::Info::Code(code) => {
      if let Some(result) = RuntimeContext::run_code(code) {
        println!("result: {result:?}");
      }
    }
    _ => eprintln!("Is not code"),
  }

  Ok(())
}
