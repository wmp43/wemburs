// functions:
// t_test -- compare averages to see if they are different
// chi_squared_test -- Check if there is a relationship between two categorical variables or if differences in frequency are due to chance.
// anova -- Determine if there are any statistically significant differences between the means of three or more unrelated groups.
// confidence intervals -- Provide a range that likely contains the true value of an unknown parameter.
// effect size -- Quantify the strength of the relationship between two variables.
// kolmogorov-smirnov tests -- compare distributions

use statrs::distribution::ContinuousCDF;
/// imports
pub mod errors;
pub use crate::inferential_statistics::errors::*;
pub use crate::{validate_statistical_input,
                descriptive_statistics::{mean_rs, median_rs, variance_rs,
                                                               percentile_rs}};
pub use crate::utils::{from_pyarray1};

use statrs::distribution::{Normal};
use pyo3::types::PyDict;
use pyo3::prelude::*;
use numpy::{PyArray1};


/// Data Validation Macro
// Also Imported from descriptive statistics.
// May need to develop a new one
/// Rust Native Computations
// Simply imported from descriptive Statistics
/// Pyfunctions
///
#[pyfunction]
pub fn confidence_interval(x: &PyArray1<f64>, ci: f64) -> PyResult<(f64, f64)> {
    // Takes array, ci.
    // Maybe returns tuple with lower bound and upper bound
    let x_data = match from_pyarray1(x) {
        Ok(data) => data,
        Err(e) => return Err(StatsError::Conversion.into()),
    };
    validate_statistical_input!(basic, x_data);
    if ci < 0.0 || ci > 1.0 {
        return Err(PyErr::from(StatsError::InvalidInputValue));
    };

    let (var, mean, n)  = (variance_rs(&x_data), mean_rs(x_data), x_data.len() as f64);
    let std_error = (var / n).sqrt();

    let alpha = 1.0 - ci;
    let z = Normal::new(0.0, 1.0)

        .map_err(|_| (PyErr::from(StatsError::UnderlyingError)))?
        .inverse_cdf(1.0 - alpha / 2.0);

    // let lower_bound = x_bar - Z*(std/n.sqrt())
    // let upper_bound = x_bar + Z*(std.n.sqrt())
    let lower_bound = mean - z * std_error;
    let upper_bound = mean + z * std_error;

    Ok((lower_bound, upper_bound))
}

#[pyfunction]
pub fn kolmogorov_smirnov_test(x: &PyArray1<f64>, y: &PyArray1<f64>) -> PyResult<f64> {
    let x_data = match from_pyarray1(x) {
        Ok(data) => data,
        Err(e) => return Err(StatsError::Conversion.into()),
    };
    let y_data = match from_pyarray1(y) {
        Ok(data) => data,
        Err(e) => return Err(StatsError::Conversion.into()),
    };
    validate_statistical_input!(basic, x_data);
    validate_statistical_input!(basic, y_data);

    // Sort both arrays
    let mut x_sorted = x_data.to_vec();
    x_sorted.sort_by(|a, b| a.partial_cmp(b)
        .unwrap_or(std::cmp::Ordering::Greater));


    let mut y_sorted = y_data.to_vec();
    y_sorted.sort_by(|a, b| a.partial_cmp(b)
        .unwrap_or(std::cmp::Ordering::Greater));


    // Combine, deduplicate, and sort
    let mut combined = [x_sorted.as_slice(), y_sorted.as_slice()].concat();
    combined.sort_by(|a, b| a.partial_cmp(b)
        .unwrap_or(std::cmp::Ordering::Greater));


    // Calculate the ECDFs and find the maximum difference
    let mut max_diff = 0.0;
    for &value in &combined {
        let ecdf_x = x_sorted.iter().filter(|&&v| v <= value).count() as f64 / x_sorted.len() as f64;
        let ecdf_y = y_sorted.iter().filter(|&&v| v <= value).count() as f64 / y_sorted.len() as f64;
        let diff = (ecdf_x - ecdf_y).abs();
        if diff > max_diff {
            max_diff = diff;
        }
    }

    Ok(max_diff)
}