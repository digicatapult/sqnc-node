// This file contains the different types of restrictions that can be evaluated during
// a call to `validate_process`

use codec::{Decode, Encode};

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Restriction {
    None, // TODO: implement some actual restrictions
}

impl Default for Restriction {
    fn default() -> Self {
        Restriction::None
    }
}

// TODO: update args appropriately and implement restriction functionality
#[allow(dead_code)]
pub fn validate_restriction(restriction: &Restriction) -> bool {
    match *restriction {
        Restriction::None => true, // TODO implement some actual restrictions
    }
}
