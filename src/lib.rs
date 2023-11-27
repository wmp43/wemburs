pub mod descriptive_statistics;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

//
#[pymodule]
fn wemburs(py: Python, m: &PyModule) -> PyResult<()> {
    let dstats = PyModule::new(py, "descriptive_statistics")?;
    dstats.add_function(wrap_pyfunction!(descriptive_statistics::mean, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(descriptive_statistics::trimmed_mean, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(descriptive_statistics::weighted_mean, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(descriptive_statistics::median, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(descriptive_statistics::variance, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(descriptive_statistics::trimmed_variance, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(descriptive_statistics::median_absolute_deviation, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(descriptive_statistics::iqr, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(descriptive_statistics::range, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(descriptive_statistics::range, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(descriptive_statistics::covariance, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(descriptive_statistics::correlation, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(descriptive_statistics::skewness, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(descriptive_statistics::kurtosis, dstats)?)?;
    dstats.add_function(wrap_pyfunction!(descriptive_statistics::summary_statistics, dstats)?)?;
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



