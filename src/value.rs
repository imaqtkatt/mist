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

macro_rules! from_mist_value {
  ($type:ty, $variant:ident) => {
    impl From<MistValue> for $type {
      fn from(value: MistValue) -> Self {
        if let MistValue::$variant(inner) = value {
          inner
        } else {
          panic!("Could not convert {:?} to {}", value, stringify!($type))
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

from_mist_value!(bool, Boolean);
from_mist_value!(i8, Byte);
from_mist_value!(i16, Short);
from_mist_value!(i32, Integer);
from_mist_value!(f32, Float);
from_mist_value!(i64, Long);
from_mist_value!(f64, Double);
from_mist_value!(char, Char);
from_mist_value!(usize, ObjectReference);

impl Default for MistValue {
  fn default() -> Self {
    Self::Integer(i32::default())
  }
}

impl TryFrom<Option<Ordering>> for MistValue {
  type Error = ();

  fn try_from(value: Option<Ordering>) -> Result<Self, Self::Error> {
    value.map(Self::from).ok_or(())
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
