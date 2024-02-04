pub const CLASS: u8 = 7;
pub const FIELD_REF: u8 = 9;
pub const METHOD_REF: u8 = 10;
pub const INTERFACE_METHOD_REF: u8 = 11;
pub const STRING: u8 = 8;
pub const INTEGER: u8 = 3;
pub const FLOAT: u8 = 4;
pub const LONG: u8 = 5;
pub const DOUBLE: u8 = 6;
pub const NAME_AND_TYPE: u8 = 12;
pub const UTF_8: u8 = 1;
pub const METHOD_HANDLE: u8 = 15;
pub const METHOD_TYPE: u8 = 16;
pub const INVOKE_DYNAMIC: u8 = 18;

#[derive(Clone, Debug)]
pub enum Entry {
  Utf8(Utf8Info),
  Integer(IntegerInfo),
  Float(FloatInfo),
  Long(LongInfo),
  Double(DoubleInfo),
  Class(ClassInfo),
  String(StringInfo),
  FieldRef(FieldRefInfo),
  MethodRef(MethodRefInfo),
  InterfaceMethodRef(InterfaceMethodRefInfo),
  NameAndType(NameAndTypeInfo),
}

#[derive(Clone, Debug)]
pub struct ClassInfo {
  pub tag: u8,
  pub name_index: u16,
}

#[derive(Clone, Debug)]
pub struct FieldRefInfo {
  pub tag: u8,
  pub class_index: u16,
  pub name_and_type_index: u16,
}

#[derive(Clone, Debug)]
pub struct MethodRefInfo {
  pub tag: u8,
  pub class_index: u16,
  pub name_and_type_index: u16,
}

#[derive(Clone, Debug)]
pub struct InterfaceMethodRefInfo {
  pub tag: u8,
  pub class_index: u16,
  pub name_and_type_index: u16,
}

#[derive(Clone, Debug)]
pub struct StringInfo {
  pub tag: u8,
  pub string_index: u16,
}

#[derive(Clone, Debug)]
pub struct IntegerInfo {
  pub tag: u8,
  pub bytes: u32,
}

#[derive(Clone, Debug)]
pub struct FloatInfo {
  pub tag: u8,
  pub bytes: u32,
}

#[derive(Clone, Debug)]
pub struct LongInfo {
  pub tag: u8,
  pub high_bytes: u32,
  pub low_bytes: u32,
}

#[derive(Clone, Debug)]
pub struct DoubleInfo {
  pub tag: u8,
  pub high_bytes: u32,
  pub low_bytes: u32,
}

#[derive(Clone, Debug)]
pub struct NameAndTypeInfo {
  pub tag: u8,
  pub index: u16,
  pub descriptor_index: u16,
}

#[derive(Clone, Debug)]
pub struct Utf8Info {
  pub tag: u8,
  pub length: u16,
  pub bytes: String,
}

#[derive(Clone, Debug)]
pub struct MethodHandleInfo {
  pub tag: u8,
  pub reference_kind: u8,
  pub reference_index: u16,
}
