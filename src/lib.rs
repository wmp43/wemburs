pub mod descriptive_statistics;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
pub use descriptive_statistics::{mean, median, trimmed_mean, weighted_mean, variance, trimmed_variance, median_absolute_deviation, iqr, range, covariance, correlation, skewness, kurtosis, summary_statistics};

//
#[pymodule]
fn wemburs(py: Python, m: &PyModule) -> PyResult<()> {
    let dstats = PyModule::new(py, "descriptive_statistics")?;
    dstats.add_function(wrap_pyfunction!(mean, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(trimmed_mean, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(weighted_mean, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(median, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(variance, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(trimmed_variance, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(median_absolute_deviation, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(iqr, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(range, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(covariance, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(correlation, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(skewness, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(kurtosis, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(summary_statistics, dstats)?)?;
    m.add_submodule(dstats)?;
    Ok(())
}
    // Implement November 22nd
    //
    //
    //
    //

    //    Other Descriptive Functions to include
    //


    // outlier detection
    //    let od = PyModule::new(py, "outlier_detection")?;
    //    outlier_detection.add_function(wrap_pyfunction!(outlier_detection::median_absolute_deviation, od)?)?;


    // visualization
    //    let od = PyModule::new(py, "visualization")?;
    //    od.add_function(wrap_pyfunction!(visualization::box_plot, od)?)?;



