pub mod attribute_info;
pub mod field;
pub mod method;
pub mod pool;

use std::io::Read;

use crate::class::pool::Entry;

use self::{
  attribute_info::{AttributeInfo, Code},
  field::FieldInfo,
  method::MethodInfo,
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

#[derive(Debug)]
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

impl Class {
  pub fn class_name(&self) -> Option<&Entry> {
    let Entry::ClassInfo { name_index, .. } =
      &self.constant_pool[self.this_class as usize]
    else {
      unreachable!()
    };
    if let utf8info @ Entry::Utf8Info { .. } =
      &self.constant_pool[*name_index as usize]
    {
      Some(utf8info)
    } else {
      None
    }
  }

  pub fn find_method(&self, method_name: &str) -> Option<&MethodInfo> {
    for method in self.methods.iter() {
      let cp_entry = &self.constant_pool[method.name_index as usize];
      if let Entry::Utf8Info { bytes, .. } = cp_entry {
        if bytes == method_name {
          return Some(method);
        }
      }
    }

    None
  }
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
  pub fn read_class(&mut self) -> std::io::Result<Class> {
    let (magic, minor_version, major_version, constant_pool_count) =
      self.read_header()?;

    let constant_pool = self.read_constant_pool(constant_pool_count)?;

    let (access_flags, this_class, super_class, interfaces_count) =
      self.read_type_info()?;

    let interfaces = self.read_interfaces(interfaces_count)?;

    let (fields_count, fields) = self.read_fields(&constant_pool)?;

    let (methods_count, methods) = self.read_methods(&constant_pool)?;

    // TODO: read attributes
    let (attributes_count, attributes) = (0, Vec::new());

    let class = Class {
      magic,
      minor_version,
      major_version,
      constant_pool_count,
      constant_pool,
      access_flags,
      this_class,
      super_class,
      interfaces_count,
      interfaces,
      fields_count,
      fields,
      methods_count,
      methods,
      attributes_count,
      attributes,
    };

    Ok(class)
  }

  pub fn read_header(&mut self) -> std::io::Result<(u32, u16, u16, u16)> {
    let magic = self.buf.read_u32()?;

    if magic != 0xCAFEBABE {
      panic!("Is not a .class file.");
    }

    let minor_version = self.buf.read_u16()?;
    let major_version = self.buf.read_u16()?;
    let constant_pool_count = self.buf.read_u16()?;

    Ok((magic, minor_version, major_version, constant_pool_count))
  }

  pub fn read_constant_pool(
    &mut self,
    constant_pool_count: u16,
  ) -> std::io::Result<Vec<pool::Entry>> {
    let constant_pool_count = constant_pool_count - 1;
    let mut constant_pool_counter = 0;
    let mut constant_pool_entries = Vec::new();
    constant_pool_entries.resize(
      (constant_pool_count + 1) as usize,
      pool::Entry::IntegerInfo { bytes: 0 },
    );

    while constant_pool_counter < constant_pool_count {
      constant_pool_counter += 1;

      let tag = self.buf.read_u8()?;
      let item = match tag {
        pool::UTF_8 => {
          let length = self.buf.read_u16()?;

          let mut buf = vec![0u8; length as usize];
          self.buf.read_exact(&mut buf[..])?;
          let bytes =
            String::from_utf8(buf).expect("To be a valid UTF-8 String.");

          pool::Entry::Utf8Info { bytes }
        }
        pool::CLASS => {
          let name_index = self.buf.read_u16()?;
          pool::Entry::ClassInfo { name_index }
        }
        pool::NAME_AND_TYPE => {
          let index = self.buf.read_u16()?;
          let descriptor_index = self.buf.read_u16()?;

          pool::Entry::NameAndTypeInfo {
            index,
            descriptor_index,
          }
        }
        pool::METHOD_REF => {
          let class_index = self.buf.read_u16()?;
          let name_and_type_index = self.buf.read_u16()?;

          pool::Entry::MethodRefInfo {
            class_index,
            name_and_type_index,
          }
        }
        other => todo!("{other:x}"),
      };
      constant_pool_entries[constant_pool_counter as usize] = item;
    }

    Ok(constant_pool_entries)
  }

  pub fn read_type_info(&mut self) -> std::io::Result<(u16, u16, u16, u16)> {
    self.buf.read_4_u16()
  }

  pub fn read_interfaces(
    &mut self,
    interfaces_count: u16,
  ) -> std::io::Result<Vec<u16>> {
    let mut interfaces = Vec::with_capacity(interfaces_count as usize);

    for _ in 0..interfaces_count {
      let interface = self.buf.read_u16()?;
      interfaces.push(interface);
    }

    Ok(interfaces)
  }

  pub fn read_fields(
    &mut self,
    constant_pool: &[pool::Entry],
  ) -> std::io::Result<(u16, Vec<FieldInfo>)> {
    let fields_count = self.buf.read_u16()?;
    let mut fields = Vec::with_capacity(fields_count as usize);

    for _ in 0..fields_count {
      let (access_flags, name_index, descriptor_index, attributes_count) =
        self.buf.read_4_u16()?;

      let attributes = Vec::with_capacity(attributes_count as usize);

      let mut field_info = FieldInfo {
        access_flags,
        name_index,
        descriptor_index,
        attributes_count,
        attributes,
      };

      // TODO: check if this is right
      for _ in 0..attributes_count {
        let attribute = self.read_field_attribute(constant_pool)?;
        field_info.attributes.push(attribute);
      }

      fields.push(field_info);
    }

    Ok((fields_count, fields))
  }

  pub fn read_field_attribute(
    &mut self,
    constant_pool: &[pool::Entry],
  ) -> std::io::Result<AttributeInfo> {
    // let attribute_name_index = self.buf.read_u16()?;
    // let attribute_length = self.buf.read_u32()?;

    // // println!("{:?}", constant_pool[attribute_name_index as usize]);

    // let mut buf = vec![0; attribute_length as usize];
    // self.buf.read_exact(&mut buf[..])?;

    // let attribute_info = AttributeInfo {
    //   attribute_name_index,
    //   attribute_length,
    //   info: attribute_info::Info::Bytes(buf),
    // };

    // Ok(attribute_info)
    self.read_method_attribute(constant_pool)
  }

  pub fn read_methods(
    &mut self,
    constant_pool: &[pool::Entry],
  ) -> std::io::Result<(u16, Vec<MethodInfo>)> {
    let methods_count = self.buf.read_u16()?;

    let mut methods = Vec::with_capacity(methods_count as usize);

    for _ in 0..methods_count {
      let access_flags = self.buf.read_u16()?;
      let name_index = self.buf.read_u16()?;
      // println!("name_index: {:?}", constant_pool[name_index as usize]);
      let descriptor_index = self.buf.read_u16()?;
      // println!(
      //   "descriptor_index: {:?}",
      //   constant_pool[descriptor_index as usize]
      // );
      let attributes_count = self.buf.read_u16()?;
      let mut attributes = Vec::with_capacity(attributes_count as usize);

      // TODO: check if this is right
      for _ in 0..attributes_count {
        let attribute = self.read_method_attribute(constant_pool)?;
        attributes.push(attribute);
      }

      let method_info = MethodInfo {
        access_flags,
        name_index,
        descriptor_index,
        attributes_count,
        attributes,
      };

      methods.push(method_info);
    }

    Ok((methods_count, methods))
  }

  pub fn read_method_attribute(
    &mut self,
    constant_pool: &[pool::Entry],
  ) -> std::io::Result<AttributeInfo> {
    let attribute_name_index = self.buf.read_u16()?;
    let attribute_length = self.buf.read_u32()?;

    let cp_entry = &constant_pool[attribute_name_index as usize];

    // TODO: refactor this messy code.
    if let Entry::Utf8Info { bytes } = cp_entry {
      match bytes.as_ref() {
        "LineNumberTable" => {
          let line_number_table_length = self.buf.read_u16()?;

          let mut line_number_table =
            Vec::with_capacity(line_number_table_length as usize);
          for _ in 0..line_number_table_length {
            line_number_table.push(attribute_info::LineNumberTableInfo {
              start_pc: self.buf.read_u16()?,
              line_number: self.buf.read_u16()?,
            });
          }

          let line_number_table = attribute_info::LineNumberTable {
            attribute_name_index,
            attribute_length,
            line_number_table,
          };

          Ok(AttributeInfo {
            attribute_name_index,
            // attribute_length,
            info: attribute_info::Info::LineNumberTable(line_number_table),
          })
        }
        "Code" => {
          let max_stack = self.buf.read_u16()?;
          let max_local = self.buf.read_u16()?;
          let code_length = self.buf.read_u32()?;

          let mut code = vec![0; code_length as usize];
          self.buf.read_exact(&mut code[..])?;

          let exception_table_length = self.buf.read_u16()?;

          let mut exception_table = Vec::new();
          for _ in 0..exception_table_length {
            let (start_pc, end_pc, handler_pc, catch_type) =
              self.buf.read_4_u16()?;
            exception_table.push(attribute_info::ExceptionTableInfo {
              start_pc,
              end_pc,
              handler_pc,
              catch_type,
            });
          }

          let attributes_count = self.buf.read_u16()?;

          let mut attributes = Vec::with_capacity(attributes_count as usize);
          for _ in 0..attributes_count {
            let attribute = self.read_method_attribute(constant_pool)?;
            attributes.push(attribute);
          }

          let code = Code {
            max_stack,
            max_local,
            code_length,
            code,
            // exception_table_length,
            exception_table,
            // attributes_count,
            attributes,
          };

          Ok(AttributeInfo {
            attribute_name_index,
            // attribute_length,
            info: attribute_info::Info::Code(code),
          })
        }
        other => unimplemented!("{other:?}"),
      }
    } else {
      panic!("Attribute name index '{attribute_name_index}' needs to point to a Utf-8 entry.");
    }
  }
}

trait ClassReaderUtils {
  fn read_u8(&mut self) -> std::io::Result<u8>;

  fn read_u16(&mut self) -> std::io::Result<u16>;

  fn read_u32(&mut self) -> std::io::Result<u32>;

  fn read_4_u16(&mut self) -> std::io::Result<(u16, u16, u16, u16)>;
}

impl<R> ClassReaderUtils for R
where
  R: Read,
{
  fn read_u8(&mut self) -> std::io::Result<u8> {
    let mut buf = [0u8; 1];
    self.read_exact(&mut buf)?;
    Ok(u8::from_be_bytes(buf))
  }

  fn read_u16(&mut self) -> std::io::Result<u16> {
    let mut buf = [0u8; 2];
    self.read_exact(&mut buf)?;
    Ok(u16::from_be_bytes(buf))
  }

  fn read_u32(&mut self) -> std::io::Result<u32> {
    let mut buf = [0u8; 4];
    self.read_exact(&mut buf)?;
    Ok(u32::from_be_bytes(buf))
  }

  fn read_4_u16(&mut self) -> std::io::Result<(u16, u16, u16, u16)> {
    let mut buf = [0u8; 8];
    self.read_exact(&mut buf)?;
    let [a, b, c, d, e, f, g, h] = buf;
    Ok((
      u16::from_be_bytes([a, b]),
      u16::from_be_bytes([c, d]),
      u16::from_be_bytes([e, f]),
      u16::from_be_bytes([g, h]),
    ))
  }
}
