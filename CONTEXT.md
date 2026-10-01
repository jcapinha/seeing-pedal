# seeing-pedal

Personal experiment: a reverb + delay effect pedal built on a Daisy Seed 3, written in Rust. The signal path and effect topology are still open; this repo starts as a laptop-first workspace and ports to Daisy firmware once the shared engine is taking shape.

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

## Decisions

**Rust as the implementation language**
The pedal is implemented in Rust. Same language on laptop hosts and Daisy firmware.

**Daisy Seed 3 as the hardware target**
The pedal targets a [Daisy Seed 3](https://docs.daisy.audio/hardware/Seed3/) (Seed3). Firmware will live in `host-daisy` when bring-up starts. Board docs: [Daisy documentation](https://docs.daisy.audio/).

**Laptop-first, layered architecture**
A Cargo workspace with a shared `engine` crate, shared laptop plumbing in `host-common`, thin binaries `host-wsl` and `host-windows`, then port to the Daisy last (`host-daisy`). The engine is the reusable core; hosts are swappable I/O and control plumbing.

**Reverb + delay as the effect scope**
The pedal provides reverb and delay. Signal topology (order, routing, wet/dry) is TBD and will be explored on the laptop before locking firmware layout.
