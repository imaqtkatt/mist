use super::attribute_info::AttributeInfo;

#[derive(Clone, Debug)]
pub struct FieldInfo {
  pub access_flags: u16,
  pub name: String,
  pub descriptor: String,
  pub attributes: Vec<AttributeInfo>,
}
