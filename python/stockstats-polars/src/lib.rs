use pyo3::prelude::*;
use ::stockstats_polars::greet;

/// Wire the core Rust library into the Python module.
#[pyfunction]
fn greet_from_rust(name: &str) -> PyResult<String> {
    Ok(greet(name))
}

/// A Python module implemented in Rust.
#[pymodule]
fn stockstats_polars(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(greet_from_rust, m)?)?;
    Ok(())
}
