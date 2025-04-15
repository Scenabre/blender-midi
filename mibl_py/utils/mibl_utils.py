def generate_midi_note_bang(midi_note, vel):
    int_vel = int(round(vel*127))
    return [0x90, midi_note, int_vel, 0x80, midi_note, int_vel]


def get_midi_note_num(note_num, octave_num):
    return note_num+12*octave_num+21


def fill_array(array, size):
    to_fill = array
    arr_len = len(to_fill)
    if arr_len < size:
        for idx in range(arr_len, size):
            to_fill.append(int(-1))
    return to_fill
