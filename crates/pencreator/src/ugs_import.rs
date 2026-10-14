//! The layers for a design of UGS Designer: one for each way the shapes are cut. The pen follows the outlines, or fills
//! the shapes the Designer pockets or fills.
use crate::layers::{default_layers, Draw, PenLayer};
use base_core::ugsd::{Cut, CutType, Import};

fn name(c: CutType) -> &'static str {
    match c {
        CutType::OutsidePath => "Outside",
        CutType::InsidePath => "Inside",
        CutType::Pocket => "Pocket",
        CutType::OnPath | CutType::LaserOnPath | CutType::PlotterOnPath => "On the line",
        CutType::LaserFill | CutType::PlotterFill => "Fill",
        CutType::None => "Not cut",
        _ => "Other",
    }
}

pub fn layers(imp: &Import) -> Vec<PenLayer> {
    let mut layers = default_layers();
    for c in &imp.cuts {
        if let Some(slot) = layers.get_mut(c.index) {
            *slot = layer_for(c, slot.clone());
        }
    }
    layers
}

fn layer_for(c: &Cut, mut l: PenLayer) -> PenLayer {
    l.draw = if c.cut.is_fill() { Draw::Fill } else { Draw::Outline };
    l.name = format!("{} {} mm", name(c.cut), if (c.cut_depth - c.cut_depth.round()).abs() < 1e-9 { format!("{}", c.cut_depth as i64) } else { format!("{:.1}", c.cut_depth) });
    l.enabled = c.cut != CutType::None;
    if c.feed > 0.0 {
        l.speed = c.feed;
    }
    if l.check().is_err() {
        return PenLayer { speed: 3000.0, ..l };
    }
    l
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pockets_are_filled_and_the_rest_is_outlined() {
        let d = r#"{"version":"1","entities":[
          {"type":"RECTANGLE","cutType":"OUTSIDE_PATH","cutDepth":10,"feedRate":600,"transform":{"m00":20,"m10":0,"m01":0,"m11":10,"m02":0,"m12":0}},
          {"type":"RECTANGLE","cutType":"POCKET","cutDepth":3,"feedRate":900,"transform":{"m00":5,"m10":0,"m01":0,"m11":5,"m02":2,"m12":2}}]}"#;
        let imp = base_core::ugsd::import(d, 100.0, 16).unwrap();
        let l = layers(&imp);
        assert_eq!((l[0].draw, l[0].speed), (Draw::Outline, 600.0));
        assert_eq!((l[1].draw, l[1].speed), (Draw::Fill, 900.0));
        assert!(l.iter().all(|x| x.check().is_ok()));
    }
}
