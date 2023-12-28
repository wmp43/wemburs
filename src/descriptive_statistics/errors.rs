// Errors for Statistics files: *_statics.rs

use std::fmt::{Debug, Formatter, Result};
use pyo3::PyErr;
use pyo3::exceptions::PyValueError;

#[derive(Debug, PartialEq)]
pub enum StatsError{
    EmptyDataSet,
    InvalidInputValue,
    InconsistentLength,
    MinMaxError,
    ZeroVariance,
    Conversion
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
                StatsError::MinMaxError => "Cannot compute min or max: the dataset is empty or contains invalid values.",
                StatsError::ZeroVariance => "Cannot compute: one or both variables have zero variance.",
                StatsError::Conversion => "Cannot convert input array to numeric type."
            }
        )
    }
}


impl From<StatsError> for PyErr {
    fn from(err: StatsError) -> PyErr {
        match err {
            StatsError::EmptyDataSet => PyValueError::new_err(err.to_string()),
            StatsError::InvalidInputValue => PyValueError::new_err(err.to_string()),
            StatsError::InconsistentLength => PyValueError::new_err(err.to_string()),
            StatsError::MinMaxError => PyValueError::new_err(err.to_string()),
            StatsError::ZeroVariance => PyValueError::new_err(err.to_string()),
            StatsError::Conversion => PyValueError::new_err(err.to_string()),

        }
    }
}


// impl Debug for StatsError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         match self {
//             StatsError::EmptyDataSet => write!(f, "StatsError::EmptyDataSet"),
//             StatsError::InvalidInputValue => write!(f, "StatsError::InvalidInputValue"),
//             StatsError::InconsistentLength => write!(f, "StatsError::InconsistentLength")
//         }
//     }
// }
