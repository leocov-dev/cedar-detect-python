use pyo3::prelude::PyModule;
use pyo3::{prelude::*, Bound, PyResult, Python, pymethods};
use pyo3::types::PyList;


#[pyclass(frozen, module = "histogram_funcs")]
pub struct HistogramStats {
    pub(crate) inner: cedar_detect::histogram_funcs::HistogramStats,
}

impl HistogramStats {
    pub fn wrap(inner: cedar_detect::histogram_funcs::HistogramStats) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl HistogramStats {
    #[getter]
    pub fn mean(&self) -> f32 {
        self.inner.mean
    }
    #[getter]
    pub fn median(&self) -> usize {
        self.inner.median
    }
    #[getter]
    pub fn stddev(&self) -> f32 {
        self.inner.stddev
    }

    fn __repr__(slf: &Bound<Self>) -> PyResult<String> {
        // This is the equivalent of `self.__class__.__name__` in Python.
        let binding = slf.get_type();
        let class_name = &binding.name()?;

        Ok(format!("{}(mean={}, median={}, stddev={})",
                   class_name,
                   slf.borrow().inner.mean,
                   slf.borrow().inner.median,
                   slf.borrow().inner.stddev,
        ))
    }
}

#[pyfunction]
pub fn stats_for_histogram(histogram: Vec<u32>) -> HistogramStats {
    let raw = cedar_detect::histogram_funcs::stats_for_histogram(histogram.as_slice());
    HistogramStats::wrap(raw)
}

#[pyfunction]
pub fn get_level_for_fraction(histogram: Vec<u32>, fraction: f32) -> usize {
    cedar_detect::histogram_funcs::get_level_for_fraction(histogram.as_slice(), fraction)
}

#[pyfunction]
pub fn average_top_values(histogram: Vec<u32>, num_top_values: usize) -> u8 {
    cedar_detect::histogram_funcs::average_top_values(histogram.as_slice(), num_top_values)
}

#[pyfunction]
pub fn remove_stars_from_histogram(histogram: &PyList) {
    // TODO: how to handle a mutable argument in pyo3???
    // cedar_detect::histogram_funcs::remove_stars_from_histogram()
}


// Add this module to the main module
pub fn add_module(py: Python, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let hist_mod = PyModule::new_bound(py, "histogram_funcs")?;

    hist_mod.add_class::<HistogramStats>()?;
    hist_mod.add_function(wrap_pyfunction!(stats_for_histogram, &hist_mod)?)?;
    hist_mod.add_function(wrap_pyfunction!(get_level_for_fraction, &hist_mod)?)?;
    hist_mod.add_function(wrap_pyfunction!(average_top_values, &hist_mod)?)?;
    hist_mod.add_function(wrap_pyfunction!(remove_stars_from_histogram, &hist_mod)?)?;

    parent.add_submodule(&hist_mod)?;

    Ok(())
}