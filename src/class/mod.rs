pub mod field;
pub mod method;
pub mod pool;

use std::{fs::File, io::Read, path::Path};

use self::{
  field::FieldInfo,
  method::{AttributeInfo, MethodInfo},
};

/// Declared public; may be accessed from outside its package.
pub const ACC_PUBLIC: u16 = 0x0001;
/// Declared private; usable only within the defining class.
pub const ACC_PRIVATE: u16 = 0x0002;
/// Declared protected; may be accessed within subclasses.
pub const ACC_PROTECTED: u16 = 0x0004;
/// Declared static.
pub const ACC_STATIC: u16 = 0x0008;
/// Declared final; no subclasses allowed.
pub const ACC_FINAL: u16 = 0x0010;
/// Declared volatile; cannot be cached.
pub const ACC_VOLATILE: u16 = 0x0040;
/// Treat superclass methods specially when invoked by the invokespecial instruction.
pub const ACC_SUPER: u16 = 0x0020;
/// Declared transient; not written or read by a persistent object manager.
pub const ACC_TRANSIENT: u16 = 0x0080;
/// Is an interface, not a class.
pub const ACC_INTERFACE: u16 = 0x0200;
/// Declared abstract; must not be instantiated.
pub const ACC_ABSTRACT: u16 = 0x0400;
/// Declared synthetic; not present in the source code.
pub const ACC_SYNTHETIC: u16 = 0x1000;
/// Declared as an annotation type.
pub const ACC_ANNOTATION: u16 = 0x2000;
/// Declared as an enum type.
pub const ACC_ENUM: u16 = 0x4000;

pub struct Class {
  pub magic: u32,
  pub minor_version: u16,
  pub major_version: u16,
  pub constant_pool_count: u16,
  pub constant_pool: Vec<pool::Entry>,
  pub access_flags: u16,
  pub this_class: u16,
  pub super_class: u16,
  pub interfaces_count: u16,
  pub interfaces: Vec<u16>,
  pub fields_count: u16,
  pub fields: Vec<FieldInfo>,
  pub methods_count: u16,
  pub methods: Vec<MethodInfo>,
  pub attributes_count: u16,
  pub attributes: Vec<AttributeInfo>,
}

pub fn read_bytes(path: &Path) -> Result<Vec<u8>, std::io::Error> {
  let mut file = File::open(path)?;
  let mut buf = Vec::new();
  file.read_to_end(&mut buf)?;
  Ok(buf)
}

pub struct Reader<R: Read> {
  buf: R,
}

impl<R: Read> Reader<R> {
  pub fn new(buf: R) -> Self {
    Self { buf }
  }
}
