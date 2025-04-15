use crate::midi_server::container::Recipe;
use crate::midi_server::midi_main::init_midi_audio;
use core::time;
use midi_server::container::{DeviceState, Event, ExtTrigger};
use pyo3::prelude::*;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};

mod midi_server;
mod node_utils;

#[derive(Clone, Debug)]
struct MiBlRustProcessInner {
    tx_triggers: Vec<ExtTrigger>,
    rx_triggers: Vec<ExtTrigger>,
    close_thread: bool,
    use_sysevent: bool,
    recipes: Recipe,
    device_state: DeviceState,
}

impl MiBlRustProcessInner {
    fn new() -> Self {
        MiBlRustProcessInner {
            tx_triggers: Vec::new(),
            rx_triggers: Vec::new(),
            close_thread: false,
            use_sysevent: true,
            recipes: Recipe::new(),
            device_state: DeviceState::default(),
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

    fn get_triggers(&self) -> Vec<ExtTrigger> {
        let mut triggers = vec![];
        triggers.append(&mut self.inner.lock().expect("lock not poisoned").rx_triggers);
        triggers
    }

    fn set_triggers(&self, triggers: Vec<ExtTrigger>) {
        self.inner.lock().expect("lock not poisoned").rx_triggers = triggers;
    }

    fn set_close_signal(&self, signal: bool) {
        self.inner.lock().expect("lock not poisoned").close_thread = signal;
    }

    fn get_close_signal(&self) -> bool {
        self.inner.lock().expect("lock not poisoned").close_thread
    }

    fn set_fps(&self, fps: u64) {
        self.inner
            .lock()
            .expect("lock not poisoned")
            .device_state
            .set_fps(fps);
    }

    fn get_timestamp(&self) -> [usize; 4] {
        *self
            .inner
            .lock()
            .expect("lock not poisoned")
            .device_state
            .get_timestamp()
    }

    fn set_timestamp(&self, hours: usize, minutes: usize, seconds: usize, frames: usize) {
        println!("Setting timestamp…");
        self.inner
            .lock()
            .expect("lock not poisoned")
            .device_state
            .set_timestamp(hours, minutes, seconds, frames);
    }

    fn mi_start_server_allow_thread(&self, py: Python) {
        py.allow_threads(|| mi_start_server(self));
    }
}

fn mi_start_server(mibl: &MiBlRustProcess) {
    let (tx_channel_rx, rx_channel_rx) = channel::<Vec<ExtTrigger>>();
    let (tx_channel_tx, rx_channel_tx) = channel::<Vec<ExtTrigger>>();
    let (tx_device_state, rx_device_state) = channel::<DeviceState>();
    let int_signal_arc = Arc::new(Mutex::new(false));
    let int_signal_arc_clone = Arc::clone(&int_signal_arc);
    let recipes_arc = Arc::new(Mutex::new(Recipe::new()));
    let init_params = DeviceState::default();

    let timestamp_arc = Arc::new(Mutex::new([0; 4]));
    let timestamp_arc_clone = Arc::clone(&timestamp_arc);

    let mut last_stamp = 0;

    let midi_audio_thread = spawn(move || {
        let sender_tx = tx_channel_rx.clone();
        let sender_device_state = tx_device_state.clone();

        init_midi_audio(
            sender_tx,
            sender_device_state,
            int_signal_arc_clone,
            recipes_arc,
            &init_params.clone(),
            timestamp_arc_clone,
        );
    });

    loop {
        let ext_signal = mibl.get_close_signal();
        let timestamp_py = mibl.get_timestamp();

        *timestamp_arc.lock().unwrap() = timestamp_py;

        if ext_signal {
            *int_signal_arc.lock().unwrap() = true;
            midi_audio_thread.join().unwrap();
            return;
        }

        if let Ok(triggers) = rx_channel_rx.try_recv() {
            mibl.set_triggers(triggers);
        }

        if let Ok(device_state) = rx_device_state.try_recv() {
            println!("Hey !");
            println!("{:?}", device_state);
        }

        sleep(Duration::from_millis(10));
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
