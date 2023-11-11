// Errors for Statistics files: *_statics.rs

use std::fmt::{Debug, Formatter};

#[derive(PartialEq)]
pub enum StatsError{
    EmptyDataSet,
    InvalidInputValue
}

impl std::fmt::Display for StatsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StatsError::EmptyDataSet => "Input data array is empty, cannot perform operation.",
                StatsError::InvalidInputValue => "Input contains invalid values (e.g., NaN or Infinity).",
            }
        )
    }

}


impl std::fmt::Debug for StatsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatsError::EmptyDataSet => write!(f, "StatsError::EmptyDataSet"),
            StatsError::InvalidInputValue => write!(f, "StatsError::InvalidInputValue"),
        }
    }
}


impl std::error::Error for StatsError {}

