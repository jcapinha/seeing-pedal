# seeing-pedal

A reverb + delay effect pedal for the Daisy Seed 3, written in Rust. Scaffolding only for now — no DSP or firmware logic yet.

- **[CONTEXT.md](CONTEXT.md)** — goal, pedal glossary, and recorded decisions
- **[REJECTED.md](REJECTED.md)** — closed doors (empty for now)

## Workspace

| Crate | Role |
| --- | --- |
| [`engine/`](engine) | Shared effect core (reverb + delay); `no_std`-friendly from the start |
| [`host-common/`](host-common) | Shared laptop audio and control plumbing |
| [`host-wsl/`](host-wsl) | WSL/Linux dev host |
| [`host-windows/`](host-windows) | Native Windows host |
| [`host-daisy/`](host-daisy) | Seed 3 firmware (stub until bring-up) |

## Build

Install [Rust](https://rustup.rs), then from the repo root:

### WSL

```bash
cargo check
cargo run -p host-wsl
```

### Windows (PowerShell)

Use the MSVC toolchain (`x86_64-pc-windows-msvc`). Open the repo (including from `\\wsl$\<Distro>\...` if the tree lives under WSL), then:

```powershell
cargo check
cargo run -p host-windows
```

Hosts currently print a stub message and exit. Effect topology and hardware bring-up come later — see `CONTEXT.md`.
