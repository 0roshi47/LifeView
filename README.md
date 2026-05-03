# LifeView

Lenia continuous cellular automaton simulator built in Rust with the Bevy Engine.

https://github.com/user-attachments/videos/random-generation.mov

## Rule System

LifeView uses a continuous growth function to evolve cell states each generation:

- **μ (micro)** — growth peak. Neighborhood value at which growth is maximal. Default: `0.15`
- **σ (sigma)** — growth width. Controls how narrowly the growth curve peaks around μ. Default: `0.015`
- **radius** — neighborhood radius (in cells). Larger = more cells influence each update. Default: `13`
- **Δt (delta)** — time step per generation. Controls simulation speed. Default: `0.1`

### Growth function

```
growth(u) = 2 · exp(-((u - μ)² / (2σ²))) - 1
```

Where `u` is the weighted neighborhood average using a bell kernel:

```
w(d) = exp(-((d/r - 0.5) / 0.15)² / 2)
```

Each cell updates: `new_state = clamp(old_state + growth(u) · Δt, 0, 1)`

### Preset life-forms

- **Orbium bicaudatus** — classic 20×20 seed from official Lenia tutorial (R=13, μ=0.15, σ=0.015)

https://github.com/user-attachments/videos/orbium.mov

- **Aquarium** — ring seed with different parameters (R=10, μ=0.278, σ=0.036)

## How to Run

Clone the repository:

```bash
git clone https://github.com/0roshi47/LifeView.git
```

### Native

```bash
cargo run --release
```

### Web (WASM)

```bash
# Install wasm-server-runner if not already installed
cargo install wasm-server-runner

# Build and run in browser
cargo run --target wasm32-unknown-unknown --release
```

Requires Rust 2024 edition toolchain.
