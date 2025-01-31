use crate::midi_server::container::RawMidi;
//use log::{info, logger};
use crate::midi_server::midi_main::init_midi_audio;
use pyo3::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

mod midi_server;
mod node_utils;

#[derive(Clone, Debug)]
#[pyclass]
struct MiBlRustProcess {
    tx: RawMidi,
    rx: RawMidi,
    close_thread: bool,
}

#[pymethods]
impl MiBlRustProcess {
    #[new]
    fn new() -> Self {
        let tx = RawMidi::default();
        let rx = RawMidi::default();
        let close_thread = false;

        MiBlRustProcess {
            tx,
            rx,
            close_thread,
        }
    }

    fn get_rx_data(&self) -> &[u8] {
        self.rx.data()
    }

    fn get_rx_stamp(&self) -> u64 {
        self.rx.delta_frames
    }

    fn get_tx_data(&self) -> &[u8] {
        self.tx.data()
    }

    fn get_tx_stamp(&self) -> u64 {
        self.tx.delta_frames
    }

    fn set_tx(&mut self, delta_frames: u64, data: &[u8]) {
        let _ = self.tx.set(delta_frames, data);
    }

    fn set_rx(&mut self, delta_frames: u64, data: &[u8]) {
        let _ = self.rx.set(delta_frames, data);
    }

    fn toggle_close_thread(&mut self) {
        self.close_thread = !self.close_thread;
    }

    fn get_signal(&self) -> bool {
        self.close_thread
    }

    fn mi_start_server(&mut self) {
        let midi_struct = MiBlRustProcess::new();
        let midi_struct_arc = Arc::new(Mutex::new(midi_struct));

        let duration = Duration::new(10, 0);
        let start = Instant::now();

        {
            let midi_struct_locked = midi_struct_arc.lock().unwrap();
            println!(
                "Before init_midi_audio: {:?}",
                midi_struct_locked.get_rx_data()
            );
        }

        let midi_struct_arc_clone = Arc::clone(&midi_struct_arc);

        thread::spawn(move || {
            println!("Starting midi server !");
            init_midi_audio(midi_struct_arc_clone);
        });

        let mut count: i32 = 0;

        loop {
            if start.elapsed() >= duration {
                break;
            }

            println!("Get value from thread {:?}", self.get_rx_data());

            self.set_rx(
                midi_struct_arc.lock().unwrap().get_rx_stamp(),
                midi_struct_arc.lock().unwrap().get_rx_data(),
            );

            self.set_tx(
                midi_struct_arc.lock().unwrap().get_tx_stamp(),
                midi_struct_arc.lock().unwrap().get_tx_data(),
            );

            std::thread::sleep(Duration::from_millis(100));

            count += 1;
        }

        {
            let midi_struct_locked = midi_struct_arc.lock().unwrap();
            println!("After init_midi_audio: {:?}", midi_struct_locked);
        }
    }
}

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
