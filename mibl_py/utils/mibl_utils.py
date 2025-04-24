def generate_midi_note_bang(midi_note, vel):
    int_vel = int(round(vel*127))
    return [0x90, midi_note, 0x7F, 0x80, midi_note, int_vel]


def get_midi_note_num(note_num, octave_num):
    return note_num+12*octave_num+21


def fill_array(array, size):
    to_fill = array
    arr_len = len(to_fill)
    if arr_len < size:
        for idx in range(arr_len, size):
            to_fill.append(int(-1))
    return to_fill


def gen_timestamp(fps, curr_frame, mode):

    hours = 0
    minutes = 0
    seconds = 0
    frames = 0

    match mode:
        case 0:
            hours = int(curr_frame / (3600*fps))
            minutes = int(curr_frame / (60*fps) % 60)
            seconds = int(curr_frame / fps % 60)
            frames = curr_frame % fps
        case 1:
            frames = curr_frame % 1000
            seconds = int(curr_frame / 1000) % 100
            minutes = int(curr_frame / 100000) % 100
            hours = int(curr_frame / 10000000) % 1000
        case _:
            print("[WARNING MIBL] Timestamp gen mode unknown (0 or 1): ", mode)

    return [hours, minutes, seconds, frames]
