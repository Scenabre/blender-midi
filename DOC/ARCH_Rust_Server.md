# Rust Server Architecture

The python lib that manage the MIDI protocol and communication with the system is written in RUST.

PyO3 is used to create the python wheel module.

**Project Name :** MIBL (**M**idi **I**nteractive (Mode) for **BL**ender)

> Below Project, MIBL, MiBL refer all to the same thing !

## All the files

### Cargo.toml

Project description, PyO3 config and deps for the project

### target/\*

All build files used when testing build with cargo or when building the wheel with maturin.

### src/lib.rs

This file contains all the functions, and structs, with PyO3 decoration.

> PUT in this file ONLY things directly used with PyO3, or linked with the python client.

The only (private) function is `mi_start_server`.

See *PyO3* and *Rust Workflow* sections for further explanation.

### src/node\_utils (/mod.rs)

All the stuffs that can be used by the Blender Python plugins (ONLY)

> Mod.rs index the files for the module

### src/node\_utils/math.rs

All math operations that needed re-immplementation due to Blender limitation (see below).

> When a custom NodeTree is created, all the logic belongs to the developper of the NodeTree. It's extremely difficult to share default ressources with a custom NodeTree (other math nodes for example, update structure)
> Blender give access only to base struct (Node, NodeTree, Properties, Operators), all needed to be implemented.

So PUT HERE only math things that mimick the others math node in Blender (e.g Add, Multiply, Map Range). I implemented this in Rust for performances and usuability.

### src/node\_utils/sys\_event.rs

Some obscur functions that can be used for good interoperability between the server and the plugin that is not present in basic MIBL `PyO3` structure (mainly to avoid mess with threads, you know)

**CURATION NEEDED in this file !**

### src/midi\_server (/mod.rs)

The main module of the project. It contains all the code to run the MIDI server, that communicate with the system, all MIDI Devices and the plugin.

This project use Threads, `Arc`, `Mutex`, `Channels` to allow listening to event without blocking Blender (Blender is a little stubborn concerning the threads and executions of loops)

### src/midi\_server/container.rs

This file define all the data structures used in MIBL. Mainly public stuffs.

#### Constants

- `MAX_MIDI_MSG_SIZE : usize` : The maximum midi message length, mandatory for certains MIDI backend, unused with midir, but anyway I keep it.

#### Types

- `Ingredient = (Vec<u8>, Vec<Vec<u8>>, Option<f32>)` : First is the message that trigger the server, second is the midi message to send to devices if the event is triggered, the last is the value to send to the python plugin if the event is triggered.
- `Recipe = Vec<Ingredient>` : A good soup made of all trigger events (Ingredient).
- `ExtTrigger = (u64, f32)` : Value to send to Python if en `Event` is triggered. (trigger index, value)
- `WaitData = (u64, Vec<Vec<u8>>)` : TODO
- `TriggerResult = (Option<Vec<RawMidi>>, Option<Vec<ExtTrigger>>)` : When en `Event` is triggered return MIDI message to send to MIDI device and send all ExtTrigger to Python.
- `MidiResult = Result<MidiProcess, &'static str>` : A Result type return `MidiProcess` if Ok, return a static str on Err. (See below for `MidiProcess` struct)

<div style="page-break-after: always; visibility: hidden"> 

\\pagebreak 

</div>

#### Structs

##### CCflag

Contain the main MIDI Continuous Controller flags (CC channel, CC number, CC lsb and msb value, CC Note).

```rust
#[derive(Debug, Clone, Copy, Default)]
pub struct CCflag {
    pub cc_lsb_flag: bool,
    pub cc_channel: u8,
    pub cc_num: u8,
    pub cc_msb_value: u8,
    pub cc_note: u8,
}
```

###### Functions

- `new(cc_lsb_flag: bool,cc_channel: u8,cc_num: u8,cc_msb_value: u8,cc_note: u8)` returns `CCflag` struct (self)

##### SIGflag

Contains all the signal flags used by the server needed to trigger some updates ask by the python plugin or the MIDI flags on the rust side (*e.g Note Bang*)

```rust
#[derive(Debug, Clone, Default)]
pub struct SIGflag {
    pub reset_signal: bool,
    pub note_on: bool,
    pub note_bang: bool,
    pub note_bang_value: u8,
    pub note_led_on: Vec<u8>,
    pub note_toggle: u8,
    pub note_need_toggle: bool,
    pub cc_flag: CCflag,
    pub update_recipe: bool,
    pub update_lcd_vec: bool,
    pub update_lcd_string: bool,
    pub update_vpot: bool,
    pub update_faders: bool,
    pub update_chan_btns: bool,
    pub update_fps: bool,
    pub stop_thread: bool,
    pub use_sys_event: bool,
    pub debug: bool,
}
```

##### RawMidi

The base struct to build a midi message that can be used by midi backend.

```rust
#[derive(Clone)]
pub struct RawMidi {
    /// The amount of time passed, in frames, relative to the start of the process cycle.
    pub delta_frames: u64, // stamp

    data: Vec<u8>,

    len: u8,
}
```

*Note : delta\_frames is the data stamp but like MAX\_MIDI\_MSG\_SIZE, it's not used by all backend. But anyway I keep it.*

###### Functions

- `new(delta_frames: u64, data: &[u8]) -> Result<Self, usize>` -> RawMidi if OK, data length if Err.
- `data(&self) -> &[u8]` -> Midi data vector
- `delta_frames(&self) -> &u64` -> Stamp but mostly useless with midir backend.

###### Traits

- `std::fmt::Debug` : Format the debug output message.
- `Default` : default values for RawMidi. all possible value set to 0.

##### Event

An Event is build with datas contained in a Ingredient. It's more elegant than only some vectors of midi messages.

TODO : find a way to add/remove attr in Event struct without breaking all the code.

```rust
#[derive(Clone)]
pub struct Event {
    index: u64,
    name: String,
    mesg_in: Vec<u8>,
    mesg_out: Option<Vec<Vec<u8>>>,
    ext_value_out: Option<f32>,
    mod_rule: u8,            // 0 in->out, 1 in+x = out, 2 in-x = out,
    mod_amount: Option<f32>, // Increase/Descrease by
    note_bang: bool,
    toggable: bool,
}
```

- `index` : Internal Event number.
- `name` : A human readable name of the Event (*e.g Fader #1 CLICK*)
- `mesg_in` : The midi message trigger. If this message is recieved by the server so the Event happened.
- `mesg_out` : The message to send to **THE MIDI DEVICE** if the Event is triggered.
- `ext_value_out` : The value to send to **THE PYTHON PLUGIN** if the Event is triggered. If arg is provided (*i.e* `Some(1.0)` ) by the python plugin, this value is **ALWAYS** sent to the plugin by the server. If `None` the server send the MIDI value sent by the MIDI device to the python plugin.
- `mod_rule` : Apply an operation to `ext_value_out` if Event is triggered. See below for possible rules.
- `note_bang` : Should the server waiting for a MIDI note bang (*i.e Note On then Note Off*) ?
- `toggable` : Is the Event is a "toggable" Event (*e.g Turn Off a button when the user click again on it*)

###### Functions

Only setter and getter, no comment, list below :

- `get_index(&self) -> &u64`
- `get_name(&self) -> &str`
- `get_mesg_in(&self) -> &Vec<u8>`
- `get_mesg_data(&self) -> &Option<Vec<Vec<u8>>>`
- `get_mod_amount(&self) -> Option<f32>`
- `get_val_out(&self) -> Option<f32>`
- `get_bang_signal(&self) -> bool`
- `get_toggable(&self) -> bool`

###### Traits

- `Default` : The default no-op Event, somewhat useless as it, but idk maybe you want create no-op Event on the fly, so here you are.
- `Debug` : A pretty printer for the Event when you want use debug print. More pretty than just numbers in the big void of your distress console.

##### DeviceState **MACKIE CONTROL PROTOCOL**

A struct to store some useful system state of the MIDI device in ***Mackie Control*** mode.

```rust
#[derive(Clone)]
pub struct DeviceState {
    timestamp: [usize; 4],
    lcd_vec: Option<Vec<(u8, u8, String)>>,
    lcd_string: Option<String>,
    vpot: Vec<[u8; 3]>,
    faders: Vec<(u8, f32)>,
    chan_btns: Vec<(u8, u8, bool)>,
    fps: u64,
}
```

- `timestamp` : HOUR, MINUTES, SECONDS, FRAMES (or BARS, BEATS, SUB DIVISION, TICKS for my fellow musician buddies)
- `lcd_vec` : LCD screen #, Line #, The message. If you want to use all the screens with line return to display a long message use `lcd_string below`. **Be careful** error is returned if the vector exceed 14 chars.
- `lcd_string` : A single string that span across all the LCD screens (useful for warnings, infos, error)
- `vpot` : Store the state of the device pan knobs. Each vector is vpot index, mode (for the led), value.
- `faders` : Store the position of the faders. Each vector is fader number, value.
- `chan_btns` : Store the state (on/off) of each buttons in the channel strip (Rec, Solo, Mute, Select). Each vector is channel number, button number, state.
- `fps` : The current fps provided by the python plugin.

###### Functions

- new : return a new DeviceState struct, or String if error (*e.g LCD string to long, channel in bad range*)

```rust
 
new(
        timestamp: [usize; 4],
        lcd_vec: Option<Vec<(u8, u8, String)>>, // (lcd_#, line_#, Message)
        lcd_string: Option<String>,
        vpot: Vec<[u8; 3]>,             // [vpot_idx, mode, value]
        faders: Vec<(u8, f32)>,         // [fader_num, pb_value]
        chan_btns: Vec<(u8, u8, bool)>, // (chan_#, btn_#, on/off)
        fps: u64,
    ) -> Result<DeviceState, String>
```

    

All the others functions is setter and getter, list below :

- `get_timestamp(&self) -> &[usize; 4]`
- `set_timestamp(&mut self, hours: usize, minutes: usize, seconds: usize, frames: usize)`
- `get_lcd_vec(&self) -> &Option<Vec<(u8, u8, String)>>`
- `set_lcd_vec(&mut self, lcd_vec: Option<Vec<(u8, u8, String)>>)`
- `get_lcd_string(&self) -> &Option<String>`
- `set_lcd_string(&mut self, str: Option<String>)`
- `get_vpots(&self) -> &Vec<[u8; 3]>`
- `set_vpots(&mut self, vpot_vec: Vec<[u8; 3]>)`
- `get_faders(&self) -> &Vec<(u8, f32)>`
- `set_faders(&mut self, fader_vec: Vec<(u8, f32)>)`
- `get_chan_btns(&self) -> &Vec<(u8, u8, bool)>`
- `set_chan_btns(&mut self, chan_btns: Vec<(u8, u8, bool)>)`
- `get_fps(&self) -> &u64`
- `set_fps(&mut self, fps: u64)`

###### Traits

- `Default` : You can use Default trait to generate a DeviceState with No-op values. Useful if the DeviceState is unknown in advance.
- `Debug` : Pretty printer to display info if use debug display in print.

##### MidiMesg

A struct used to store a MIDI message in a prettier manner than RawMidi. Use mostly to print a human readable format MIDI message to the user.

Unlike others struct, this use public attributes.

TODO : Check if needed to use getter setter here.

```rust
#[derive(Debug)]
pub struct MidiProcess {
    pub debug: Option<MidiMesg>,
    pub to_send: TriggerResult,
}
```

###### Functions

- `new() -> Self` : Create an empty midi message (channel = 0, name = "", value = 0.0)

##### MidiProcess

The `MidiProcess` struct is used in return of function. It contains a debug attribute with pretty formatted MIDI Message, and all the results of triggered Events (MIDI Messages to send to the MIDI device and values to send to python)

Unlike other struct `MidiProcess` has public attributes.

TODO : Check if the usage of public attrs is a problem.

```rust
#[derive(Debug)]
pub struct MidiProcess {
    pub debug: Option<MidiMesg>,
    pub to_send: TriggerResult,
}
```

### src/midi\_server/math\_utils.rs

A file that contains some math utils not specific to the project.

**Please don't put all your math stuff here, this file is used only for non specific tasks (e.g split all digits of a number)**

#### Functions

- `split_digits(number_to_split: &usize, vector_size: u8) -> Vec<u8>` : Split a number into its digits (*e.g* `Input = 123, Output = [1,2,3]`)

### src/midi\_server/sys\_events.rs

This file store system MIDI message value and name, specific to certain protocoles.

#### Constants

- `SYS_EVENT_ARRAY: [(u8, &str); 66]` : It's a list of all system buttons MIDI numbers used in Mackie Control Protocole. Each tuple is (MIDI btn number, Human readable btn name)

### src/midi\_server/setup\_client\_params.rs

It's a server file that handle all the code to connect the Rust server to Midi Device.

The current MIDI backend used is `midir`

#### Dependancies

From `midir` (midi backend) :

`midir::{Ignore, MidiInput, MidiInputPort, MidiOutput, MidiOutputPort};`

From `std` (threads management):

`std::sync::{Arc, Mutex};`

From `thiserror` :

`thiserror::Error;`

#### Constants

- `CLIENT_NAME_IN: &str` : The client name input for the host system.
- `CLIENT_NAME_OUT: &str` : The client name output for the host system.
- `DEFAULT_PORT_NAME: &str` : The port name for the host system.

#### Types

- `SetupResult = Result<AudioParams, ParamsInitError>` : A pretty Result used as return for `setup_client_params()` function. Contains the `AudioParams` struct if Ok, the enum `ParamsInitError` if Err.

#### Enumerators

##### ParamsInitError

Pretty self explanatory, contains all errors strings for the setup of the midi server.

```rust
#[derive(Error, Debug)]
pub enum ParamsInitError {
    #[error("No input port found")]
    InputPortNotfound,
    #[error("Unable to create MidiInput")]
    MidiInputError,
    #[error("Unable to create MidiOutput")]
    MidiOutputError,
}
```

#### Structs

##### AudioParams

This struct contains all the public parameters of the midi server.

```rust
pub struct AudioParams {
    pub port_name: String,
    pub midi_input: MidiInput,
    pub midi_input_port: MidiInputPort,
    pub midi_output: MidiOutput,
    pub midi_output_port: MidiOutputPort,
}
```

###### Attributes

- `port_name` : Simple string, the port name for the host system.
- `midi_input` : See the `midir` crates doc for explanation, search for `MidiInput`.
- `midi_input_port` : See the `midir` crates doc for explanation, search for `MidiInputPort`.
- `midi_output` : See the `midir` crates doc for explanation, search for `MidiOutput`.
- `midi_output_port` : See the `midir` crates doc for explanation, search for `MidiOutputPort`.


#### Functions

##### setup\_client\_params

`setup_client_params() -> SetupResult`

This function create Input and Output for the system MIDI backend, creates MIDI ports, connect to system MIDI server.

Return the result `SetupResult` : `AudioParams` if `Ok()`, `ParamsInitError` if `Err()`.

### src/midi\_server/midi\_event.rs

This file convert the list of triggers asked by the client into vector of `Event`. The functions return either an information use to build the human readable event name, or a `MidiMesg` used by the server.

The function `craft_recipe` build the vector, it's the core function of this file.

All others are utils functions used by core function.

#### Dependancies

From `std` : `std::vec`

From `crate` (internal) :

`use crate::midi_server::container::{Event, MidiMesg, Recipe};`

`use crate::midi_server::sys_event::SYS_EVENT_ARRAY;`

`use crate::node_utils::sys_event::convert_half;`

#### Constants

- `SIZE: usize` : Unused so need to be remove.
- `EPSILON: f32`: Unused so need to be remove.
- `CHROM_RANGE: [&str; 12]` : The octave chromatic range in human readable format (A to G#).

#### Functions

##### craft\_recipe

```rust
craft_recipe(
    use_sys: &bool,
    custom_events: Option<&Recipe>,
) -> Result<Option<Vec<Event>>, String>
```

This function get optional trigger events list `custom_events` and a boolean `use_sys`. The `use_sys` ask for `craft_recipe` to build all the `Event` of the buttons found in the `SYS_EVENT_ARRAY`. `custom_events` is a `Recipe` created by the client.

Return `Some(Vec<Event>)` or `None` on `Ok()` or a pretty formated error string on `Err()`

##### get\_note\_name

`get_note_name(note: u8) -> &'static str`

Get a MIDI note number (in the 21 to 127 range) and return the corresponding note name on the chormatic range (A to G#).

##### get\_octave

`get_octave(note: u8) -> u8`

Get a MIDI note number (in the 21 to 127 range) and return the corresponding octave number according to MIDI spec.

##### get\_channel

`get_channel(cmd: u8) -> u8`

Get the MIDI command and return the MIDI channel number.

##### process\_note

`process_note(cmd: u8, note: u8, vel: u8) -> MidiMesg`

Get a MIDI note message (command, note, velocity) and convert it into a MidiMesg.

##### process\_cc

`process_cc(cc_num: u8, cc_msb_value: u8, cc_lsb_value: Option<u8>) -> MidiMesg`

Get a MIDI Continuous Controller message (cc number, and value) and convert it into a MidiMesg.

##### process\_sys

`process_sys(event: &[u8])`

This function is No-op at this moment, because SysEX is very tricky and device specific. Virtually SysEX can send all type of binary data (including file transfer), can corrupt firmware or poisoned communication. So, for the sake of stability and security, I deactivate SysEx on my server.

But with cautious, SysEX can be used for some cool stuff, just haven't enough time to implement this at this moment.

##### process\_pitch\_bend

`process_pitch_bend(pitch: (u8, u8)) -> MidiMesg`

Get a MIDI Pitch Bend message (pitch value in `(lsb,msb)` format) and convert it into a MidiMesg.

Least Significant Byte (LSB) and Most Significant Byte (MSB), is the separation of the two bytes of a MIDI pitch bend value into two MIDI messages. So to calculate the actual value of the pitch bend (and normalize it in a `f32` between 0.0 and 1.0), we need the two lsb and msb.

### src/midi\_server/midi\_process\_mesg.rs

Only one function `process_midi_mesg` see below.

#### Dependancies

From `crate` (internal) :

- `use crate::midi_server::container::{Event, ExtTrigger, MidiMesg, MidiProcess, MidiResult, RawMidi, SIGflag, TriggerResult, MAX_MIDI_MSG_SIZE};`

- `use crate::midi_server::midi_event::{get_channel, get_note_name, get_octave, process_cc, process_note, process_pitch_bend,process_sys};`

- `use crate::midi_server::midi_send_mesg::make_raw_midi_mesg;`

- `use crate::node_utils::sys_event::convert_half;`


#### Functions

##### process\_midi\_mesg

```rust
pub fn process_midi_mesg(
    event: &RawMidi,
    protocole: &str,
    sig_flag: &mut SIGflag,
    triggers: &Option<Vec<Event>>,
) -> MidiResult
```

To avoid confusion, `craft_recipe` converts a `Recipe` into `MidiMesg`. And in this file, `process_midi_mesg` get the MIDI message sent by the MIDI device and check if there is a correspondance in the vector of `Event` created by `craft_recipe`. The function contains all the logic to manage triggers. It informs the server that a trigger exists and which value to send to the MIDI Device (feedback) and/or to the client.

At this moment only one `protocole` is used, the Mackie Control Protocole.

`event` is the input MIDI message sent by the MIDI device.

            print(ingredient.ing\_name)

`sig_flag` see `src/midi_server/container.rs`.

`triggers` is the optional vector of `Event` created by `craft_recipe`.

### src/midi\_server/midi\_send\_mesg.rs

This file contains all functions that only build `RawMidi` messages to send to the MIDI Device (\_e.g meter\_led, note\_bang\_). There are two utility functions, that help in the process, too.

#### Dependancies

From `std` :

- `use std::sync::{Arc, Mutex};` (threads management)

From `crate` (internal) :

- `use crate::midi_server::container::{DeviceState, Event, RawMidi, Recipe, SIGflag, MAX_MIDI_MSG_SIZE};`
- `use crate::midi_server::math_utils::split_digits;`
- `use crate::midi_server::midi_event::craft_recipe;`
- `use crate::midi_server::sys_event::SYS_EVENT_ARRAY;`

#### Functions


##### convert\_value\_to\_lsb\_msb

`convert_value_to_lsb_msb(value: f32) -> (u8, u8)`

Convert a Pitch Bend value (like faders) in the range `[0.0, 1.0]` to MIDI lsb,msb tuple (c.f src/midi\_server/midi\_event process\_pitch\_bend function)

- `value` is float (f32)

- return a tuple of u8.

##### make\_raw\_midi\_mesg

`make_raw_midi_mesg(stamp: &u64, mesg: &[u8]) -> Result<RawMidi, String>`

Take a timestamp and an array of u8 and convert into a `RawMidi`.

Return `RawMidi` if Ok(), a pretty formatted error message if Err().

##### make\_raw\_midi\_mesg\_fast

`make_raw_midi_mesg_fast(stamp: &u64, mesg: &[u8]) -> Result<RawMidi, String>`

NEED REFACTOR. Initally this function performed less test than `make_raw_midi_mesg`. **But now the two functions are identical**. Need to be removed or `make_raw_midi_mesg` need to implement more test to avoid user error. In fact, all my tests and conditions are performed before forge a MIDI message. It more convenient than a big function with all possible error to handle.

So need to take decision about that.

##### make\_lcd\_mesg **MACKIE CONTROL**

```rust
make_lcd_mesg(
    time: u64,
    lcd_num: u8,
    line_num: u8,
    mesg: String,
) -> Result<RawMidi, String>
```

Create a MIDI Mackie Control LCD message.

Get the time, the lcd screen number (1 to 9), the line number (each LCD screen has two lines) and a text message.

Return a `RawMidi` if Ok() or an error string if Err()

**The message must only contains u8 char (one char, one byte) !**

##### gen\_lcd\_string **MACKIE CONTROL**

`gen_lcd_string(stamp: u64, mesg: Option<String>) -> Result<Vec<RawMidi>, String>`

Create a MIDI Mackie Control LCD message.

A more convenient way to display a message on Mackie Control Surface. Your message span all LCD screens, so no need to calculate the number of chars for each screen.

Get the time and a Optional String. If the mesg is `None`, flush out all LCD screens (convenient to erase all messages).

Return a `RawMidi` if Ok() or an error string if Err()

##### timestamp\_gen **MACKIE CONTROL**

```rust
timestamp_gen(
    hours: usize,
    minutes: usize,
    seconds: usize,
    frames: usize,
) -> Result<Vec<RawMidi>, String>
```

TODO : Rename the function to transport\_timecode

Generate the LCD timecode MIDI message.

Get the four timecode elements : frames/ticks, seconds/sub\_division, minutes/beats, hours/bars.

Return a vector of `RawMidi` if Ok(), error string otherwise.

##### assign\_gen **MACKIE CONTROL**

`assign_gen(assign: usize) -> Result<Vec<RawMidi>, String>`

Generate the LCD assignment MIDI message.

Get the assignment number.

Returns a vector of `RawMidi` if Ok(), error string otherwise.

##### pan\_knob\_gen **MACKIE CONTROL**

`pan_knob_gen(mode: u8, knob_num: u8, knob_value: u8) -> Result<RawMidi, String>`

Generate MIDI message of knobs LEDs.

The table below show the LEDs rings state in function of the mode and the value. 

|          |  Mode 0 (`0b00`)  |  Mode 1 (`0b01`)  |  Mode 2 (`0b10`)  |  Mode 3 (`0b11`)  |
| --- | --- | --- | --- | --- |
|  `0x00`  |   `-----------`   |   `-----------`   |   `-----------`   |   `-----------`   |
|  `0x01`  |   `×----------`   |   `××××××-----`   |   `×----------`   |   `-----×-----`   |
|  `0x02`  |   `-×---------`   |   `-×××××-----`   |   `××---------`   |   `----×××----`   |
|  `0x03`  |   `--×--------`   |   `--××××-----`   |   `×××--------`   |   `---×××××---`   |
|  `0x04`  |   `---×-------`   |   `---×××-----`   |   `××××-------`   |   `--×××××××--`   |
|  `0x05`  |   `----×------`   |   `----××-----`   |   `×××××------`   |   `-×××××××××-`   |
|  `0x06`  |   `-----×-----`   |   `-----×-----`   |   `××××××-----`   |   `××××××××××`    |
|  `0x07`  |   `------×----`   |   `-----××----`   |   `×××××××----`   |   `××××××××××`    |
|  `0x08`  |   `-------×---`   |   `-----×××---`   |   `××××××××---`   |   `××××××××××`    |
|  `0x09`  |   `--------×--`   |   `-----××××--`   |   `×××××××××--`   |   `××××××××××`    |
|  `0x0A`  |   `---------×-`   |   `-----×××××-`   |   `××××××××××-`   |   `××××××××××`    |
|  `0x0B`  |   `----------×`   |   `-----××××××`   |   `×××××××××××`   |   `××××××××××`    |

Get the mode (see above), the knob number (1..=8) and the knob value (0..=11).

Returns `RawMidi` if Ok(), error string if Err().

##### meter\_led **MACKIE CONTROL**

`meter_led(meter_num: u8, sound_value: i8, clip: bool) -> Result<RawMidi, String>`

Generate the LED level meter MIDI message.

Get the meter number, the sound value in dB and a cliping boolean.

Returns `RawMidi` if Ok(), error string if Err().

For reference :

The MIDI message used is a *Channel Pressure* (0x0D). The value field of the message (1 byte), is divided into two 4 bits. The least significants bits are for the sound value (see table below), the most significants bits are for the meter number (channel).

*Note : not all control surface has 13 metering LEDs*

|  Value   |  Signal           |  LEDs Color        |
| --- | --- | --- |
|  `0x-F`  |  Clear overload   |  ?                 |
|  `0x-E`  |  Set overload     |  ?                 |
|  `0x-D`  |  100 % (\\> 0 dB)  |  ?                 |
|  `0x-C`  |  0 dB             |  Red (clip)        |
|  `0x-B`  |  \\>= -2 dB        |  Yellow            |
|  `0x-A`  |  \\>= -4 dB        |  Yellow            |
|  `0x-9`  |  \\>= -6 dB        |  Yellow            |
|  `0x-8`  |  \\>= -8 dB        |  Green             |
|  `0x-7`  |  \\>= -10 dB       |  Green             |
|  `0x-6`  |  \\>= -14 dB       |  Green             |
|  `0x-5`  |  \\>= -20 dB       |  Green             |
|  `0x-4`  |  \\>= -30 dB       |  Green             |
|  `0x-3`  |  \\>= -40 dB       |  Green             |
|  `0x-2`  |  \\>= -50 dB       |  Green             |
|  `0x-1`  |  \\>= -60 dB       |  Green             |
|  `0x-0`  |  0% < -60 dB      |  All LEDs Off      |


##### send\_note\_bang

`send_note_bang(note: u8, led_value: u8) -> Result<Vec<RawMidi>, String>`

Generate a *Note Bang* MIDI message. Not protocole specific I think. A note bang is a *Note On* message directly follow by a *Note Off* message.

Get the note to trigger, and the LED state (Off, Blink, Solid, see table below).

Returns a vector of `RawMidi` if Ok(), error string otherwise.

|  LED State  |  Velocity  |  Hex     |  Remarks                      |
| --- | --- | --- | --- |
|  Off        |  0         |  `0x00`  |  EVEN numbers                 |
|  Blink      |  1         |  `0x01`  |  ODD numbers except `0xF7`    |
|  Solid      |  127       |  `0xF7`  |                               |

##### initialize\_mc\_device **MACKIE CONTROL**

`initialize_mc_device(init_values: &DeviceState) -> Result<Vec<RawMidi>, String>`

Initialize a Mackie Control device with `DeviceState`.

This function should be call when the server start in MC mode, so the user can have a feedback of the server initialization and can perform some visual checks.

Get `DeviceState` (c.f `src/midi_server/container.rs`)

Return a vector of `RawMidi` id Ok(), error string if Err().

##### reset\_mc\_device **MACKIE CONTROL**

`reset_mc_device() -> Result<Vec<RawMidi>, String>`

Reset the state of a Mackie Control device. Useful to flush out all LCD messages, buttons state, etc.

Return a vector of `RawMidi` id Ok(), error string if Err().

##### signal\_handling

```rust
signal_handling(
    int_signal: &Arc<Mutex<SIGflag>>,
    triggers_events: &Arc<Mutex<Option<Vec<Event>>>>,
    recipe: &Arc<Mutex<Recipe>>,
    device_params: &Arc<Mutex<DeviceState>>,
    debug: bool,
) -> Option<Vec<RawMidi>>
```

Use extensively `Arc` and `Mutex`. This function handle the updates of the server flags, the trigger events, the recipe and `DeviceState`.

Signals can be sent by the client, internaly by the server or triggered by the MIDI device. This is a core part of the MIDI server. Furthers explanations in `src/midi_server/midi_main.rs`.

Get `Arc<Mutex<T>>` of : `SIGflag`, `Option<Vec<Event>>`, `Recipe` and `DeviceState`. Get also a boolean to print (or not) debug messages.

### src/midi\_server/midi\_main.rs

This is the main file of the MIDI server. Here you can find the initialization of the server and the callback that handle the inputs. These functions use `Arc` and `Mutex` intensively.

#### Dependancies

From `crate` (internal) :

- `crate::midi_server::container::{DeviceState, Event, ExtTrigger, RawMidi, Recipe, SIGflag};` : see `src/midi_server/container.rs`
- `crate::midi_server::midi_event::craft_recipe;` : see `src/midi_server/midi_event.rs`
- `crate::midi_server::midi_process_mesg::process_midi_mesg;` : see `src/midi_server/midi_process_mesg.rs`
- \`crate::midi\_server::midi\_send\_mesg::{

    gen\_lcd\_string, initialize\_mc\_device, reset\_mc\_device, signal\_handling, timestamp\_gen,

};`&#32;: see `src/midi_server/midi_send_mesg.rs`

- `crate::midi_server::setup_client_params::setup_client_params;` : see `src/midi_server/setup_client_params.rs`

From `std` :

- `std::sync::mpsc::{channel, Sender};` : use standard channel to enable communication between threads (safe).
- `std::sync::{Arc, Mutex};` : thread data management.
- `std::thread::sleep;` : wait a duration before continuing.
- `std::time::Duration;` : Rust `Duration` struct to express time in human readable format.

From `log` :

- `log::{error, info, warn};` **UNUSED NEED TO BE REMOVED** 

From `midir` :

- `midir::MidiOutputConnection;` : MIDI backend `midir` used by the server in `setup_client_params` (c.f `src/midi_server/setup_client_params`)

#### Functions

##### init\_midi\_audio

```rust
pub fn init_midi_audio(
    tx: Sender<Vec<ExtTrigger>>,
    ext_signal: Sender<DeviceState>,
    int_signal: Arc<Mutex<SIGflag>>,
    recipe: Arc<Mutex<Recipe>>,
    device_params: Arc<Mutex<DeviceState>>,
) 
```

Initialize the MIDI server then handle all updates from client (main thread) and MIDI device.

See the `Blender_Rust_Communication.md` for further explanations on the workflow.

##### input\_callback

```rust
fn input_callback(
    data_in: (&u64, &[u8]),
    sigflag: &Arc<Mutex<SIGflag>>,
    int_tx: &Sender<Vec<Vec<u8>>>,
    ext_tx: &Sender<Vec<ExtTrigger>>,
    triggers: &Arc<Mutex<Option<Vec<Event>>>>,
    device_params: &Arc<Mutex<DeviceState>>,
)
```

Each time a MIDI message is recieved by the server, call this function for message processing. This function dispatch data either to the MIDI device or to the client. Handle signals too (e.g resetting device, or note bang).

Get references to `Arc`, `Mutex`, `Sender`, created in `init_midi_audio`.

Return nothing, but display errors and debug message in `stdout`.

## PyO3 integration and Python specific code

This program strictly separate the core logic of the server from the language bindings. So the client can be programmed in any language with minimal modification of the code base (virtually only a new `lib.rs` should be created to bind with a new language).

Blender extension use Python. So the first binding is for Python language. The `PyO3` lib allow to create a Python compatible Rust lib.

We'll explore the `src/lib.rs` file.

All basics math functions of this file will be skipped. These functions call the corresponding math function implemented in `src/node_utils/math.rs`.

### Dependancies

From `crate` (internal):

- `crate::midi_server::midi_main::init_midi_audio`
- `crate::midi_server::container::{DeviceState, Event, ExtTrigger, SIGflag, Recipe}`

From `core` :

- `core::time`

From `PyO3` :

- `pyo3::prelude::*`

From `std` :

- `std::sync::mpsc::channel;`
- `std::sync::{Arc, Mutex};`
- `std::thread::{sleep, spawn};`
- `std::time::{Duration, Instant};`

### Modules

Call two modules :

- `midi_server` : where all the server logic is stored.
- `node_utils` : a group of functions usefull in the client implementation (*e.g Math for Blender nodes*)

### Structs

#### MiBlRustProcessInner

This is a workaround to enable the usage of `Mutex` in a `pyclass` because a `pyclass` can't be `Mutex` by design.

So the `MiBlRustProcess` contain a "inner" attribute that contain the mutable thread-safe structure described here.

```rust
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
```

`tx_triggers` and `rx_triggers` are use for the exchange 

#### MiBlRustProcess

```rust
#[pyclass(frozen)]
struct MiBlRustProcess {
    inner: Mutex<MiBlRustProcessInner>,
}
```

```rust
#[pymethods]
impl MiBlRustProcess {
    #[new]
    fn new() -> Self
    fn get_triggers(&self) -> Vec<ExtTrigger>
    fn set_triggers(&self, triggers: Vec<ExtTrigger>)
    fn get_close_signal(&self) -> bool
    fn set_close_signal(&self, signal: bool)
    fn get_sysevent(&self) -> bool
    fn set_sysevent(&self, use_sysevent: bool)
    fn get_devicestate(&self) -> PyDeviceState
    fn get_devicestate_update(&self) -> Option<Vec<u8>>
    fn set_devicestate_update(&self, updates: Vec<u8>)
    fn get_timestamp(&self) -> [usize; 4]
    fn set_timestamp(&self, hours: usize, minutes: usize, seconds: usize, frames: usize)
    fn get_lcd_vec(&self) -> Option<Vec<(u8, u8, String)>>
    fn set_lcd_vec(&self, lcd_vec: Vec<(u8, u8, String)>)
    fn get_lcd_string(&self) -> Option<String>
    fn set_lcd_string(&self, lcd_string: String)
    fn get_vpots(&self) -> Vec<[u8; 3]>
    fn set_vpots(&self, vpots: Vec<[u8; 3]>)
    fn get_faders(&self) -> Vec<(u8, f32)>
    fn set_faders(&self, faders: Vec<(u8, f32)>)
    fn get_chan_btns(&self) -> Vec<(u8, u8, bool)>
    fn set_chan_btns(&self, chan_btns: Vec<(u8, u8, bool)>)
    fn get_fps(&self) -> u64
    fn set_fps(&self, fps: u64)
    fn get_recipe(&self) -> Recipe
    fn set_recipe(&self, recipe: Recipe)
    fn get_recipe_need_update(&self) -> bool
    fn set_recipe_need_update(&self, update: bool)
    fn get_toggle_btn(&self) -> u8
    fn set_toggle_btn(&self, btn: u8)
    fn get_toggle_need_update(&self) -> bool
    fn set_toggle_need_update(&self, state: bool)
    fn mi_start_server_allow_thread(&self, debug: bool, py: Python)
}
```

##### Attributes

##### Functions

#### PyDeviceState

```rust
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
```

### Functions

#### mibllib

```rust
#[pymodule]
fn mibllib(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()>
```


## Server workflow and threads architecture

### TL;TD

1. The client creates `MiBlRustProcess`.
2. The client spawns the server with `mi_start_server_allow_thread()`. There are now, two threads, one for client and one for the MIDI server.
3. The server creates two bi-directional channels : one to exchange datas, one to exchange `DeviceState` updates.
4. The server start its loop.
5. The client and the server can exchange data via methods and struct in `MiBlRustProcess`

### Channels, Sync, Threads explanation

#### Channels

- External Channels (exchange ExtTrigger, Rust <-> Python Plugins)
- Internal Channels (exchange u8, Rust <-> Midi Device)
- DeviceState channels


