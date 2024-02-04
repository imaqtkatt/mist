#[derive(Clone, Debug)]
pub struct MethodInfo {
  pub access_flags: u16,
  pub name_index: u16,
  pub descriptor_index: u16,
  pub attributes_count: u16,
  pub attribute_info: Vec<AttributeInfo>,
}

#[derive(Clone, Debug)]
pub struct AttributeInfo {
  pub attribute_name_index: u16,
  pub attribute_length: u32,
  pub info: Vec<u8>,
}
