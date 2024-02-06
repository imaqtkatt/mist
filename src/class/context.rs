use std::{collections::HashMap, time::SystemTime};

use crate::value::MistValue;

use super::{
  attribute_info::{AttributeInfo, Code},
  method::MethodInfo,
  Class,
};

pub struct Context {
  classes: HashMap<String, Class>,
}

impl Context {
  pub fn new() -> Self {
    let mut this = Self {
      classes: HashMap::new(),
    };

    let mut foo = Class::default();
    foo.methods.push(MethodInfo {
      access_flags: 0,
      name: String::from("currentTimeMillis"),
      descriptor: String::from("()J"),
      attributes: vec![AttributeInfo::Code(Code::new_native(Box::new(|_| {
        SystemTime::now()
          .duration_since(SystemTime::UNIX_EPOCH)
          .map(|duration| MistValue::Long(duration.as_millis() as i64))
          .ok()
      })))],
    });

    this.classes.insert(String::from("java/lang/System"), foo);
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
