use container::RawMidi;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

mod container;
mod midi_event;
mod midi_main;
mod midi_process_mesg;
mod midi_send_mesg;
mod setup_client_params;

#[derive(Clone)]
#[pyclass]
struct MiBlRustProcess {
    tx: RawMidi,
    rx: RawMidi,
    need_update: bool,
}

#[pymethods]
impl MiBlRustProcess {
    #[new]
    fn new() -> Self {
        let tx = RawMidi::default();
        let rx = RawMidi::default();
        let need_update = false;

        MiBlRustProcess {
            tx,
            rx,
            need_update,
        }
    }

    fn get_rx(&self) -> &[u8] {
        self.rx.data()
    }

    fn set_tx(&mut self, delta_frames: u64, data: &[u8]) {
        let _ = self.tx.set(delta_frames, data);
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_float_custom(a: f32, b: f32) -> PyResult<f32> {
    Ok(a + b)
}

#[pyfunction]
fn mi_start_server() {
    let midi_struct = MiBlRustProcess::new();
    let midi_struct_arc = Arc::new(Mutex::new(midi_struct));
    midi_main::init_midi_audio(midi_struct_arc);
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn bl_interactive_midi(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MiBlRustProcess>()?;
    m.add_function(wrap_pyfunction!(sum_float_custom, m)?)?;
    m.add_function(wrap_pyfunction!(mi_start_server, m)?)?;

    Ok(())
}
