mod builder;
mod command;

pub use builder::Builder;
pub use command::Command;

/// The default factory enabled fee amounts, denominated in hundredths of bips.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(usize)]
pub enum FeeAmount {
    Lowest = 100,
    Low = 500,
    Medium = 3000,
    High = 10000,
}

impl FeeAmount {
    /// The default factory tick spacings by fee amount.
    pub fn tick_spacing(&self) -> usize {
        match self {
            Self::Lowest => 1,
            Self::Low => 10,
            Self::Medium => 60,
            Self::High => 200,
        }
    }
}
