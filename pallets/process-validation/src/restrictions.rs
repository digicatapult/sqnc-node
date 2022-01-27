use codec::{Decode, Encode};

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Restriction {
  None
  // TODO implement some actual restrictions
}

impl Default for Restriction {
  fn default() -> Self {
      Restriction::None
  }
}

#[allow(dead_code)]
pub fn validate_restriction(restriction: &Restriction) -> bool {
  match *restriction {
    Restriction::None => true
    // TODO implement some actual restrictions
  }
}

