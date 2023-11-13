// Errors for Statistics files: *_statics.rs

use std::fmt::{Debug, Formatter, Result};

#[derive(PartialEq)]
pub enum StatsError{
    EmptyDataSet,
    InvalidInputValue,
    InconsistentLength
}

impl std::fmt::Display for StatsError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match self {
                StatsError::EmptyDataSet => "Input data array is empty, cannot perform operation.",
                StatsError::InvalidInputValue => "Input contains invalid values (e.g., NaN or Infinity).",
                StatsError::InconsistentLength => "Input data and weight arrays length do not match",
            }
        )
    }

}


impl Debug for StatsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            StatsError::EmptyDataSet => write!(f, "StatsError::EmptyDataSet"),
            StatsError::InvalidInputValue => write!(f, "StatsError::InvalidInputValue"),
            StatsError::InconsistentLength => write!(f, "StatsError::InconsistentLength")
        }
    }
}


impl std::error::Error for StatsError {}

