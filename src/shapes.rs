use bevy::{
    ecs::system::{Commands, ResMut},
    math::IVec2,
};

use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct Shape {
    pub name: String,
    pub optimal_rule: Rule,
    pub cells_state: Vec<f32>,
    pub cells_pos: Vec<IVec2>,
}

impl Shape {
    pub fn new(
        name: String,
        optimal_rule: Rule,
        cells_state: Vec<f32>,
        cells_pos: Vec<IVec2>,
    ) -> Self {
        Self {
            name,
            optimal_rule,
            cells_state,
            cells_pos,
        }
    }

    /// Build from a 2D row-major grid of f32 values, top-left origin.
    /// Zero cells are skipped (not spawned).
    pub fn from_grid(name: impl Into<String>, optimal_rule: Rule, grid: &[&[f32]]) -> Self {
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;
        let origin = IVec2::new(cols / 2, rows / 2);
        let mut cells_state = Vec::new();
        let mut cells_pos = Vec::new();
        for (y, row) in grid.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                if val > 0.0 {
                    cells_state.push(val);
                    // flip y so row 0 = top of pattern = positive y in grid
                    cells_pos.push(IVec2::new(x as i32, rows - 1 - y as i32) - origin);
                }
            }
        }
        Self::new(name.into(), optimal_rule, cells_state, cells_pos)
    }

    pub fn disc(
        name: impl Into<String>,
        optimal_rule: Rule,
        r: i32,
        state_fn: impl Fn(f32) -> f32,
    ) -> Self {
        let mut cells_state = Vec::new();
        let mut cells_pos = Vec::new();
        for x in -r..=r {
            for y in -r..=r {
                let dist = ((x * x + y * y) as f32).sqrt();
                if dist <= r as f32 {
                    let t = dist / r as f32;
                    cells_state.push(state_fn(t).clamp(0.0, 1.0));
                    cells_pos.push(IVec2::new(x, y));
                }
            }
        }
        Self::new(name.into(), optimal_rule, cells_state, cells_pos)
    }

    pub fn ring(
        name: impl Into<String>,
        optimal_rule: Rule,
        r_inner: i32,
        r_outer: i32,
        state_fn: impl Fn(f32) -> f32,
    ) -> Self {
        let mut cells_state = Vec::new();
        let mut cells_pos = Vec::new();
        for x in -r_outer..=r_outer {
            for y in -r_outer..=r_outer {
                let dist = ((x * x + y * y) as f32).sqrt();
                if dist >= r_inner as f32 && dist <= r_outer as f32 {
                    let t = (dist - r_inner as f32) / (r_outer - r_inner).max(1) as f32;
                    cells_state.push(state_fn(t).clamp(0.0, 1.0));
                    cells_pos.push(IVec2::new(x, y));
                }
            }
        }
        Self::new(name.into(), optimal_rule, cells_state, cells_pos)
    }

    /// Decode Chakazul's (zip) compressed cell format used in Lenia-LifeForms.js.
    ///
    /// Format: `"label1.data1/label2.data2/..."`
    /// - Segments separated by `/`
    /// - Label char before `.` encodes the row's y-ordinal position
    /// - `0` = empty cell (0.0)
    /// - `×` (U+00D7) = horizontal gap/padding between cell clusters in same row
    /// - Any other char c maps to value = clamp((ord(c) - 128) / 127.0)
    ///
    /// Multiple segments at the same y-ordinal combine into a single row.
    /// The resulting grid is centered and zero-padded to rectangular form.
    pub fn from_compressed(name: impl Into<String>, optimal_rule: Rule, data: &str) -> Self {
        let rows = decode_compressed(data);
        let grid_refs: Vec<&[f32]> = rows.iter().map(|r| r.as_slice()).collect();
        Self::from_grid(name, optimal_rule, &grid_refs)
    }

    /// Orbium bicaudatus seed from the official Lenia Colab tutorial
    /// (OpenLenia/Lenia-Tutorial: Tutorial_From_Conway_to_Lenia).
    /// Parameters: R=13, mu=0.15, sigma=0.015, T=10
    pub fn orbium_bicaudatus(name: impl Into<String>) -> Self {
        let rule = Rule::single_channel(0.15, 0.015, 13);
        Self::from_grid(name, rule, ORBIUM_GRID)
    }
}

/// Decode Chakazul's (zip) compressed cell format.
///
/// Returns a `Vec<&[f32]>` where each inner slice is one row of the grid.
/// The grid is padded to rectangular form and centered.
fn decode_compressed(data: &str) -> Vec<Vec<f32>> {
    // Parse segments: "label.data" groups separated by '/'
    // Collect (y_ordinal, data_string) pairs
    #[derive(Debug)]
    struct Segment {
        y_ord: u16,
        data: String,
    }

    let mut segments: Vec<Segment> = Vec::new();
    let mut last_y_ord: Option<u16> = None;

    for seg_str in data.split('/') {
        if seg_str.is_empty() {
            continue;
        }
        if let Some(dot_pos) = seg_str.find('.') {
            let label = &seg_str[..dot_pos];
            let row_data = &seg_str[dot_pos + 1..];
            let y_ord = if label.is_empty() {
                last_y_ord.unwrap_or(0)
            } else {
                label.chars().next().map(|c| c as u16).unwrap_or(0)
            };
            last_y_ord = Some(y_ord);
            segments.push(Segment {
                y_ord,
                data: row_data.to_string(),
            });
        } else {
            // No dot: continuation of last label
            let y_ord = last_y_ord.unwrap_or(0);
            segments.push(Segment {
                y_ord,
                data: seg_str.to_string(),
            });
        }
    }

    // Group segments by y_ord and merge them into rows
    // Segments at the same y combine left-to-right (as they appear in the string)
    use std::collections::BTreeMap;
    let mut rows_map: BTreeMap<u16, Vec<String>> = BTreeMap::new();
    for seg in segments {
        rows_map.entry(seg.y_ord).or_default().push(seg.data);
    }

    // For each y, merge segments into a single row string
    // The '×' character acts as a spacer between segments
    let mut raw_rows: Vec<(u16, Vec<f32>)> = Vec::new();
    for (y_ord, segs) in &rows_map {
        let mut row_vals: Vec<f32> = Vec::new();
        for (si, seg_data) in segs.iter().enumerate() {
            if si > 0 {
                // Insert a gap between segments
                row_vals.push(0.0);
            }
            for c in seg_data.chars() {
                let val = decode_cell_char(c);
                row_vals.push(val);
            }
        }
        raw_rows.push((*y_ord, row_vals));
    }

    // Sort by y_ord
    raw_rows.sort_by_key(|(y, _)| *y);

    // Find min/max y to determine vertical span
    let min_y = raw_rows.first().map(|(y, _)| *y).unwrap_or(0);
    let max_y = raw_rows.last().map(|(y, _)| *y).unwrap_or(0);

    // Determine horizontal bounds: find the widest row and center everything
    let max_width = raw_rows.iter().map(|(_, r)| r.len()).max().unwrap_or(0);

    // Build the final rectangular grid
    let num_rows = (max_y - min_y + 1) as usize;
    let grid_width = max_width;

    let mut grid = vec![vec![0.0f32; grid_width]; num_rows];

    for (y_ord, row_vals) in &raw_rows {
        let row_idx = (*y_ord - min_y) as usize;
        if row_idx >= num_rows {
            continue;
        }
        let pad_left = (grid_width - row_vals.len()) / 2;
        for (i, &val) in row_vals.iter().enumerate() {
            if pad_left + i < grid_width {
                grid[row_idx][pad_left + i] = val;
            }
        }
    }

    grid
}

/// Decode a single character from the compressed format into a cell state value.
fn decode_cell_char(c: char) -> f32 {
    match c {
        '0' | '.' => 0.0,
        _ => {
            let code = c as u32;
            // Encoding: value = (code_point - 128) / 127, clamped to [0, 1]
            // This maps Latin-1 supplement (0x80-0xFF) roughly to [0, 1]
            // Extended Unicode chars above 0xFF are clamped to 1.0
            let val = (code as f32 - 128.0) / 127.0;
            val.clamp(0.0, 1.0)
        }
    }
}

/// Exact 20×20 Orbium bicaudatus grid from the official Lenia Colab tutorial
/// (OpenLenia/Lenia-Tutorial). Parameters: R=13, mu=0.15, sigma=0.015.
const ORBIUM_GRID: &[&[f32]] = &[
    &[0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.10, 0.14, 0.10, 0.00, 0.00, 0.03, 0.03, 0.00, 0.00, 0.30, 0.00, 0.00, 0.00, 0.00],
    &[0.00, 0.00, 0.00, 0.00, 0.00, 0.08, 0.24, 0.30, 0.30, 0.18, 0.14, 0.15, 0.16, 0.15, 0.09, 0.20, 0.00, 0.00, 0.00, 0.00],
    &[0.00, 0.00, 0.00, 0.00, 0.00, 0.15, 0.34, 0.44, 0.46, 0.38, 0.18, 0.14, 0.11, 0.13, 0.19, 0.18, 0.45, 0.00, 0.00, 0.00],
    &[0.00, 0.00, 0.00, 0.00, 0.06, 0.13, 0.39, 0.50, 0.50, 0.37, 0.06, 0.00, 0.00, 0.00, 0.02, 0.16, 0.68, 0.00, 0.00, 0.00],
    &[0.00, 0.00, 0.00, 0.11, 0.17, 0.17, 0.33, 0.40, 0.38, 0.28, 0.14, 0.00, 0.00, 0.00, 0.00, 0.00, 0.18, 0.42, 0.00, 0.00],
    &[0.00, 0.00, 0.09, 0.18, 0.13, 0.06, 0.08, 0.26, 0.32, 0.32, 0.27, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.82, 0.00, 0.00],
    &[0.27, 0.00, 0.16, 0.12, 0.00, 0.00, 0.00, 0.25, 0.38, 0.44, 0.45, 0.34, 0.00, 0.00, 0.00, 0.00, 0.00, 0.22, 0.17, 0.00],
    &[0.00, 0.07, 0.20, 0.02, 0.00, 0.00, 0.00, 0.31, 0.48, 0.57, 0.60, 0.57, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.49, 0.00],
    &[0.00, 0.59, 0.19, 0.00, 0.00, 0.00, 0.00, 0.20, 0.57, 0.69, 0.76, 0.76, 0.49, 0.00, 0.00, 0.00, 0.00, 0.00, 0.36, 0.00],
    &[0.00, 0.58, 0.19, 0.00, 0.00, 0.00, 0.00, 0.00, 0.67, 0.83, 0.90, 0.92, 0.87, 0.12, 0.00, 0.00, 0.00, 0.00, 0.22, 0.07],
    &[0.00, 0.00, 0.46, 0.00, 0.00, 0.00, 0.00, 0.00, 0.70, 0.93, 1.00, 1.00, 1.00, 0.61, 0.00, 0.00, 0.00, 0.00, 0.18, 0.11],
    &[0.00, 0.00, 0.82, 0.00, 0.00, 0.00, 0.00, 0.00, 0.47, 1.00, 1.00, 0.98, 1.00, 0.96, 0.27, 0.00, 0.00, 0.00, 0.19, 0.10],
    &[0.00, 0.00, 0.46, 0.00, 0.00, 0.00, 0.00, 0.00, 0.25, 1.00, 1.00, 0.84, 0.92, 0.97, 0.54, 0.14, 0.04, 0.10, 0.21, 0.05],
    &[0.00, 0.00, 0.00, 0.40, 0.00, 0.00, 0.00, 0.00, 0.09, 0.80, 1.00, 0.82, 0.80, 0.85, 0.63, 0.31, 0.18, 0.19, 0.20, 0.01],
    &[0.00, 0.00, 0.00, 0.36, 0.10, 0.00, 0.00, 0.00, 0.05, 0.54, 0.86, 0.79, 0.74, 0.72, 0.60, 0.39, 0.28, 0.24, 0.13, 0.00],
    &[0.00, 0.00, 0.00, 0.01, 0.30, 0.07, 0.00, 0.00, 0.08, 0.36, 0.64, 0.70, 0.64, 0.60, 0.51, 0.39, 0.29, 0.19, 0.04, 0.00],
    &[0.00, 0.00, 0.00, 0.00, 0.10, 0.24, 0.14, 0.10, 0.15, 0.29, 0.45, 0.53, 0.52, 0.46, 0.40, 0.31, 0.21, 0.08, 0.00, 0.00],
    &[0.00, 0.00, 0.00, 0.00, 0.00, 0.08, 0.21, 0.21, 0.22, 0.29, 0.36, 0.39, 0.37, 0.33, 0.26, 0.18, 0.09, 0.00, 0.00, 0.00],
    &[0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.03, 0.13, 0.19, 0.22, 0.24, 0.24, 0.23, 0.18, 0.13, 0.05, 0.00, 0.00, 0.00, 0.00],
    &[0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.02, 0.06, 0.08, 0.09, 0.07, 0.05, 0.01, 0.00, 0.00, 0.00, 0.00, 0.00],
];

#[derive(bevy::prelude::Resource, Debug, Default)]
pub struct Shapes(pub Vec<Shape>);

impl Shapes {
    pub fn add(&mut self, shape: Shape) {
        self.0.push(shape);
    }
}

pub fn insert_shapes(mut commands: Commands) {
    commands.insert_resource(Shapes(Vec::new()));
}

pub fn add_shapes(mut shapes: ResMut<Shapes>) {
    // Orbium bicaudatus — exact 20×20 grid from the official Lenia Colab tutorial
    // (R=13, mu=0.15, sigma=0.015)
    let orbium = Shape::orbium_bicaudatus("Orbium");
    shapes.add(orbium);

    // Aquarium — ring seed, different parameters
    let aquarium = Shape::ring(
        "Aquarium",
        Rule::single_channel(0.278, 0.036, 10),
        2,
        7,
        |t| (1.0 - t).powi(2),
    );
    shapes.add(aquarium);
}
