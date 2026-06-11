# MIDI Interactive mode for BLender (MIBL)

This project aims to add a node tree that allows a MIDI device to be connected to Geometry Nodes (as well as system `Operators` like save, play,…) via a new `Interactive Mode` node tree.

## Goal

This project originated from the need to enable non technical users like designers and artists, to control `Blender Geometry Nodes`. The geo node panel with public exposed variables was not sufficient.

In the field of live audio and video, we rely heavily on consoles, and real-time hardware devices (__e.g MIDI, DMX, SDI__), wetherefor decided to try connecting these devices to Blender Geo Node for realtime procedural geometry processing.

Blender can read MIDI files but not real-time MIDI stream. Futhermore, we can't modify "Geometry Node" using plugins (i.e to add nodes or alter the update chain). So, we've created a Blender Plugin that adds a new "Node Tree" system allowing you to create and update Blender's geometry attributes. These attributes can be read by "Geometry Node".

This project is part of a broader project, [Chamanyte](https://chamanyte.dev) a comprehensive system for designing procedural ecosystems (primarily terrain and vegetation), based on preconfigured rules and that can be tweaked in real time using external devices.

To build the prototype for this major project, we chose Blender and MIDI Device with MCU (Mackie Control protocol) capabilities. So here we are :)

## Limitations

Blender doesn't like threads and external loops (perhaps because of CPython's GIL ?). So we treaked a little in operators to be able to run threads. This isn't a problem, but it requiers some polish to be beautiful (and don't break the garbage collector and process flow of Blender when quiting the software)

For now, only MCU is supported; therefore, all control surfaces with MCU should be work with the plugin.

## Features

### TO DO

#### Rust Lib

- Cross-plateform code (easy: the MIDI backend is already cross-plateform)
- Expose a list of devices (easy: `midir` provides it; just integrate it into `DeviceState`)
- Expose a list of available backends (unknown: consult the `midir` doc)
- Implement others protocol such as HUI or Raw MIDI.

#### Blender Plugin

- Select the MIDI device and backend in the GUI (hardcoded at this moment)
- Make universal protocol nodes protocol-independant (like MCU)

### DONE

#### Rust Lib

- Linux plateform supported with `Jack`, `Alsa`, etc.
- Mackie Device protocol
- Selection of wich MIDI features can be triggered (e.g to retrieve the value of Pan Knob on channel 1)
- Internal feedback when a trigger is activated (e.g trigering Pan Knob sends a value to a fader)
- Bidirectional communication.

#### Blender Plugin

- Implement all sockets, ops, props.
- Implement group IN/OUT nodes of the node tree.
- Implement the logical update flow for the node tree (NEED REVIEW and POLISH. This is a particularly delicate part)

## INSTALLATION INSTRUCTIONS

### General dependancies

- `rust` and `cargo`
- `maturin` or other tools for compiling the Rust lib into a Python wheel

### Linux

#### Dependancies

- An implementation of `jack2`, either via Pipewire (recommended) with `pipewire-jack` _OR_ via standalone `jack2`.
- An implementation `alsa`, either via `pipewire-alse` (recommended) _OR_ via the packages provided by your distribution.
- `a2jmidid` is recommended to convert alsa MIDI ports into Jack ones.

#### Build and Use

Creating a Python virtual environment is recommended on most modern Linux distributions.

##### Build the Rust lib

1. Build the Rust lib into Python wheels `> maturin build`

2. If no error is provided copy the `target/wheels/mibllib-{version}-{CPythonVersion}-{CPythonVersion}-{Plateform}.whl` into `mibl_py/wheels/`

3. Then build the Blender extensions using `blender-launcher` script in blender install dir (see below).

#### Build and install the Blender plugin

We recommend using the `blender-launcher` script located Blender installation directory. If you prefer to use the `blender` executable, simply modify the command lines accordingly while keeping the same arguments.

- Get addon `ID` and addon `Version`. 

```
cd /path/to/mibl_py
export addon_id=$(grep -m 1 id blender_manifest.toml | cut -d '=' -f 2 | tr -d '"' | tr -d ' ')
export addon_version=$(grep -m 1 version blender_manifest.toml | cut -d '=' -f 2 | tr -d '"' | tr -d ' ')
```

- Remove previously installed extension if needed : `blender-launcher --command extension remove $addon_id`

- Build the extension : `blender-launcher --command extension build`

- Install the extension in user space: `blender-launcher --command extension install-file -r user_default $addon_id-$addon_version.zip`

- Open Blender, activate the extension, set preferences as needed.

### Run the addon

- Get the `Interactive Mode` region.
- Add the `Group Input` and `Group Output` Nodes.
- In the right panel's (under N) `MiBl` tab, you can start and stop the `Midi Server`. 

## Documentation

All details regarding the project's architecture, as well as the `NodeTree` reference, can be found in the `DOC` folder.

We typically convert Markdown documents to PDF using `ghpdf`.

## WARNINGS

This project is a prototype and relies heavily on external dependancies such as `midir`.

These dependancies were designed with cross-plateform compatibility in mind, which means they don't always meet the requirements for real-time audio.

My code is highly modular and organized in a funnel-like structure: the deeper you go into the modules, the less they depend on a specific plateform or software.
Thus, if the loop design is not suited for real-time, I can update the code according to rules without breaking the entire logic.
The same applies to software plugin dependencies.

Currently, Rust is used for the backend and then compiled into Python package (wheels). The package is used in the Blender's Python Plugin.

If you want to use the Rust lib in another plugin, no problem: just update `lib.rs` or create a new lib file.

See :
[Ross Bencina about real time audio design](http://www.rossbencina.com/code/real-time-audio-programming-101-time-waits-for-nothing
) 
[Ross Bencina about non blocking software design architecture](http://www.rossbencina.com/code/lockfree)

## Thanks

Many many thanks to [NicoG60](https://github.com/NicoG60/TouchMCU/blob/main/doc/mackie_control_protocol.md) for the mackie control protocol doc.
