// Errors for Statistics files: *_statics.rs
// This needs to be completly remand for linear models

use std::fmt::{Debug, Formatter, Result};
use pyo3::PyErr;
use pyo3::exceptions::PyValueError;

#[derive(Debug, PartialEq)]
pub enum LinearModelError{
    EmptyDataSet,
    InvalidInputValue,
    InconsistentLength,
    InvalidFeatures,
    ZeroVariance
}

impl std::fmt::Display for StatsError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match self {
                LinearModelError::EmptyDataSet => "Input data array is empty, cannot perform operation.",
                LinearModelError::InvalidInputValue => "Input contains invalid values (e.g., NaN or Infinity).",
                LinearModelError::InconsistentLength => "Input arrays length do not match",
                LinearModelError::InvalidFeatures => "Input matrix has too many or too few features",
                LinearModelError::ZeroVariance => "Cannot compute: one or both variables have zero variance."
            }
        )
    }
}


impl From<LinearModelError> for PyErr {
    fn from(err: LinearModelError) -> PyErr {
        match err {
            LinearModelError::EmptyDataSet => PyValueError::new_err(err.to_string()),
            LinearModelError::InvalidInputValue => PyValueError::new_err(err.to_string()),
            LinearModelError::InconsistentLength => PyValueError::new_err(err.to_string()),
            LinearModelError::InvalidFeatures => PyValueError::new_err(err.to_string()),
            LinearModelError::ZeroVariance => PyValueError::new_err(err.to_string()),
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
