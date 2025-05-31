use std::fmt::{Display, Formatter, Result};

impl Display for crate::EulerAngles {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "({0:.p$}, {1:.p$}, {2:.p$})",
      self.roll,
      self.pitch,
      self.yaw,
      p = f.precision().unwrap_or(1)
    )
  }
}

impl From<mint::EulerAngles<f32, mint::ExtraXYZ>> for crate::EulerAngles {
  fn from(v: mint::EulerAngles<f32, mint::ExtraXYZ>) -> Self {
    crate::EulerAngles {
      roll: v.a,
      pitch: v.b,
      yaw: v.c,
    }
  }
}

impl From<crate::EulerAngles> for mint::EulerAngles<f32, mint::ExtraXYZ> {
  fn from(v: crate::EulerAngles) -> Self {
    mint::EulerAngles { a: v.roll, b: v.pitch, c: v.yaw, marker: Default::default() }
  }
}

impl mint::IntoMint for crate::EulerAngles {
  type MintType = mint::EulerAngles<f32, mint::ExtraXYZ>;
}