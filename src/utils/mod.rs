use numpy::{PyArray1, PyArrayDyn};
use ndarray::{Array1, ArrayD};
use pyo3::{PyResult, FromPyObject, PyAny};


pub fn from_pyarray1(pyarray: &PyAny) -> PyResult<Array1<f64>> {
    if let Ok(array) = pyarray.extract::<PyArray1<i64>>() {
        Ok(array.to_owned_array().mapv(|x| x as f64))
    } else if let Ok(array) = pyarray.extract::<PyArray1<f64>>() {
        Ok(array.to_owned_array())
    } else {
        Err(PyErr::new::<PyRuntimeError, _>("Unsupported numpy array type. Unable to convert to ArrayD"))
    }
}

// pub fn from_pyarray_dyn(pyarray: &PyArrayDyn<f64>) -> PyResult<ArrayD<f64>> {
//     // These are just for floating point. Could cause issue if whole number
//
//     pyarray.to_owned_array()
//         .map_err(|e| PyErr::new::<PyRuntimeError, _>(format!("Failed to convert: {}", e)))
// }


pub fn from_pyarray_dyn(pyarray: &PyAny) -> PyResult<ArrayD<f64>> {
    if let Ok(array) = pyarray.extract::<PyArrayDyn<i64>>() {
        Ok(array.to_owned_array().mapv(|x| x as f64))
    } else if let Ok(array) = pyarray.extract::<PyArrayDyn<f64>>() {
        Ok(array.to_owned_array())
    } else {
        Err(PyErr::new::<PyRuntimeError, _>("Unsupported numpy array type. Unable to convert to ArrayD"))
    }
}
