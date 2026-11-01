//! The pen settings of a layer (the way of drawing, the pen, the width, the speed, the fill, the passes, the picture method) saved
//! under a name and put on any layer later. The list, the window and the file are in `base_ui::layer_presets_ui`; this says what a
//! preset holds. The file is in the shared data folder.
use crate::layers::PenLayer;

/// The file in the shared data folder.
pub const FILE: &str = "pen_layer_presets.json";

/// What of a layer a preset keeps: everything but the name, the colour, the visibility, whether it is taken into the plot, and the
/// place in the order.
pub fn of(l: &PenLayer) -> PenLayer {
    l.clone()
}

/// A preset whose numbers are out of range is left out of the list.
pub fn valid(l: &PenLayer) -> bool {
    l.check().is_ok()
}

/// Put the settings on a layer; its name, colour, visibility, whether it is taken into the plot and its place in the order stay. A pen
/// that is not in the library any more is left off the layer (its width stays); the answer says so.
pub fn apply(preset: &PenLayer, to: &mut PenLayer, pen_exists: &dyn Fn(&str) -> bool) -> bool {
    let keep = (to.name.clone(), to.color, to.visible, to.enabled, to.rank);
    *to = preset.clone();
    (to.name, to.color, to.visible, to.enabled, to.rank) = keep;
    match to.pen.as_deref() {
        Some(id) if !pen_exists(id) => {
            to.pen = None;
            false
        }
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layers::{default_layers, Draw};
    use crate::pens::PenLibrary;

    #[test]
    fn a_preset_carries_the_pen_settings_and_leaves_the_layer_its_name_and_its_order() {
        let mut layers = default_layers();
        layers[0].draw = Draw::Fill;
        layers[0].pen_width = 0.7;
        layers[0].speed = 1500.0;
        layers[0].passes = 2;
        let p = of(&layers[0]);
        let mut other = layers[1].clone();
        other.name = "Mine".into();
        other.color = [1, 2, 3];
        other.rank = 9;
        other.visible = false;
        assert!(apply(&p, &mut other, &|_| true));
        assert_eq!((other.draw, other.pen_width, other.speed, other.passes), (Draw::Fill, 0.7, 1500.0, 2));
        assert_eq!((other.name.as_str(), other.color, other.rank, other.visible), ("Mine", [1, 2, 3], 9, false));
    }

    #[test]
    fn a_preset_with_a_pen_that_is_not_in_the_library_leaves_the_layer_without_a_pen() {
        let lib = PenLibrary::new();
        let layers = default_layers();
        let mut p = of(&layers[0]);
        p.pen = Some("gone-pen".into());
        let mut other = layers[1].clone();
        assert!(!apply(&p, &mut other, &|id| lib.get(id).is_some()));
        assert!(other.pen.is_none());
        let mut bad = of(&layers[0]);
        bad.pen_width = -1.0;
        assert!(valid(&of(&layers[0])) && !valid(&bad));
    }
}
