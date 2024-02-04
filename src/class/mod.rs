pub mod field;
pub mod method;
pub mod pool;

use std::io::Read;

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

    let (fields_count, fields) = self.read_fields()?;

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
  ) -> std::io::Result<Vec<pool::Entry>> {
    let constant_pool_count = constant_pool_count - 1;
    let mut constant_pool_counter = 0;
    let mut constant_pool_entries = Vec::new();
    constant_pool_entries.resize(
      (constant_pool_count + 1) as usize,
      pool::Entry::Integer(pool::IntegerInfo { tag: 0, bytes: 0 }),
    );

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

          let bytes =
            String::from_utf8(buf).expect("To be a valid UTF-8 String.");

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
        other => todo!("{other:x}"),
      };
      constant_pool_entries[constant_pool_counter as usize] = item;
    }

    Ok(constant_pool_entries)
  }

  pub fn read_type_info(&mut self) -> std::io::Result<(u16, u16, u16, u16)> {
    let mut buf = [0u8; 8];
    self.buf.read_exact(&mut buf)?;

    let [flag1, flag2, this1, this2, super1, super2, interface1, interface2] =
      buf;

    let access_flags = u16::from_be_bytes([flag1, flag2]);
    let this_class = u16::from_be_bytes([this1, this2]);
    let super_class = u16::from_be_bytes([super1, super2]);
    let interfaces_count = u16::from_be_bytes([interface1, interface2]);

    Ok((access_flags, this_class, super_class, interfaces_count))
  }

  pub fn read_interfaces(
    &mut self,
    interfaces_count: u16,
  ) -> std::io::Result<Vec<u16>> {
    let mut interfaces = Vec::with_capacity(interfaces_count as usize);

    for _ in 0..interfaces_count {
      let mut buf = [0u8; 2];
      self.buf.read_exact(&mut buf)?;
      let interface = u16::from_be_bytes(buf);
      interfaces.push(interface);
    }

    Ok(interfaces)
  }

  pub fn read_fields(&mut self) -> std::io::Result<(u16, Vec<FieldInfo>)> {
    let mut fields_count_buf = [0u8; 2];
    self.buf.read_exact(&mut fields_count_buf)?;
    let fields_count = u16::from_be_bytes(fields_count_buf);

    let mut fields = Vec::with_capacity(fields_count as usize);

    for _ in 0..fields_count {
      let mut buf = [0u8; 8];
      self.buf.read_exact(&mut buf)?;
      let [flag1, flag2, name1, name2, desc1, desc2, attr1, attr2] = buf;

      let access_flags = u16::from_be_bytes([flag1, flag2]);
      let name_index = u16::from_be_bytes([name1, name2]);
      let descriptor_index = u16::from_be_bytes([desc1, desc2]);
      let attributes_count = u16::from_be_bytes([attr1, attr2]);

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
        let attribute = self.read_field_attribute()?;
        field_info.attributes.push(attribute);
      }

      fields.push(field_info);
    }

    Ok((fields_count, fields))
  }

  pub fn read_field_attribute(
    &mut self,
  ) -> std::io::Result<field::AttributeInfo> {
    let mut buf = [0u8; 6];
    self.buf.read_exact(&mut buf)?;
    let [name1, name2, b1, b2, b3, b4] = buf;

    let attribute_name_index = u16::from_be_bytes([name1, name2]);
    let attribute_length = u32::from_be_bytes([b1, b2, b3, b4]);

    let mut buf = vec![0; attribute_length as usize];
    self.buf.read_exact(&mut buf[..])?;

    let attribute_info = field::AttributeInfo {
      attribute_name_index,
      attribute_length,
      info: buf,
    };

    Ok(attribute_info)
  }

  pub fn read_methods(
    &mut self,
    constant_pool: &Vec<pool::Entry>,
  ) -> std::io::Result<(u16, Vec<MethodInfo>)> {
    let mut methods_count_buf = [0u8; 2];
    self.buf.read_exact(&mut methods_count_buf)?;
    let methods_count = u16::from_be_bytes(methods_count_buf);

    let mut methods = Vec::with_capacity(methods_count as usize);

    for _ in 0..methods_count {
      let mut buf = [0u8; 8];
      self.buf.read_exact(&mut buf)?;

      let [flag1, flag2, name1, name2, desc1, desc2, attr1, attr2] = buf;

      let access_flags = u16::from_be_bytes([flag1, flag2]);
      let name_index = u16::from_be_bytes([name1, name2]);
      println!("name_index: {:?}", constant_pool[name_index as usize]);
      let descriptor_index = u16::from_be_bytes([desc1, desc2]);
      println!(
        "descriptor_index: {:?}",
        constant_pool[descriptor_index as usize]
      );
      let attributes_count = u16::from_be_bytes([attr1, attr2]);
      let attribute_info = Vec::with_capacity(attributes_count as usize);

      let mut method_info = MethodInfo {
        access_flags,
        name_index,
        descriptor_index,
        attributes_count,
        attribute_info,
      };

      // TODO: check if this is right
      for _ in 0..attributes_count {
        let attribute = self.read_method_attribute()?;
        method_info.attribute_info.push(attribute);
      }

      methods.push(method_info);
    }

    Ok((methods_count, methods))
  }

  pub fn read_method_attribute(
    &mut self,
  ) -> std::io::Result<method::AttributeInfo> {
    let mut buf = [0u8; 6];
    self.buf.read_exact(&mut buf)?;
    let [name1, name2, b1, b2, b3, b4] = buf;

    let attribute_name_index = u16::from_be_bytes([name1, name2]);
    let attribute_length = u32::from_be_bytes([b1, b2, b3, b4]);

    let mut buf = vec![0; attribute_length as usize];
    self.buf.read_exact(&mut buf[..])?;

    let attribute_info = method::AttributeInfo {
      attribute_name_index,
      attribute_length,
      info: buf,
    };

    Ok(attribute_info)
  }
}
