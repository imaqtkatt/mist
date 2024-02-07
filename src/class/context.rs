use std::collections::HashMap;

use super::{
  attribute_info::{AttributeInfo, Code},
  method::MethodInfo,
  native::*,
  Class,
};

#[derive(Default)]
pub struct Context {
  classes: HashMap<String, Class>,
}

impl Context {
  pub fn new() -> Self {
    let mut this = Self::default();

    // TODO: refactor this
    let mut java_lang_system = Class::default();
    java_lang_system.methods.push(MethodInfo {
      access_flags: 0,
      name: String::from("currentTimeMillis"),
      descriptor: String::from("()J"),
      attributes: vec![AttributeInfo::Code(Code::native(
        java::lang::system::current_time_millis,
        0,
      ))],
    });

    let mut java_lang_math = Class::default();
    java_lang_math.methods.push(MethodInfo {
      access_flags: 0,
      name: String::from("sqrt"),
      descriptor: String::from("(D)D"),
      attributes: vec![AttributeInfo::Code(Code::native(
        java::lang::math::sqrt,
        1,
      ))],
    });

    this
      .classes
      .insert(String::from("java/lang/System"), java_lang_system);
    this
      .classes
      .insert(String::from("java/lang/Math"), java_lang_math);
    this
  }
}

impl Context {
  pub fn lookup_class(&self, class_name: &str) -> Option<&Class> {
    self.classes.get(class_name)
  }

  pub fn add_class(&mut self, class: Class) {
    let class_name = class.this_class.clone();

    match self.classes.entry(class_name) {
      std::collections::hash_map::Entry::Occupied(_) => todo!(),
      std::collections::hash_map::Entry::Vacant(e) => {
        e.insert(class);
      }
    }
  }

  pub fn lookup_method(
    &self,
    class_name: &str,
    method_name: &str,
    descriptor: &str,
  ) -> Option<&MethodInfo> {
    println!("{method_name} {descriptor}");
    let class = self.lookup_class(class_name)?;

    class.lookup_method_with_descriptor(method_name, descriptor)
  }
}
