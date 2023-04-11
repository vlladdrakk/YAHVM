use std::fmt::{Debug, Formatter, Display, self};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Var {
  Integer(i8),
  Float(f32),
}

impl Debug for Var {
  fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
    match self {
      Var::Integer(i) => write!(formatter, "{}", i),
      Var::Float(f) => write!(formatter, "{}", f),
    }
  }
}

impl Display for Var {
  fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
    match self {
      Var::Integer(i) => write!(formatter, "{}", i),
      Var::Float(f) => write!(formatter, "{}", f),
    }
  }
}