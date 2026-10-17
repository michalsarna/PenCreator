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
    /// A picture on the layer is drawn with lines (see `PictureMethod`).
    Picture,
}

impl Draw {
    pub const ALL: [Draw; 4] = [Draw::Outline, Draw::Fill, Draw::OutlineAndFill, Draw::Picture];

    pub fn label(self) -> &'static str {
        match self {
            Draw::Outline => "Outline",
            Draw::Fill => "Fill with lines",
            Draw::OutlineAndFill => "Fill and outline",
            Draw::Picture => "Picture",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Draw::Outline => "The pen follows the lines of the shapes. Open lines work too.",
            Draw::Fill => "Closed shapes are filled with parallel lines; holes stay empty.",
            Draw::OutlineAndFill => "The fill first, then the outline over it, so the edge is clean.",
            Draw::Picture => "A picture on the layer is drawn with lines: dithered rows, dots, or crossed hatching by tone.",
        }
    }

    pub fn fills(self) -> bool {
        matches!(self, Draw::Fill | Draw::OutlineAndFill)
    }

    pub fn outlines(self) -> bool {
        matches!(self, Draw::Outline | Draw::OutlineAndFill)
    }

    pub fn is_picture(self) -> bool {
        self == Draw::Picture
    }
}

/// How a picture is drawn with lines.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PictureMethod {
    /// The picture is dithered; every row is drawn where the picture is dark.
    #[default]
    Rows,
    /// The picture is dithered; every dark cell is a dot.
    Dots,
    /// Crossed hatching: the darker the picture, the more directions of lines.
    Tones,
}

impl PictureMethod {
    pub const ALL: [PictureMethod; 3] = [PictureMethod::Rows, PictureMethod::Dots, PictureMethod::Tones];

    pub fn label(self) -> &'static str {
        match self {
            PictureMethod::Rows => "Dithered rows",
            PictureMethod::Dots => "Dots",
            PictureMethod::Tones => "Hatching by tone",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            PictureMethod::Rows => "Each row is drawn as lines where the dithered picture is dark. The dithering method is chosen below.",
            PictureMethod::Dots => "A dot for every dark cell of the dithered picture.",
            PictureMethod::Tones => "Crossed hatching: the darker the picture, the more directions of lines.",
        }
    }

    /// Does the method dither the picture?
    pub fn dithers(self) -> bool {
        self != PictureMethod::Tones
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
    /// The pen of the library the layer draws with (its width is `pen_width`); `None` for a layer that only has a width and a
    /// colour, as in older projects.
    #[serde(default)]
    pub pen: Option<String>,
    /// A picture on the layer (`Draw::Picture`): how it is drawn, and with which dithering. The gap and the angle of the lines
    /// are the ones of the fill.
    #[serde(default)]
    pub picture: PictureMethod,
    #[serde(default)]
    pub dither: base_core::raster::Dither,
    /// Where the layer is in the order the layers are done in (the lower first; equal numbers go by layer). Moving layers
    /// in the list renumbers them all.
    #[serde(default)]
    pub rank: u32,
}


pub const MAX_PASSES: u8 = 5;

impl PenLayer {
    /// Draw with this pen: its width is the width of the line, and the gap of the fill and the pen are not changed.
    pub fn use_pen(&mut self, p: &crate::pens::Pen) {
        self.pen = Some(p.id.clone());
        self.pen_width = p.thickest().clamp(0.01, 20.0);
    }

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

/// The layers in the order they are done in: by their rank, then by their number.
pub fn layer_order(layers: &[PenLayer]) -> Vec<usize> {
    let mut o: Vec<usize> = (0..layers.len()).collect();
    o.sort_by_key(|&i| (layers[i].rank, i));
    o
}

/// Move a layer one place earlier (`up`) or later in the order the layers are done in; all the layers get a new rank.
pub fn move_layer(layers: &mut [PenLayer], layer: usize, up: bool) {
    let mut o = layer_order(layers);
    let Some(at) = o.iter().position(|&i| i == layer) else { return };
    let to = if up { at.checked_sub(1) } else { (at + 1 < o.len()).then_some(at + 1) };
    if let Some(to) = to {
        o.swap(at, to);
    }
    for (rank, &i) in o.iter().enumerate() {
        layers[i].rank = rank as u32;
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
                pen: None,
                picture: PictureMethod::default(),
                dither: base_core::raster::Dither::default(),
                rank: 0,
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

    #[test]
    fn layers_can_be_moved_in_the_order_they_are_done_in() {
        
        let mut l = default_layers();
        assert_eq!(&layer_order(&l)[..3], &[0, 1, 2]);
        move_layer(&mut l, 2, true);
        assert_eq!(&layer_order(&l)[..3], &[0, 2, 1]);
        move_layer(&mut l, 0, false);
        assert_eq!(&layer_order(&l)[..3], &[2, 0, 1]);
        move_layer(&mut l, 2, true);
        assert_eq!(layer_order(&l)[0], 2, "the first cannot go earlier");
        let last = layer_order(&l)[l.len() - 1];
        move_layer(&mut l, last, false);
        assert_eq!(layer_order(&l)[l.len() - 1], last, "the last cannot go later");
        // Older projects have no ranks: the order is the numbers.
        assert_eq!(layer_order(&default_layers()), (0..l.len()).collect::<Vec<_>>());
    }
}
