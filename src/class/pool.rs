pub const CLASS: u16 = 7;
pub const FIELD_REF: u16 = 9;
pub const METHOD_REF: u16 = 10;
pub const INTERFACE_METHOD_REF: u16 = 11;
pub const STRING: u16 = 8;
pub const INTEGER: u16 = 3;
pub const FLOAT: u16 = 4;
pub const LONG: u16 = 5;
pub const DOUBLE: u16 = 6;
pub const NAME_AND_TYPE: u16 = 12;
pub const UTF_8: u16 = 1;
pub const METHOD_HANDLE: u16 = 15;
pub const METHOD_TYPE: u16 = 16;
pub const INVOKE_DYNAMIC: u16 = 18;

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
