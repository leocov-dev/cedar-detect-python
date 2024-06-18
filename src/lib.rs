use pyo3::{prelude::*};

#[macro_use]
mod algorithm_wrapper;
mod histogram_funcs_wrapper;

#[pyfunction]
fn extract_centroids() -> PyResult<()> {
    // TODO: not sure about implementing this, but it should call the already wrapped funcs
    Ok(())
}

#[pymodule]
#[pyo3(name = "_cedar_detect")]
fn py_module(py:Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    algorithm_wrapper::add_module(py, m)?;
    histogram_funcs_wrapper::add_module(py, m)?;

    m.add_function(wrap_pyfunction!(extract_centroids, m)?)?;

    Ok(())
}