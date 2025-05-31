use std::{str::FromStr, fmt::{Display, Formatter, Result}};


impl From<&crate::Uuid> for uuid::Uuid {
  fn from(uuid: &crate::Uuid) -> uuid::Uuid {
    uuid::Uuid::from_str(String::from_utf8_lossy(&uuid.value).as_ref()).unwrap()
  }
}

impl From<&uuid::Uuid> for crate::Uuid {
  fn from(uuid: &uuid::Uuid) -> crate::Uuid {
    crate::Uuid {
      value: uuid.to_string().into_bytes(),
      version: 0,
    }
  }
}

impl Display for crate::Uuid {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", String::from_utf8_lossy(&self.value))
  }
}