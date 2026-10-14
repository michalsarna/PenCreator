//! The part of the project file that belongs to PenCreator: the pens (layers) and a snapshot of the plotter. Other
//! programs keep it untouched, and PenCreator keeps theirs.
use crate::device::Device;
use crate::layers::{default_layers, PenLayer, LAYER_COUNT};
use serde::{Deserialize, Serialize};

pub const SECTION: &str = "pen";
/// The version of this part. A project with a higher one is refused.
pub const SECTION_FORMAT: u32 = 1;

/// The plotter the project was made for: enough to warn when it is opened for another one.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Machine {
    pub name: String,
    pub x: f64,
    pub y: f64,
}

impl Machine {
    pub fn of(d: &Device) -> Machine {
        Machine { name: d.name.clone(), x: d.area.x, y: d.area.y }
    }

    /// What the project was made for, in words; `None` when the work area is the same.
    pub fn differences(&self, d: &Device) -> Option<String> {
        let same = |a: f64, b: f64| (a - b).abs() < 0.5;
        if same(self.x, d.area.x) && same(self.y, d.area.y) {
            None
        } else {
            Some(format!("{} ({} × {} mm)", self.name, self.x, self.y))
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PenSection {
    pub format: u32,
    pub layers: Vec<PenLayer>,
    pub machine: Machine,
}

impl PenSection {
    pub fn new(layers: &[PenLayer], device: &Device) -> PenSection {
        PenSection { format: SECTION_FORMAT, layers: layers.to_vec(), machine: Machine::of(device) }
    }
}

/// What a project brings in.
pub struct Loaded {
    pub layers: Vec<PenLayer>,
    pub machine: Option<Machine>,
}

/// Read the section of a project. A project without one (a drawing, or a project of another program) gets the default
/// layers; a layer with wrong numbers is replaced by the default one.
pub fn load(p: &base_core::project::Project) -> Result<Loaded, String> {
    let Some(s) = p.section::<PenSection>(SECTION)? else {
        return Ok(Loaded { layers: default_layers(), machine: None });
    };
    if s.format > SECTION_FORMAT {
        return Err(format!("the plotting part of the project has format {}, this program reads up to {SECTION_FORMAT}; update the program", s.format));
    }
    let defaults = default_layers();
    let mut layers: Vec<PenLayer> = s.layers.into_iter().take(LAYER_COUNT).collect();
    for (i, l) in layers.iter_mut().enumerate() {
        if l.check().is_err() {
            *l = defaults[i].clone();
        }
    }
    while layers.len() < LAYER_COUNT {
        layers.push(defaults[layers.len()].clone());
    }
    Ok(Loaded { layers, machine: Some(s.machine) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use base_core::project::Project;
    use base_core::scene::Scene;

    #[test]
    fn the_section_goes_into_the_project_and_comes_back() {
        let mut layers = default_layers();
        layers[1].pen_width = 0.8;
        layers[1].name = "Red marker".into();
        let mut p = Project::new(Scene::default());
        p.set_section(SECTION, &PenSection::new(&layers, &Device::dummy()));
        let back = load(&Project::from_json(&p.to_json()).unwrap()).unwrap();
        assert_eq!((back.layers[1].pen_width, back.layers[1].name.as_str()), (0.8, "Red marker"));
        assert_eq!(back.machine.unwrap().name, "Dummy");
    }

    #[test]
    fn a_project_without_the_section_gets_the_default_layers_and_a_newer_one_is_refused() {
        let l = load(&Project::new(Scene::default())).unwrap();
        assert_eq!(l.layers.len(), LAYER_COUNT);
        assert!(l.machine.is_none());
        let mut newer = Project::new(Scene::default());
        newer.set_section(SECTION, &serde_json::json!({ "format": 99, "layers": [], "machine": { "name": "x", "x": 1, "y": 1 } }));
        assert!(load(&newer).is_err());
    }

    #[test]
    fn a_wrong_layer_is_replaced_and_missing_ones_added() {
        let mut layers = default_layers();
        layers.truncate(3);
        layers[1].passes = 0;
        let mut p = Project::new(Scene::default());
        p.set_section(SECTION, &PenSection::new(&layers, &Device::dummy()));
        let l = load(&Project::from_json(&p.to_json()).unwrap()).unwrap();
        assert_eq!(l.layers.len(), LAYER_COUNT);
        assert!(l.layers.iter().all(|x| x.check().is_ok()));
    }

    #[test]
    fn the_project_of_another_plotter_is_noticed_and_other_sections_are_kept() {
        let m = Machine::of(&Device::dummy());
        assert!(m.differences(&Device::dummy()).is_none());
        let mut d = Device::dummy();
        d.area.x = 600.0;
        assert!(m.differences(&d).unwrap().starts_with("Dummy"));
        let mut p = Project::new(Scene::default());
        p.set_section("laser", &serde_json::json!({ "format": 1 }));
        let kept = p.other_sections(SECTION);
        let mut out = Project::new(Scene::default());
        out.set_section(SECTION, &PenSection::new(&default_layers(), &Device::dummy()));
        out.restore_sections(&kept);
        let mut names = out.section_names();
        names.sort();
        assert_eq!(names, vec!["laser", "pen"]);
    }
}
