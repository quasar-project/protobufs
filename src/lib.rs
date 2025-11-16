include!(concat!(env!("OUT_DIR"), "/_includes.rs"));

pub use quasar::pb::*;

use std::str::FromStr;

impl From<Uuid> for uuid::Uuid {
  fn from(uuid: Uuid) -> uuid::Uuid {
    uuid::Uuid::from_str(String::from_utf8_lossy(&uuid.value).as_ref()).unwrap()
  }
}

mod types;
pub mod util;

#[allow(unused_imports)]
pub use self::types::*;
