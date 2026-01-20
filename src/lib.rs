use pyo3::prelude::*;

mod fluxmeter;
mod geometry;
mod numpy;
mod parma;
mod particles;


#[pymodule]
fn _core(module: &Bound<PyModule>) -> PyResult<()> {
    let py = module.py();

    // Initialise the numpy interface.
    numpy::initialise(py)?;

    // Initialise parma.
    parma::initialise()?;

    // Register class object(s).
    module.add_class::<fluxmeter::Fluxmeter>()?;

    Ok(())
}
