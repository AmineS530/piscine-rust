#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
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
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.antigen, &self.rh_factor).cmp(&(&other.antigen, &other.rh_factor))
    }
}

impl FromStr for BloodType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (antigen_str, rh_str) = s.split_at(s.len() - 1);
        Ok(BloodType {
            antigen: antigen_str.parse()?,
            rh_factor: rh_str.parse()?,
        })
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rh = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{:?}{}", self.antigen, rh)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        let antigen_ok = matches!(
            (self.antigen.clone(), other.antigen.clone()),
            (Antigen::A, Antigen::A | Antigen::O)
                | (Antigen::B, Antigen::B | Antigen::O)
                | (
                    Antigen::AB,
                    Antigen::A | Antigen::B | Antigen::AB | Antigen::O
                )
                | (Antigen::O, Antigen::O)
        );

        let rh_ok = self.rh_factor == RhFactor::Positive || other.rh_factor == RhFactor::Negative;

        antigen_ok && rh_ok
    }
    pub fn donors(&self) -> Vec<Self> {
        blood_types()
            .into_iter()
            .filter(|bt| self.can_receive_from(bt))
            .collect()
    }
    pub fn recipients(&self) -> Vec<BloodType> {
        blood_types()
            .into_iter()
            .filter(|bt| bt.can_receive_from(self))
            .collect()
    }
}
pub fn blood_types() -> Vec<BloodType> {
    vec![
        BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Positive,
        },
        BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        },
        BloodType {
            antigen: Antigen::B,
            rh_factor: RhFactor::Positive,
        },
        BloodType {
            antigen: Antigen::B,
            rh_factor: RhFactor::Negative,
        },
        BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        },
        BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        },
        BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Positive,
        },
        BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Negative,
        },
    ]
}
