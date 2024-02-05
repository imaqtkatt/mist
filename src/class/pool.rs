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
  Utf8Info {
    bytes: String,
  },
  IntegerInfo {
    bytes: u32,
  },
  FloatInfo {
    bytes: u32,
  },
  LongInfo {
    high_bytes: u32,
    low_bytes: u32,
  },
  DoubleInfo {
    high_bytes: u32,
    low_bytes: u32,
  },
  ClassInfo {
    name_index: u16,
  },
  StringInfo {
    string_index: u16,
  },
  FieldRefInfo {
    class_index: u16,
    name_and_type_index: u16,
  },
  MethodRefInfo {
    class_index: u16,
    name_and_type_index: u16,
  },
  InterfaceMethodRefInfo {
    class_index: u16,
    name_and_type_index: u16,
  },
  NameAndTypeInfo {
    index: u16,
    descriptor_index: u16,
  },
}
