# LifeView

Lenia — continuous cellular automaton simulator built in Rust with the Bevy Engine.

<video src="videos/random-generation.mp4" controls width="100%"></video>

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

### Polynomial growth

Toggling **Polynomial growth** replaces the Gaussian with a compact quartic:

```
growth(u) = 2 · (1 - (|u - μ| / 3σ)²)⁴ - 1   if |u - μ| ≤ 3σ, else -1
```

The exponent α (default 4.0) controls the steepness of the falloff.

### Asymptotic (target mode)

In normal mode, the growth function pushes cells toward equilibrium independently of their current state. In **asymptotic (target) mode**, each cell drives toward the target value of the current kernel:

```
Δ = target(u) - current   where   target(u) = exp(-((u - μ)² / (2σ²)))
```

This creates an attractor dynamic — cells converge smoothly toward the kernel's response curve, producing self-stabilizing patterns that drift slowly rather than oscillating. Useful for gliders and slowly-evolving lifeforms.

### Multiple kernels and channels

Each kernel defines an independent convolution neighborhood with its own μ, σ, radius, height, and peak weights. Kernels can connect different channels (c₀ → c₁), enabling cross-channel interactions. Multiple kernels on the same channel combine via height-weighted sum, with a sum-mode option for additive mixing.

### Preset life-forms

| Shape                                   | Grid    | Kernels         | Channels | Parameters                                   |
| --------------------------------------- | ------- | --------------- | -------- | -------------------------------------------- |
| **Orbium bicaudatus**                   | 20×20   | 1               | 1        | R=13, μ=0.15, σ=0.015                        |
| **Hydrogeminium natans**                | 51×55   | 1               | 1        | R=18, μ=0.26, σ=0.036, peaks=[0.5, 1, 0.667] |
| **Fish** (multi-kernel)                 | 21×22   | 3               | 1        | R=10, μ=0.156/0.193/0.342, sum-mode          |
| **Tessellatium gyrans** (multi-channel) | 18×24×3 | 15              | 3        | R=12, cross-wired c₀→c₁→c₂                   |
| **Asymptotic Glider**                   | 17×17   | 1 (target mode) | 1        | R=13, μ=0.25, σ=0.1                          |

**Orbium bicaudatus**

<video src="videos/orbium.mp4" controls width="100%"></video>

**Hydrogeminium natans** — large radial seed with multi-peak kernel, produces slowly-rotating blobs.

**Fish** — three averaged kernels (sum mode, polynomial growth) for composite dynamics.

**Tessellatium gyrans** — 3-channel, 15-kernel configuration with cross-channel connections and polynomial growth. Each channel feeds into the others via weighted kernels.

**Asymptotic Glider** — uses target mode (attractor dynamic) instead of standard growth, producing a self-stabilizing glider.

### Creating custom shapes

Add shapes to `shapes.rs` using `Shape::from_grid(name, rule, &grid_data)` with a 2D slice of 0–1 values. Grids are imported from the [official Lenia Colab tutorial](https://colab.research.google.com/github/Chakazul/Lenia/blob/master/Lenia_colab.ipynb).

## Color maps

LifeView includes four scientific color maps applied to cell states:

| Map         | Tone                       | Best for                        |
| ----------- | -------------------------- | ------------------------------- |
| **Viridis** | purple → teal → yellow     | Perceptually uniform default    |
| **Plasma**  | dark blue → red → yellow   | High-contrast, vibrant patterns |
| **Inferno** | black → red → white        | Dark backgrounds, hotspots      |
| **Cividis** | dark teal → amber → yellow | Deutanopia/protanopia-safe      |

Switch between them in the Display panel. The **Smooth cells** toggle enables a CPU bicubic-interpolated heatmap for a fluid, smoke-like display.

<video src="videos/colormaps.mp4" controls width="100%"></video>

### Single-channel display

Cell values (0–1) are mapped through the selected gradient — low values map to one end, high values to the other.

### Multi-channel display

With 2–3 channels, each channel maps to its own color channel:

- **3 channels** — direct RGB mapping
- **2 channels** — first channel controls brightness, second controls hue intensity

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
