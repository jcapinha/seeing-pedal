# seeing-pedal

Personal experiment: a stereo reverb + delay effect for a Daisy Seed 3, written in Rust. Effect topology is still open; this repo starts as a laptop-first workspace and ports to Daisy firmware once the shared engine is taking shape.

## Language

**Host**:
The thin wrapper that connects the engine to a specific environment's audio I/O. On the laptop there are two hosts (`host-wsl` under WSL, `host-windows` native on Windows) sharing common plumbing in `host-common`; on the Daisy, `host-daisy` uses the embedded audio drivers.
_Avoid_: runner, container

**Engine**:
The shared effect core (reverb + delay and their parameters). Same crate runs on laptop hosts and on the Daisy.
_Avoid_: DSP as a crate name

**Reverb**:
An effect that simulates space and reflections after the dry input. Algorithm, size, and modulation details are TBD.

**Delay**:
An effect that repeats the input after a time offset. Line length, feedback, and filtering are TBD.

**Wet / dry**:
Blend between the processed (wet) signal and the unprocessed (dry) input. Per-effect and master mix rules are TBD.

**Feedback**:
How much of the delay output is fed back into the delay line. Can build repeats or runaway if unbounded; limits are TBD.

**Topology**:
How reverb and delay are ordered and mixed (serial, parallel, insert vs send, one block vs two, etc.). TBD — several setups will be tried.

**Pedal**:
In this project, a standalone effect unit: audio in, audio out, controls, and firmware on the Seed. MIDI and UI details are TBD.

**Line level**:
The signal level the Seed 3 codec expects. Synth line outputs sit here. This is the input and output level the pedal is built for.

**Instrument level**:
The weaker, high-impedance level of an electric guitar. Guitar pedals boost this before their circuit. For now, this pedal does not. In the future, a circuit to address this might be built.

## Decisions

**Rust as the implementation language**
The pedal is implemented in Rust. Same language on laptop hosts and Daisy firmware.

**Daisy Seed 3 as the hardware target**
The pedal targets a [Daisy Seed 3](https://docs.daisy.audio/hardware/Seed3/) (Seed3). Firmware will live in `host-daisy` when bring-up starts. Board docs: [Daisy documentation](https://docs.daisy.audio/). The codec (TAC5242) is stereo line in and line out.

**Line-level stereo synths as the sources**
Inputs are the Polyend Play line out and the Roland S-1 headphone out. The Play is already line level. The S-1 headphone out is hotter than line. Its volume knob is the input trim, and it must stay below the point where the codec clips. The signal path is stereo. The analog front end is unity gain, with no instrument-level boost for the time being.

**Laptop-first, layered architecture**
A Cargo workspace with a shared `engine` crate, shared laptop plumbing in `host-common`, thin binaries `host-wsl` and `host-windows`, then port to the Daisy last (`host-daisy`). The engine is the reusable core; hosts are swappable I/O and control plumbing.

**Reverb + delay as the effect scope**
The pedal provides reverb and delay. Signal topology (order, routing, wet/dry) is TBD and will be explored on the laptop before locking firmware layout. One reverb algorithm runs at a time. Switching algorithms at runtime is in scope. Which algorithms exist is still TBD. Several full reverbs do not run together.

**DaisyCloudSeed is a reference, not the codebase**
[GuitarML/DaisyCloudSeed](https://github.com/GuitarML/DaisyCloudSeed) shows the shape: an audio block in, an effect, a block out, knobs as 0 to 1 parameters, a bypass footswitch. It is C++ for the PedalPCB Terrarium (a guitar pedal board), one reverb algorithm with presets, and a separate CloudyReverb firmware. Do not port it or flash its binary. The Terrarium pinout does not apply.
