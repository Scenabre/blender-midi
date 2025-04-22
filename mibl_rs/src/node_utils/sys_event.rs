use crate::midi_server::sys_event::SYS_EVENT_ARRAY;

pub fn get_event_by_index(idx: usize) -> Option<(u8, String)> {
    if idx >= SYS_EVENT_ARRAY.len() {
        println!(
            "Unable to pick event at index {} index out of range ({})",
            idx,
            SYS_EVENT_ARRAY.len()
        );
        return None;
    }

    Some((SYS_EVENT_ARRAY[idx].0, SYS_EVENT_ARRAY[idx].1.to_string()))
}

pub fn convert_half(vel: u8) -> f32 {
    vel as f32 / 127.0
}
