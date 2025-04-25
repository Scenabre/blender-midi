use crate::midi_server::container::Recipe;
use crate::midi_server::midi_main::init_midi_audio;
use core::time;
use midi_server::container::{DeviceState, Event, ExtTrigger, SIGflag};
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
    recipe: Recipe,
    recipe_need_update: bool,
    device_state: PyDeviceState,
    toggle_btn: u8,
    toggle_btn_sig: bool,
}

impl MiBlRustProcessInner {
    fn new() -> Self {
        MiBlRustProcessInner {
            tx_triggers: Vec::new(),
            rx_triggers: Vec::new(),
            close_thread: false,
            use_sysevent: true,
            recipe: Recipe::new(),
            recipe_need_update: true,
            device_state: PyDeviceState::default(),
            toggle_btn: 0,
            toggle_btn_sig: false,
        }
    }
}

#[pyclass(frozen)]
struct MiBlRustProcess {
    inner: Mutex<MiBlRustProcessInner>,
}

#[derive(Clone, Debug, Default)]
#[pyclass(frozen)]
struct PyDeviceState {
    lcd_vec_update: bool,
    lcd_string_update: bool,
    vpot_update: bool,
    faders_update: bool,
    chan_btns_update: bool,
    fps_update: bool,
    inner: DeviceState,
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

    fn get_sysevent(&self) -> bool {
        self.inner.lock().expect("lock not poisoned").use_sysevent
    }

    fn set_sysevent(&self, use_sysevent: bool) {
        self.inner.lock().expect("lock not poisoned").use_sysevent = use_sysevent;
    }

    fn get_devicestate(&self) -> PyDeviceState {
        self.inner
            .lock()
            .expect("lock not poisoned")
            .device_state
            .clone()
    }

    fn get_devicestate_update(&self) -> Option<Vec<u8>> {
        let mut updates: Option<Vec<u8>> = None;
        let mut tmp_update = vec![];

        if self.inner.lock().unwrap().device_state.lcd_vec_update {
            tmp_update.push(0);
        }

        if self.inner.lock().unwrap().device_state.lcd_string_update {
            tmp_update.push(1);
        }

        if self.inner.lock().unwrap().device_state.vpot_update {
            tmp_update.push(2);
        }

        if self.inner.lock().unwrap().device_state.faders_update {
            tmp_update.push(3);
        }

        if self.inner.lock().unwrap().device_state.chan_btns_update {
            tmp_update.push(4);
        }

        if self.inner.lock().unwrap().device_state.fps_update {
            tmp_update.push(5);
        }

        if !tmp_update.is_empty() {
            updates = Some(tmp_update);
        }

        updates
    }

    fn set_devicestate_update(&self, updates: Vec<u8>) {
        for update in updates {
            match update {
                0 => self.inner.lock().unwrap().device_state.lcd_vec_update = true,
                1 => self.inner.lock().unwrap().device_state.lcd_string_update = true,
                2 => self.inner.lock().unwrap().device_state.vpot_update = true,
                3 => self.inner.lock().unwrap().device_state.faders_update = true,
                4 => self.inner.lock().unwrap().device_state.chan_btns_update = true,
                5 => self.inner.lock().unwrap().device_state.fps_update = true,
                _ => println!("Attribute unknown"),
            }
        }
    }

    fn get_timestamp(&self) -> [usize; 4] {
        *self
            .inner
            .lock()
            .expect("lock not poisoned")
            .device_state
            .inner
            .get_timestamp()
    }

    fn set_timestamp(&self, hours: usize, minutes: usize, seconds: usize, frames: usize) {
        self.inner
            .lock()
            .expect("lock not poisoned")
            .device_state
            .inner
            .set_timestamp(hours, minutes, seconds, frames);
    }

    fn get_lcd_vec(&self) -> Option<Vec<(u8, u8, String)>> {
        self.inner
            .lock()
            .expect("lock not poisoned")
            .device_state
            .inner
            .get_lcd_vec()
            .clone()
    }

    fn set_lcd_vec(&self, lcd_vec: Vec<(u8, u8, String)>) {
        self.inner
            .lock()
            .expect("lock not poisoned")
            .device_state
            .inner
            .set_lcd_vec(Some(lcd_vec));
    }

    fn get_lcd_string(&self) -> Option<String> {
        self.inner
            .lock()
            .expect("lock not poisoned")
            .device_state
            .inner
            .get_lcd_string()
            .clone()
    }

    fn set_lcd_string(&self, lcd_string: String) {
        self.inner
            .lock()
            .expect("lock not poisoned")
            .device_state
            .inner
            .set_lcd_string(Some(lcd_string));
    }

    fn get_vpots(&self) -> Vec<[u8; 3]> {
        self.inner
            .lock()
            .expect("lock not poisoned")
            .device_state
            .inner
            .get_vpots()
            .clone()
    }

    fn set_vpots(&self, vpots: Vec<[u8; 3]>) {
        self.inner
            .lock()
            .expect("lock not poisoned")
            .device_state
            .inner
            .set_vpots(vpots);
    }

    fn get_faders(&self) -> Vec<(u8, f32)> {
        self.inner
            .lock()
            .expect("lock not poisoned")
            .device_state
            .inner
            .get_faders()
            .clone()
    }

    fn set_faders(&self, faders: Vec<(u8, f32)>) {
        self.inner
            .lock()
            .expect("lock not poisoned")
            .device_state
            .inner
            .set_faders(faders);
    }

    fn get_chan_btns(&self) -> Vec<(u8, u8, bool)> {
        self.inner
            .lock()
            .expect("lock not poisoned")
            .device_state
            .inner
            .get_chan_btns()
            .clone()
    }

    fn set_chan_btns(&self, chan_btns: Vec<(u8, u8, bool)>) {
        self.inner
            .lock()
            .expect("lock not poisoned")
            .device_state
            .inner
            .set_chan_btns(chan_btns)
    }

    fn get_fps(&self) -> u64 {
        *self
            .inner
            .lock()
            .expect("lock not poisoned")
            .device_state
            .inner
            .get_fps()
    }

    fn set_fps(&self, fps: u64) {
        self.inner
            .lock()
            .expect("lock not poisoned")
            .device_state
            .inner
            .set_fps(fps);
    }

    fn get_recipe(&self) -> Recipe {
        self.inner.lock().expect("lock not poisoned").recipe.clone()
    }

    fn set_recipe(&self, recipe: Recipe) {
        self.inner.lock().expect("lock not poisoned").recipe = recipe;
    }

    fn get_recipe_need_update(&self) -> bool {
        self.inner
            .lock()
            .expect("lock not poisoned")
            .recipe_need_update
    }

    fn set_recipe_need_update(&self, update: bool) {
        self.inner
            .lock()
            .expect("lock not poisoned")
            .recipe_need_update = update
    }

    fn get_toggle_btn(&self) -> u8 {
        self.inner.lock().expect("lock not poisoned").toggle_btn
    }

    fn set_toggle_btn(&self, btn: u8) {
        self.inner.lock().expect("lock not poisoned").toggle_btn = btn;
    }

    fn get_toggle_need_update(&self) -> bool {
        self.inner.lock().expect("lock not poisoned").toggle_btn_sig
    }

    fn set_toggle_need_update(&self, state: bool) {
        self.inner.lock().expect("lock not poisoned").toggle_btn_sig = state;
    }

    fn mi_start_server_allow_thread(&self, debug: bool, py: Python) {
        py.allow_threads(|| mi_start_server(self, debug));
    }
}

fn mi_start_server(mibl: &MiBlRustProcess, debug: bool) {
    let (tx_channel_rx, rx_channel_rx) = channel::<Vec<ExtTrigger>>();
    let (tx_channel_tx, rx_channel_tx) = channel::<Vec<ExtTrigger>>();
    let (tx_device_state, rx_device_state) = channel::<DeviceState>();

    let int_signal_arc = Arc::new(Mutex::new(SIGflag {
        debug,
        use_sys_event: mibl.get_sysevent(),
        ..Default::default()
    }));

    let int_signal_arc_clone = Arc::clone(&int_signal_arc);

    let recipe_arc = Arc::new(Mutex::new(mibl.get_recipe()));
    let recipe_arc_clone = Arc::clone(&recipe_arc);

    let orig_device_state = mibl.get_devicestate().inner;

    let device_state = Arc::new(Mutex::new(orig_device_state.clone()));
    drop(orig_device_state);
    let device_state_clone = Arc::clone(&device_state);

    let device_params_lock = device_state.lock().unwrap();
    let mut fps = *device_params_lock.get_fps();
    drop(device_params_lock);

    let mut duration: u64 = 1000 / fps;

    let midi_audio_thread = spawn(move || {
        let sender_tx = tx_channel_rx.clone();
        let sender_device_state = tx_device_state.clone();

        init_midi_audio(
            sender_tx,
            sender_device_state,
            int_signal_arc_clone,
            recipe_arc_clone,
            device_state_clone,
        );
    });

    loop {
        let ext_signal = mibl.get_close_signal();

        if ext_signal {
            int_signal_arc.lock().unwrap().stop_thread = true;
            midi_audio_thread.join().unwrap();
            return;
        }

        let timestamp_py = mibl.get_timestamp();

        if let Some(updates) = mibl.get_devicestate_update() {
            if updates.contains(&0) {
                let lcd_vec = mibl.get_lcd_vec();
                device_state.lock().unwrap().set_lcd_vec(lcd_vec);
                int_signal_arc.lock().unwrap().update_lcd_vec = true;
            }

            if updates.contains(&1) {
                let lcd_string = mibl.get_lcd_string();
                device_state.lock().unwrap().set_lcd_string(lcd_string);
                int_signal_arc.lock().unwrap().update_lcd_string = true;
            }

            if updates.contains(&2) {
                let vpots = mibl.get_vpots();
                device_state.lock().unwrap().set_vpots(vpots);
                int_signal_arc.lock().unwrap().update_vpot = true;
            }

            if updates.contains(&3) {
                let fader_vec = mibl.get_faders();
                device_state.lock().unwrap().set_faders(fader_vec);
                int_signal_arc.lock().unwrap().update_faders = true;
            }

            if updates.contains(&4) {
                let chan_btns = mibl.get_chan_btns();
                device_state.lock().unwrap().set_chan_btns(chan_btns);
                int_signal_arc.lock().unwrap().update_chan_btns = true;
            }

            if updates.contains(&5) {
                fps = mibl.get_fps();
                duration = 1000 / fps;
                device_state.lock().unwrap().set_fps(fps);
                int_signal_arc.lock().unwrap().update_fps = true;
            }
        }

        if mibl.get_recipe_need_update() {
            let py_recipe = mibl.get_recipe();
            println!("Get recipe from python : {:?}", py_recipe);
            *recipe_arc.lock().unwrap() = py_recipe;
            int_signal_arc.lock().unwrap().use_sys_event = mibl.get_sysevent();
            int_signal_arc.lock().unwrap().update_recipe = true;
            mibl.set_recipe_need_update(false);
        }

        if mibl.get_toggle_need_update() {
            let toggle_btn = mibl.get_toggle_btn();
            int_signal_arc.lock().unwrap().note_need_toggle = true;
            int_signal_arc.lock().unwrap().note_toggle = toggle_btn;
        }

        device_state.lock().unwrap().set_timestamp(
            timestamp_py[0],
            timestamp_py[1],
            timestamp_py[2],
            timestamp_py[3],
        );

        if let Ok(triggers) = rx_channel_rx.try_recv() {
            mibl.set_triggers(triggers);
        }

        if let Ok(device_state) = rx_device_state.try_recv() {
            println!("Hey !");
            println!("{:?}", device_state);
        }

        sleep(Duration::from_millis(duration));
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

#[pyfunction]
fn mibl_get_event_by_index(idx: usize) -> Option<(u8, String)> {
    node_utils::sys_event::get_event_by_index(idx)
}

#[pyfunction]
fn mibl_get_sys_event_len() -> usize {
    node_utils::sys_event::get_sys_event_len()
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
    m.add_function(wrap_pyfunction!(mibl_get_event_by_index, m)?)?;
    m.add_function(wrap_pyfunction!(mibl_get_sys_event_len, m)?)?;
    Ok(())
}
