use numpy::{PyArray1, PyArrayDyn};
use ndarray::{Array1, ArrayD};
use pyo3::{PyResult, FromPyObject, PyAny};
// need to deal with exporting and using the following functions
// todo()!
pub fn from_pyarray1(pyarray: &PyArray1<f64>) -> PyResult<Array1<f64>> {
    Ok(pyarray.to_owned_array())
}

pub fn from_pyarray_dyn(pyarray: &PyArrayDyn<f64>) -> PyResult<ArrayD<f64>> {
    pyarray.to_owned_array().into_pyresult();
    todo()
}
