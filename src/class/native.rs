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
  }
}
