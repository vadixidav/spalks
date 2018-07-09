use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use Trilean::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Trilean {
    Known(bool),
    Unknown,
}

impl Trilean {
    /// If the value is known, returns that, otherwise evaluates `f`.
    pub fn known_or_else<F: FnOnce() -> bool>(self, f: F) -> bool {
        match self {
            Known(b) => b,
            Unknown => f(),
        }
    }

    /// If the value is known, returns that, otherwise `or`.
    pub fn known_or(self, or: bool) -> bool {
        self.known_or_else(|| or)
    }

    /// If the value is known, returns that, otherwise false.
    pub fn known(self) -> bool {
        self.known_or(false)
    }
}

impl BitAnd for Trilean {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        match self {
            Known(true) => rhs,
            Known(false) => Known(false),
            Unknown => match rhs {
                Known(true) => Unknown,
                Known(false) => Known(false),
                Unknown => Unknown,
            },
        }
    }
}

impl Not for Trilean {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Known(b) => Known(!b),
            Unknown => Unknown,
        }
    }
}

impl BitAndAssign for Trilean {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for Trilean {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        match self {
            Known(true) => Known(true),
            Known(false) => rhs,
            Unknown => match rhs {
                Known(true) => Known(true),
                Known(false) => Unknown,
                Unknown => Unknown,
            },
        }
    }
}

impl BitOrAssign for Trilean {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for Trilean {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Known(a), Known(b)) => Known(a ^ b),
            _ => Unknown,
        }
    }
}

impl BitXorAssign for Trilean {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}
