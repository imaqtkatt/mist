pub mod java {
  pub mod lang {
    pub mod system {
      use crate::{local::Local, value::MistValue};
      use std::time::SystemTime;

      pub fn current_time_millis(_: &Local) -> Option<MistValue> {
        SystemTime::now()
          .duration_since(SystemTime::UNIX_EPOCH)
          .map(|duration| MistValue::Long(duration.as_millis() as i64))
          .ok()
      }
    }

    pub mod math {
      use crate::{local::Local, value::MistValue};

      pub fn sqrt(local: &Local) -> Option<MistValue> {
        let MistValue::Double(double) = local.load(0) else {
          panic!();
        };
        Some(MistValue::Double(double.sqrt()))
      }
    }
  }
}
