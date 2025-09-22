#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;
impl FromStr for Antigen {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "AB" => Ok(Self::AB),
            "B" => Ok(Self::B),
            "O" => Ok(Self::O),
            _ => Err(()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Positive),
            "-" => Ok(Self::Negative),
            _ => Err(()),
        }
    }
}

impl Ord for BloodType {
}

impl FromStr for BloodType {

}

use std::fmt::{self, Debug};

impl Debug for BloodType {
}

impl BloodType {
	pub fn can_receive_from(&self, other: &Self) -> bool {
	}

	pub fn donors(&self) -> Vec<Self> {
	}

	pub fn recipients(&self) -> Vec<BloodType> {
}