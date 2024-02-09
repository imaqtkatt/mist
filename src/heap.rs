use crate::value::MistValue;

#[derive(Debug)]
pub struct Heap {
  memory: Vec<Object>,
}

#[derive(Debug)]
pub struct Object {
  pub id: usize,
  pub class: usize,
  pub mem: Vec<MistValue>,
}

impl Object {
  pub fn null() -> Self {
    Self {
      id: 0,
      class: 0,
      mem: Vec::new(),
    }
  }
}

impl Heap {
  pub fn new() -> Self {
    Self {
      memory: vec![Object::null()],
    }
  }
}

impl Heap {
  pub fn newarray(&mut self, _atype: u8, count: i32) -> usize {
    let id = self.memory.len();
    self.memory.push(Object {
      id,
      class: 0, // FIXME
      mem: vec![MistValue::Integer(0); count as usize],
    });
    id
  }

  pub fn iastore(&mut self, arrayref: usize, index: i32, value: i32) {
    let object = &mut self.memory[arrayref];
    object.mem[index as usize] = MistValue::Integer(value);
  }

  pub fn get(&self, arrayref: usize, index: i32) -> MistValue {
    self.memory[arrayref].mem[index as usize]
  }
}
