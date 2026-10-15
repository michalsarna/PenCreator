//! The layers of a plotting project. A layer is one pen: its colour and width, how fast it draws, and whether the shapes
//! on it are drawn as outlines, filled with hatching, or both.
use serde::{Deserialize, Serialize};

/// How many layers a project has.
/// How many layers there are, and their colours: the same in all programs (`base_core::layers`).
pub use base_core::layers::{LAYER_COUNT, PALETTE};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Draw {
    /// The pen follows the lines.
    Outline,
    /// The closed shapes are filled with parallel lines.
    Fill,
    /// The fill first, then the outline over it.
    OutlineAndFill,
}

impl Draw {
    pub const ALL: [Draw; 3] = [Draw::Outline, Draw::Fill, Draw::OutlineAndFill];

    pub fn label(self) -> &'static str {
        match self {
            Draw::Outline => "Outline",
            Draw::Fill => "Fill with lines",
            Draw::OutlineAndFill => "Fill and outline",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Draw::Outline => "The pen follows the lines of the shapes. Open lines work too.",
            Draw::Fill => "Closed shapes are filled with parallel lines; holes stay empty.",
            Draw::OutlineAndFill => "The fill first, then the outline over it, so the edge is clean.",
        }
    }

    pub fn fills(self) -> bool {
        self != Draw::Outline
    }

    pub fn outlines(self) -> bool {
        self != Draw::Fill
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PenLayer {
    pub name: String,
    pub color: [u8; 3],
    /// Shown in the work area.
    pub visible: bool,
    /// Taken into the plot and the G-code.
    pub enabled: bool,
    pub draw: Draw,
    /// The width of the line the pen makes (mm).
    pub pen_width: f64,
    /// The speed of the pen on the paper (mm/min).
    pub speed: f64,
    /// The angle of the fill lines (degrees).
    pub hatch_angle: f64,
    /// The distance between fill lines (mm).
    pub hatch_gap: f64,
    /// A second set of fill lines across the first.
    pub cross_hatch: bool,
    /// How many times every line is drawn (a pen that is too light).
    pub passes: u8,
}


pub const MAX_PASSES: u8 = 5;

impl PenLayer {
    pub fn color32(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.color[0], self.color[1], self.color[2])
    }

    /// Is every number of the layer usable?
    pub fn check(&self) -> Result<(), String> {
        let ok = |v: f64, lo: f64, hi: f64| v.is_finite() && v >= lo && v <= hi;
        if !ok(self.pen_width, 0.01, 20.0) || !ok(self.speed, 1.0, 100_000.0) || !ok(self.hatch_angle, -360.0, 360.0) || !ok(self.hatch_gap, 0.02, 100.0) || self.passes == 0 || self.passes > MAX_PASSES {
            return Err(format!("layer {}: a number is out of range", self.name));
        }
        if self.name.len() > 120 {
            return Err("a layer name is too long".into());
        }
        Ok(())
    }
}

/// The layers of a new project: four pens (black, red, blue, green) and a black fill, the others waiting.
pub fn default_layers() -> Vec<PenLayer> {
    (0..LAYER_COUNT)
        .map(|i| {
            let (name, draw) = match i {
                0 => ("Black pen", Draw::Outline),
                1 => ("Red pen", Draw::Outline),
                2 => ("Blue pen", Draw::Outline),
                3 => ("Green pen", Draw::Outline),
                4 => ("Black fill", Draw::Fill),
                _ => ("", Draw::Outline),
            };
            PenLayer {
                name: if name.is_empty() { format!("Layer {}", i + 1) } else { name.to_string() },
                color: if i == 4 { PALETTE[0] } else { PALETTE[i] },
                visible: true,
                enabled: true,
                draw,
                pen_width: 0.4,
                speed: 3000.0,
                hatch_angle: 45.0,
                hatch_gap: 0.4,
                cross_hatch: false,
                passes: 1,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_project_has_sixteen_valid_layers() {
        let l = default_layers();
        assert_eq!(l.len(), LAYER_COUNT);
        assert!(l.iter().all(|x| x.check().is_ok()));
        assert_eq!(l[4].draw, Draw::Fill);
        let colours: std::collections::HashSet<_> = l.iter().take(4).map(|x| x.color).collect();
        assert_eq!(colours.len(), 4);
    }

    #[test]
    fn wrong_numbers_are_refused() {
        let mut l = default_layers().remove(0);
        l.passes = 0;
        assert!(l.check().is_err());
        l.passes = 1;
        l.pen_width = f64::NAN;
        assert!(l.check().is_err());
        l.pen_width = 0.4;
        l.hatch_gap = 0.0;
        assert!(l.check().is_err());
    }
}
