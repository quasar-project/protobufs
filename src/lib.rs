mod pb {
  tonic::include_proto!("quasar.pb");

  pub mod relay {
    pub mod datagrams {
      pub mod nav {
        tonic::include_proto!("quasar.pb.relay.datagrams.nav");
      }
    }

    pub mod services {
      pub mod nav {
        tonic::include_proto!("quasar.pb.relay.services.nav");
      }
    }
  }
}

pub use pb::*;
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
