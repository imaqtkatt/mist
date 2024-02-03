use std::cmp::Ordering;

/// A JVM value.
#[derive(Clone, Copy, Debug)]
pub enum MistValue {
  /// True or False.
  Boolean(bool),
  /// From -128 to 127.
  Byte(i8),
  /// From -32768 to 32767.
  Short(i16),
  /// From -2147483648 to 2147483647.
  Integer(i32),
  /// From -9223372036854775808 to 9223372036854775807.
  Float(f32),
  Long(i64),
  Double(f64),
  /// From 0 to 65535.
  // TODO: use u16?
  Char(char),
  ObjectReference(usize),
}

impl MistValue {
  pub fn short_name(&self) -> char {
    match self {
      MistValue::Boolean(..) => 'Z',
      MistValue::Byte(..) => 'B',
      MistValue::Short(..) => 'S',
      MistValue::Integer(..) => 'I',
      MistValue::Float(..) => 'F',
      MistValue::Long(..) => 'J',
      MistValue::Double(..) => 'D',
      MistValue::Char(..) => 'C',
      MistValue::ObjectReference(..) => 'A',
    }
  }
}

macro_rules! mist_value_into {
  ($type:ty, $variant:ident) => {
    impl Into<$type> for MistValue {
      fn into(self) -> $type {
        if let Self::$variant(inner) = self {
          inner
        } else {
          panic!("Could not convert {:?} to {}", self, stringify!($type))
        }
      }
    }
  };
}

impl From<char> for MistValue {
  fn from(value: char) -> Self {
    match value {
      'Z' => Self::Boolean(bool::default()),
      'B' => Self::Byte(i8::default()),
      'S' => Self::Short(i16::default()),
      'I' => Self::Integer(i32::default()),
      'F' => Self::Float(f32::default()),
      'J' => Self::Long(i64::default()),
      'D' => Self::Double(f64::default()),
      'C' => Self::Char('\0'),
      'A' => Self::ObjectReference(usize::default()),
      chr => panic!("Unknown value '{chr}'"),
    }
  }
}

mist_value_into!(bool, Boolean);
mist_value_into!(i8, Byte);
mist_value_into!(i16, Short);
mist_value_into!(i32, Integer);
mist_value_into!(f32, Float);
mist_value_into!(i64, Long);
mist_value_into!(f64, Double);
mist_value_into!(char, Char);
mist_value_into!(usize, ObjectReference);

impl Default for MistValue {
  fn default() -> Self {
    Self::Integer(i32::default())
  }
}

impl TryFrom<Option<Ordering>> for MistValue {
  type Error = ();

  fn try_from(value: Option<Ordering>) -> Result<Self, Self::Error> {
    value
      .and_then(|ordering| Some(Self::from(ordering)))
      .ok_or(())
  }
}

impl From<Ordering> for MistValue {
  fn from(value: Ordering) -> Self {
    match value {
      Ordering::Less => Self::Integer(-1),
      Ordering::Equal => Self::Integer(0),
      Ordering::Greater => Self::Integer(1),
    }
  }
}
