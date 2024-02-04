pub mod field;
pub mod method;
pub mod pool;

use std::{fs::File, io::Read, path::Path};

use self::{
  field::FieldInfo,
  method::{AttributeInfo, MethodInfo},
  pool::MethodRefInfo,
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

impl<R: Read> Reader<R> {
  pub fn read_header(&mut self) -> std::io::Result<(u32, u16, u16, u16)> {
    let mut magic_buf = [0u8; 4];
    self.buf.read_exact(&mut magic_buf)?;

    if magic_buf != [0xCA, 0xFE, 0xBA, 0xBE] {
      panic!("Is not a .class file.");
    }
    let magic = u32::from_be_bytes(magic_buf);

    let mut minor_version_buf = [0u8; 2];
    self.buf.read_exact(&mut minor_version_buf)?;
    let minor_verson = u16::from_be_bytes(minor_version_buf);

    let mut major_version_buf = [0u8; 2];
    self.buf.read_exact(&mut major_version_buf)?;
    let major_version = u16::from_be_bytes(major_version_buf);

    let mut constant_pool_count_buf = [0u8; 2];
    self.buf.read_exact(&mut constant_pool_count_buf)?;
    let constant_pool_count = u16::from_be_bytes(constant_pool_count_buf);

    Ok((magic, minor_verson, major_version, constant_pool_count))
  }

  pub fn read_constant_pool(
    &mut self,
    constant_pool_count: u16,
  ) -> std::io::Result<()> {
    let constant_pool_count = constant_pool_count - 1;
    let mut constant_pool_counter = 0;
    let mut constant_pool_entries =
      Vec::with_capacity(constant_pool_count as usize);

    while constant_pool_counter < constant_pool_count {
      constant_pool_counter += 1;
      let mut tag_buf = [0u8; 1];
      self.buf.read_exact(&mut tag_buf)?;
      let tag = u8::from_be_bytes(tag_buf);

      let item = match tag {
        pool::UTF_8 => {
          let mut len_buf = [0u8; 2];
          self.buf.read_exact(&mut len_buf)?;
          let len = u16::from_be_bytes(len_buf);

          let mut buf = vec![0u8; len as usize];
          self.buf.read_exact(&mut buf[..])?;

          let bytes = String::from_utf8_lossy(&buf);
          let bytes = String::from(bytes);

          pool::Entry::Utf8(pool::Utf8Info {
            tag,
            length: len,
            bytes,
          })
        }
        pool::CLASS => {
          let mut buf = [0u8; 2];
          self.buf.read_exact(&mut buf)?;

          let name_index = u16::from_be_bytes(buf);

          pool::Entry::Class(pool::ClassInfo { tag, name_index })
        }
        pool::NAME_AND_TYPE => {
          let mut buf = [0u8; 4];
          self.buf.read_exact(&mut buf)?;

          let [a, b, c, d] = buf;

          let index = u16::from_be_bytes([a, b]);
          let descriptor_index = u16::from_be_bytes([c, d]);

          pool::Entry::NameAndType(pool::NameAndTypeInfo {
            tag,
            index,
            descriptor_index,
          })
        }
        pool::METHOD_REF => {
          let mut buf = [0u8; 4];
          self.buf.read_exact(&mut buf)?;

          let [a, b, c, d] = buf;

          let class_index = u16::from_be_bytes([a, b]);
          let name_and_type_index = u16::from_be_bytes([c, d]);

          pool::Entry::MethodRef(pool::MethodRefInfo {
            tag,
            class_index,
            name_and_type_index,
          })
        }
        other => panic!("{other:x}"),
      };
      constant_pool_entries.push(item);
    }

    println!("{constant_pool_entries:?}");

    Ok(())
  }
}
