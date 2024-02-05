use std::fs::File;

use crate::{
  class::pool::{Entry, Utf8Info},
  run::RuntimeContext,
};

pub mod class;
pub mod local;
pub mod opcode;
pub mod run;
pub mod stack;
pub mod value;

fn main() {
  let file = File::open("./App.class").unwrap();
  let mut reader = class::Reader::new(file);
  let class = reader.read_class().unwrap();
  println!("{class:?}");

  let main_method = &class.methods[1];
  let attribute_info = &main_method.attribute_info[0];

  let info = &attribute_info.info;

  println!("info: {info:?}");

  let mut rt = RuntimeContext::new(info);
  rt.run();
}
