use super::attribute_info::AttributeInfo;

#[derive(Clone, Debug)]
pub struct FieldInfo {
  pub access_flags: u16,
  pub name_index: u16,
  pub descriptor_index: u16,
  pub attributes_count: u16,
  pub attributes: Vec<AttributeInfo>,
}
