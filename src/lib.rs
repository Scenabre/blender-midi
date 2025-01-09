//use crate::midi_main::main;
use pyo3::prelude::*;

mod container;
mod midi_event;
mod midi_main;
mod midi_process_mesg;
mod midi_send_mesg;
mod setup_client_params;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_float_custom(a: f32, b: f32) -> PyResult<f32> {
    Ok(a + b)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn bl_interactive_midi(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_float_custom, m)?)?;
    m.add_function(wrap_pyfunction!(midi_main::main, m)?)?;

    Ok(())
}
