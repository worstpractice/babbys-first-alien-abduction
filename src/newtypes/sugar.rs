use std::{
  fmt::{
    Display,
    Formatter,
    Result,
  },
  ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
  },
};

#[derive(PartialEq, PartialOrd)]
pub struct Sugar(pub isize);

impl Add for Sugar {
  type Output = isize;

  fn add(self, rhs: Self) -> Self::Output {
    self.0 + rhs.0
  }
}

impl AddAssign for Sugar {
  fn add_assign(&mut self, rhs: Self) {
    self.0 += rhs.0;
  }
}

impl Default for Sugar {
  fn default() -> Self {
    Self(0)
  }
}

impl Display for Sugar {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.0)
  }
}

impl Sub for Sugar {
  type Output = isize;

  fn sub(self, rhs: Self) -> Self::Output {
    self.0 - rhs.0
  }
}

impl SubAssign for Sugar {
  fn sub_assign(&mut self, rhs: Self) {
    self.0 -= rhs.0;
  }
}
