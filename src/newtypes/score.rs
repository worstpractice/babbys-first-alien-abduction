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

#[derive(PartialEq)]
pub struct Score(pub usize);

impl Add for Score {
  type Output = usize;

  fn add(self, rhs: Self) -> Self::Output {
    self.0 + rhs.0
  }
}

impl AddAssign for Score {
  fn add_assign(&mut self, rhs: Self) {
    self.0 += rhs.0;
  }
}

impl Default for Score {
  fn default() -> Self {
    Self(0)
  }
}

impl Display for Score {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.0)
  }
}

impl Sub for Score {
  type Output = usize;

  fn sub(self, rhs: Self) -> Self::Output {
    self.0 - rhs.0
  }
}

impl SubAssign for Score {
  fn sub_assign(&mut self, rhs: Self) {
    self.0 -= rhs.0;
  }
}
