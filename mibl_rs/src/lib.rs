use crate::midi_server::container::{RawMidi, MAX_MIDI_MSG_SIZE};
use crate::midi_server::midi_main::init_midi_audio;
use pyo3::prelude::*;
use simple_logger::SimpleLogger;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};

mod midi_server;
mod node_utils;

#[derive(Clone, Debug)]
struct MiBlRustProcessInner {
    tx_data: Vec<u8>,
    rx_data: Vec<u8>,
    rx_stamp: u64,
    rx_len: u8,
    close_thread: bool,
}

impl MiBlRustProcessInner {
    fn new() -> Self {
        let close_thread = false;

        MiBlRustProcessInner {
            tx_data: Vec::with_capacity(MAX_MIDI_MSG_SIZE),
            rx_data: Vec::with_capacity(MAX_MIDI_MSG_SIZE),
            rx_stamp: 0,
            rx_len: 0,
            close_thread,
        }
    }
}

#[pyclass(frozen)]
struct MiBlRustProcess {
    inner: Mutex<MiBlRustProcessInner>,
}

#[pymethods]
impl MiBlRustProcess {
    #[new]
    fn new() -> Self {
        let mibl = Mutex::new(MiBlRustProcessInner::new());

        MiBlRustProcess { inner: mibl }
    }

    fn get_rx_data(&self) -> Vec<u8> {
        self.inner
            .lock()
            .expect("lock not poisoned")
            .rx_data
            .clone()
    }

    fn get_tx_data(&self) -> Vec<u8> {
        self.inner
            .lock()
            .expect("lock not poisoned")
            .tx_data
            .clone()
    }

    fn get_rx_stamp(&self) -> u64 {
        self.inner.lock().expect("lock not poisoned").rx_stamp
    }

    fn get_data_len(&self) -> u8 {
        self.inner.lock().expect("lock not poisoned").rx_len
    }

    fn set_rx_stamp(&self, stamp: u64) {
        self.inner.lock().expect("lock not poisoned").rx_stamp = stamp;
    }

    fn set_rx_data(&self, rx_data: Vec<u8>) {
        self.inner.lock().expect("lock not poisoned").rx_data = rx_data;
    }

    fn set_tx_data(&self, tx_data: Vec<u8>) {
        self.inner.lock().expect("lock not poisoned").rx_data = tx_data;
    }

    fn set_close_signal(&self, signal: bool) {
        self.inner.lock().expect("lock not poisoned").close_thread = signal;
    }

    fn get_signal(&self) -> bool {
        self.inner.lock().expect("lock not poisoned").close_thread
    }

    fn mi_start_server_allow_thread(&self, py: Python) {
        py.allow_threads(|| mi_start_server(self));
    }
}

fn mi_start_server(mibl: &MiBlRustProcess) {
    //let duration = Duration::new(10, 0);
    //let start = Instant::now();

    //SimpleLogger::new()
    //    .with_level(log::LevelFilter::Debug)
    //    .without_timestamps()
    //    .init()
    //    .unwrap();

    let (tx_channel_rx, rx_channel_rx) = channel::<(u64, Vec<u8>)>();
    let (tx_channel_tx, rx_channel_tx) = channel::<(u64, Vec<u8>)>();
    let (tx_signal, rx_signal) = channel::<bool>();
    let int_signal = Arc::new(Mutex::new(false));
    let mut int_signal_arc = Arc::clone(&int_signal);

    let mut last_stamp = 0;

    let midi_audio_thread = spawn(move || {
        let sender_rx = tx_channel_rx.clone();
        let sender_tx = tx_channel_tx.clone();
        let sender_signal = tx_signal.clone();

        init_midi_audio(sender_tx, sender_rx, sender_signal, &mut int_signal_arc);
    });

    loop {
        let ext_signal = mibl.get_signal();

        if ext_signal {
            *int_signal.lock().unwrap() = true;
            midi_audio_thread.join().unwrap();
            return;
        }

        if let Ok((stamp, rx_data)) = rx_channel_rx.try_recv() {
            mibl.set_rx_stamp(stamp);
            mibl.set_rx_data(rx_data);
        }

        if let Ok(signal) = rx_signal.try_recv() {
            mibl.set_close_signal(signal)
        }

        sleep(Duration::from_millis(100));
    }
}

// MATH FUNCTIONS
#[pyfunction]
fn mibl_add(a: f32, b: f32) -> f32 {
    node_utils::math::add(a, b)
}

#[pyfunction]
fn mibl_multiply(a: f32, b: f32) -> f32 {
    node_utils::math::multiply(a, b)
}

#[pyfunction]
fn mibl_divide(a: f32, b: f32) -> f32 {
    node_utils::math::divide(a, b)
}

#[pyfunction]
fn mibl_abs(a: f32) -> f32 {
    node_utils::math::abs(a)
}

#[pyfunction]
fn mibl_mul_add(a: f32, b: f32, c: f32) -> f32 {
    node_utils::math::mul_add(a, b, c)
}

#[pyfunction]
fn mibl_pow(a: f32, n: f32) -> f32 {
    node_utils::math::pow(a, n)
}

#[pyfunction]
fn mibl_log(a: f32, n: f32) -> f32 {
    node_utils::math::log(a, n)
}

#[pyfunction]
fn mibl_exp(a: f32) -> f32 {
    node_utils::math::exp(a)
}

#[pyfunction]
fn mibl_sqrt(a: f32) -> f32 {
    node_utils::math::sqrt(a)
}

#[pyfunction]
fn mibl_inv_sqrt(a: f32) -> f32 {
    node_utils::math::inv_sqrt(a)
}

#[pyfunction]
fn mibl_min(a: f32, b: f32) -> f32 {
    node_utils::math::min(a, b)
}

#[pyfunction]
fn mibl_max(a: f32, b: f32) -> f32 {
    node_utils::math::max(a, b)
}

#[pyfunction]
fn mibl_compare(a: f32, b: f32, e: f32) -> bool {
    node_utils::math::compare(a, b, e)
}

#[pyfunction]
fn mibl_lt(a: f32, b: f32) -> bool {
    node_utils::math::lt(a, b)
}

#[pyfunction]
fn mibl_gt(a: f32, b: f32) -> bool {
    node_utils::math::gt(a, b)
}

#[pyfunction]
fn mibl_le(a: f32, b: f32) -> bool {
    node_utils::math::le(a, b)
}

#[pyfunction]
fn mibl_ge(a: f32, b: f32) -> bool {
    node_utils::math::ge(a, b)
}

#[pyfunction]
fn mibl_map_range(
    value: f32,
    from_min: f32,
    from_max: f32,
    to_min: f32,
    to_max: f32,
    clamp: bool,
) -> f32 {
    node_utils::math::map_range(value, from_min, from_max, to_min, to_max, clamp)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn mibllib(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MiBlRustProcess>()?;
    // MATH FUNCTION
    m.add_function(wrap_pyfunction!(mibl_add, m)?)?;
    m.add_function(wrap_pyfunction!(mibl_multiply, m)?)?;
    m.add_function(wrap_pyfunction!(mibl_divide, m)?)?;
    m.add_function(wrap_pyfunction!(mibl_abs, m)?)?;
    m.add_function(wrap_pyfunction!(mibl_mul_add, m)?)?;
    m.add_function(wrap_pyfunction!(mibl_pow, m)?)?;
    m.add_function(wrap_pyfunction!(mibl_log, m)?)?;
    m.add_function(wrap_pyfunction!(mibl_exp, m)?)?;
    m.add_function(wrap_pyfunction!(mibl_sqrt, m)?)?;
    m.add_function(wrap_pyfunction!(mibl_inv_sqrt, m)?)?;
    m.add_function(wrap_pyfunction!(mibl_min, m)?)?;
    m.add_function(wrap_pyfunction!(mibl_max, m)?)?;
    m.add_function(wrap_pyfunction!(mibl_compare, m)?)?;
    m.add_function(wrap_pyfunction!(mibl_lt, m)?)?;
    m.add_function(wrap_pyfunction!(mibl_gt, m)?)?;
    m.add_function(wrap_pyfunction!(mibl_le, m)?)?;
    m.add_function(wrap_pyfunction!(mibl_ge, m)?)?;
    m.add_function(wrap_pyfunction!(mibl_map_range, m)?)?;
    Ok(())
}
