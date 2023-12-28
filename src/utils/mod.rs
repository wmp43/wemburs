use pyo3::exceptions::PyRuntimeError;
use numpy::{PyArray1};
use ndarray::{Array1};
use pyo3::{PyResult, FromPyObject, PyAny, PyErr};



pub fn from_pyarray1(pyarray: &PyAny) -> PyResult<Array1<f64>> {
    if let Ok(array) = pyarray.extract::<PyArray1<i64>>() {
        Ok(array.to_owned_array().mapv(|x| x as f64))
    } else if let Ok(array) = pyarray.extract::<PyArray1<f64>>() {
        Ok(array.to_owned_array())
    } else {
        Err(PyErr::new::<PyRuntimeError, _>("Unsupported numpy array type."))
    }
}
