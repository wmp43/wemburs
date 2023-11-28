// functions:
// t_test -- compare averages to see if they are different
// chi_squared_test -- Check if there is a relationship between two categorical variables or if differences in frequency are due to chance.
// anova -- Determine if there are any statistically significant differences between the means of three or more unrelated groups.
// confidence intervals -- Provide a range that likely contains the true value of an unknown parameter.
// effect size -- Quantify the strength of the relationship between two variables.
// kolmogorov-smirnov tests -- compare distributions

/// imports
pub mod errors;
pub use crate::inferential_statistics::errors::*;
pub use crate::descriptive_statistics::{mean_rs, median_rs, variance_rs, percentile_rs};

use pyo3::types::PyDict;
use statrs::statistics::Data;
use pyo3::prelude::*;
use numpy::{PyArray1};

/// Data Validation Macro

/// Rust Native Computations

/// Pyfunctions

pub fn confidence_interval(x: &PyArray1<f64>, ci: f64) -> PyResult<f64> {
    // Takes array, ci.
    // Maybe returns tuple with lower bound and upper bound
    let read_x = x.readonly();
    let data = read_x.as_slice()?;
    // Validate input macro here!!
    // Rust native
    let (var, sqrt, mean, n)  = (variance_rs(&data), var.sqrt(), mean_rs(data), data.len());
    // need pyDict, Hashmap!!! equivalent for CI to z score mapping
    // x_bar +- Z*(std/n.sqrt())

}

