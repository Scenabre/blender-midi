use container::RawMidi;
use log::{info, logger};
use midi_main::init_midi_audio;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

mod container;
mod midi_event;
mod midi_main;
mod midi_process_mesg;
mod midi_send_mesg;
mod setup_client_params;

#[derive(Clone, Debug)]
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

    fn get_tx(&self) -> &[u8] {
        self.tx.data()
    }

    fn set_tx(&mut self, delta_frames: u64, data: &[u8]) {
        let _ = self.tx.set(delta_frames, data);
    }

    fn set_rx(&mut self, delta_frames: u64, data: &[u8]) {
        let _ = self.rx.set(delta_frames, data);
    }

    fn mi_start_server(&mut self) {
        let midi_struct = self;
        let midi_struct_arc = Arc::new(Mutex::new(midi_struct.clone()));
        //midi_main::init_midi_audio(midi_struct_arc);

        // Debug print before the call
        {
            let midi_struct_locked = midi_struct_arc.lock().unwrap();
            println!("Before init_midi_audio: {:?}", midi_struct_locked.get_rx());
        }

        // Clone the Arc to pass it to the new thread
        let midi_struct_arc_clone = Arc::clone(&midi_struct_arc);

        // Run the server in a separate thread
        thread::spawn(move || {
            println!("Starting midi server !");
            init_midi_audio(midi_struct_arc_clone);
        });

        //init_midi_audio(midi_struct_arc.clone());

        // Debug print after the call
        {
            let midi_struct_locked = midi_struct_arc.lock().unwrap();
            println!("After init_midi_audio: {:?}", midi_struct_locked);
        }
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_float_custom(a: f32, b: f32) -> PyResult<f32> {
    Ok(a + b)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn mibllib(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MiBlRustProcess>()?;
    m.add_function(wrap_pyfunction!(sum_float_custom, m)?)?;
    Ok(())
}
