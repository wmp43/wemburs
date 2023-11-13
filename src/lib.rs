pub mod descriptive_statistics;
use descriptive_statistics::mean;
use pyo3::prelude::*;

#[pymodule]
fn wemburs(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mean, m)?)?;
    Ok(())
}

