use std::collections::HashMap;

use super::{pool::Entry, Class};

pub struct Context {
  classes: HashMap<String, Class>,
}

impl Context {
  pub fn new(classes: HashMap<String, Class>) -> Self {
    Self { classes }
  }
}

impl Context {
  pub fn lookup_class(&self, class_name: &str) -> Option<&Class> {
    self.classes.get(class_name)
  }

  pub fn add_class(&mut self, class: Class) {
    let Entry::Utf8Info { bytes } = class.class_name().unwrap() else {
      panic!()
    };

    match self.classes.entry(bytes.to_string()) {
      std::collections::hash_map::Entry::Occupied(_) => todo!(),
      std::collections::hash_map::Entry::Vacant(e) => {
        e.insert(class);
      }
    }
  }
}
