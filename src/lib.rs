pub mod descriptive_statistics;
use descriptive_statistics::*;
use pyo3::prelude::*;

#[pymodule]
fn wemburs(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mean, m)?)?;
    m.add_function(wrap_pyfunction!(trimmed_mean, m)?)?;
    m.add_function(wrap_pyfunction!(weighted_mean, m)?)?;
    m.add_function(wrap_pyfunction!(median, m)?)?;
    m.add_function(wrap_pyfunction!(variance, m)?)?;
    m.add_function(wrap_pyfunction!(trimmed_variance, m)?)?;
    //m.add_function(wrap_pyfunction!(median_absolute_deviation, m)?)?;
    Ok(())
}

