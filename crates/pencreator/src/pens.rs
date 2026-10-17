//! The pens: a kind of pen with the size of the mark it leaves (width and height of the dot, so a chisel marker draws a thick
//! line one way and a thin one the other), its ink and how see-through it is. A layer draws with one pen; the preview shows the
//! lines the way the pen makes them. The library has ready-made pens and the person's own.
use serde::{Deserialize, Serialize};

pub const MAX_PENS: usize = 200;
pub const MAX_FILE: u64 = 400_000;
const FILE_FORMAT: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PenKind {
    Pencil,
    Fountain,
    Fineliner,
    Marker,
    Highlighter,
    Brush,
}

impl PenKind {
    pub const ALL: [PenKind; 6] = [PenKind::Pencil, PenKind::Fountain, PenKind::Fineliner, PenKind::Marker, PenKind::Highlighter, PenKind::Brush];

    pub fn label(self) -> &'static str {
        match self {
            PenKind::Pencil => "Pencil",
            PenKind::Fountain => "Fountain pen (nib)",
            PenKind::Fineliner => "Fineliner",
            PenKind::Marker => "Felt-tip marker",
            PenKind::Highlighter => "Highlighter",
            PenKind::Brush => "Brush pen",
        }
    }
}

/// The shape of the ends of a line.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cap {
    /// A round tip: the line ends in a half circle, a dot is round.
    #[default]
    Round,
    /// A flat tip: the line ends square (a chisel or a nib).
    Flat,
}

impl Cap {
    pub const ALL: [Cap; 2] = [Cap::Round, Cap::Flat];

    pub fn label(self) -> &'static str {
        match self {
            Cap::Round => "Round ends",
            Cap::Flat => "Flat ends",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pen {
    /// Identifies the pen in the projects; stays the same when the pen is renamed.
    pub id: String,
    pub name: String,
    pub kind: PenKind,
    /// The width of the mark (mm): across the tip.
    pub width: f64,
    /// The height of the mark (mm): along the tip. The same as the width for a round tip.
    pub height: f64,
    /// The turn of the tip (degrees): the direction of the width of a chisel or a nib.
    #[serde(default)]
    pub angle: f64,
    /// The colour of the ink.
    pub color: [u8; 3],
    /// How much of the paper is covered: 1 is solid ink, less is see-through (a highlighter), overlaps get darker.
    #[serde(default = "full")]
    pub opacity: f64,
    #[serde(default)]
    pub cap: Cap,
    #[serde(default)]
    pub notes: String,
    /// A ready-made pen: it cannot be changed or deleted. Never written to a file.
    #[serde(skip)]
    pub builtin: bool,
}

fn full() -> f64 {
    1.0
}

impl Pen {
    /// How thick a line is that goes in the direction `dir` (radians, in the work area): the width of the mark across the line.
    /// A round tip is as thick as it is wide; a chisel is thick when the line is across its width and as thin as its height when
    /// the line goes along it.
    pub fn thickness(&self, dir: f64) -> f64 {
        if (self.width - self.height).abs() < 1e-9 {
            return self.width;
        }
        // The mark is an ellipse (width x height) turned by `angle`; the thickness of a line is its extent across the line.
        let phi = dir - self.angle.to_radians();
        let (a, b) = (self.width / 2.0, self.height / 2.0);
        2.0 * ((a * phi.sin()).powi(2) + (b * phi.cos()).powi(2)).sqrt()
    }

    /// The thickest and the thinnest line this pen draws.
    pub fn thickest(&self) -> f64 {
        self.width.max(self.height)
    }

    pub fn thinnest(&self) -> f64 {
        self.width.min(self.height)
    }

    /// Is every number of the pen usable?
    pub fn check(&self) -> Result<(), String> {
        let ok = |v: f64, lo: f64, hi: f64| v.is_finite() && v >= lo && v <= hi;
        if self.id.trim().is_empty() || self.id.len() > 64 || self.name.trim().is_empty() || self.name.chars().count() > 100 || self.notes.chars().count() > 2000 {
            return Err("the name or the id is empty or too long".into());
        }
        if !ok(self.width, 0.01, 50.0) || !ok(self.height, 0.01, 50.0) || !ok(self.angle, -360.0, 360.0) || !ok(self.opacity, 0.05, 1.0) {
            return Err("a number of the pen is out of range".into());
        }
        Ok(())
    }

    pub fn color32(&self) -> egui::Color32 {
        egui::Color32::from_rgba_unmultiplied(self.color[0], self.color[1], self.color[2], (self.opacity * 255.0).round() as u8)
    }
}

fn pen(id: &str, name: &str, kind: PenKind, width: f64, height: f64, angle: f64, color: [u8; 3], opacity: f64, cap: Cap) -> Pen {
    Pen { id: id.into(), name: name.into(), kind, width, height, angle, color, opacity, cap, notes: String::new(), builtin: true }
}

/// The ready-made pens.
pub fn builtin_pens() -> Vec<Pen> {
    let black = [0x10, 0x10, 0x10];
    vec![
        pen("std-fine-02", "Fineliner 0.2 mm", PenKind::Fineliner, 0.2, 0.2, 0.0, black, 1.0, Cap::Round),
        pen("std-fine-04", "Fineliner 0.4 mm", PenKind::Fineliner, 0.4, 0.4, 0.0, black, 1.0, Cap::Round),
        pen("std-fine-08", "Fineliner 0.8 mm", PenKind::Fineliner, 0.8, 0.8, 0.0, [0x20, 0x20, 0x90], 1.0, Cap::Round),
        pen("std-pencil-05", "Pencil 0.5 mm", PenKind::Pencil, 0.5, 0.5, 0.0, [0x55, 0x55, 0x5a], 0.75, Cap::Round),
        pen("std-nib-18", "Fountain pen nib 1.8 x 0.3 mm", PenKind::Fountain, 1.8, 0.3, 45.0, [0x10, 0x20, 0x60], 0.95, Cap::Flat),
        pen("std-marker-10", "Felt-tip marker 1 mm", PenKind::Marker, 1.0, 1.0, 0.0, [0xc0, 0x20, 0x20], 0.9, Cap::Round),
        pen("std-chisel-30", "Chisel marker 3 x 1 mm", PenKind::Marker, 3.0, 1.0, 45.0, black, 0.95, Cap::Flat),
        pen("std-highlight-40", "Highlighter 4 x 1 mm", PenKind::Highlighter, 4.0, 1.0, 45.0, [0xf0, 0xe0, 0x30], 0.4, Cap::Flat),
        pen("std-brush-20", "Brush pen 2 mm", PenKind::Brush, 2.0, 2.0, 0.0, black, 0.85, Cap::Round),
    ]
}

#[derive(Serialize, Deserialize)]
struct FileData {
    #[serde(default)]
    format: u32,
    pens: Vec<Pen>,
}

/// The pens: the ready-made ones and the person's own.
#[derive(Clone, Debug)]
pub struct PenLibrary {
    pub builtin: Vec<Pen>,
    pub user: Vec<Pen>,
}

impl Default for PenLibrary {
    fn default() -> PenLibrary {
        PenLibrary { builtin: builtin_pens(), user: vec![] }
    }
}

impl PenLibrary {
    pub fn new() -> PenLibrary {
        PenLibrary::default()
    }

    pub fn all(&self) -> impl Iterator<Item = &Pen> {
        self.builtin.iter().chain(self.user.iter())
    }

    pub fn get(&self, id: &str) -> Option<&Pen> {
        self.all().find(|p| p.id == id)
    }

    fn free_id(&self) -> String {
        (1..).map(|n| format!("user-{n}")).find(|id| self.get(id).is_none()).unwrap_or_default()
    }

    /// A name that no pen has: "Name (2)", "Name (3)" ...
    pub fn free_name(&self, base: &str) -> String {
        let base = base.trim();
        let taken = |n: &str| self.all().any(|p| p.name.eq_ignore_ascii_case(n));
        if !taken(base) {
            return base.to_string();
        }
        (2..).map(|n| format!("{base} ({n})")).find(|n| !taken(n)).unwrap_or_default()
    }

    /// Add a pen of the person's: it gets an id and a name of its own. Returns the id, or why it was refused.
    pub fn add(&mut self, mut p: Pen) -> Result<String, String> {
        if self.user.len() >= MAX_PENS {
            return Err("too many pens".into());
        }
        p.builtin = false;
        p.id = self.free_id();
        p.name = self.free_name(&p.name);
        p.check()?;
        let id = p.id.clone();
        self.user.push(p);
        Ok(id)
    }

    /// Replace the pen of the person that has the same id. A ready-made pen cannot be changed.
    pub fn replace(&mut self, p: Pen) -> Result<(), String> {
        p.check()?;
        let at = self.user.iter().position(|u| u.id == p.id).ok_or("a ready-made pen cannot be changed")?;
        self.user[at] = Pen { builtin: false, ..p };
        Ok(())
    }

    pub fn delete(&mut self, id: &str) -> bool {
        let n = self.user.len();
        self.user.retain(|p| p.id != id);
        self.user.len() != n
    }

    /// The pens as a file: all of the person's, or the listed ones (ready-made pens too).
    pub fn export(&self, ids: Option<&[String]>) -> String {
        let pens: Vec<Pen> = match ids {
            Some(ids) => self.all().filter(|p| ids.contains(&p.id)).cloned().collect(),
            None => self.user.clone(),
        };
        serde_json::to_string_pretty(&FileData { format: FILE_FORMAT, pens }).unwrap_or_default()
    }

    /// Read a file of pens into the library: every pen is checked, and given an id and a name of its own; a pen that does not pass
    /// is left out and named in the answer. Returns how many were added and the problems.
    pub fn import(&mut self, text: &str) -> Result<(usize, Vec<String>), String> {
        if text.len() as u64 > MAX_FILE {
            return Err("the file is too large".into());
        }
        let data: FileData = serde_json::from_str(text).map_err(|e| format!("this is not a file of pens: {e}"))?;
        if data.format > FILE_FORMAT {
            return Err("the file is from a newer version of the program".into());
        }
        let (mut added, mut problems) = (0, vec![]);
        for p in data.pens.into_iter().take(MAX_PENS) {
            let name = p.name.clone();
            match self.add(p) {
                Ok(_) => added += 1,
                Err(e) => problems.push(format!("{name}: {e}")),
            }
        }
        Ok((added, problems))
    }

    /// Pens a project carries that the library does not have by id join it (a project made on another computer).
    pub fn adopt(&mut self, pens: Vec<Pen>) {
        for mut p in pens.into_iter().take(MAX_PENS) {
            p.builtin = false;
            if p.check().is_ok() && self.get(&p.id).is_none() && self.user.len() < MAX_PENS {
                self.user.push(p);
            }
        }
    }

    /// The person's pens as they are kept between runs.
    pub fn save_text(&self) -> String {
        self.export(None)
    }

    /// Load the person's pens (what `save_text` wrote). A damaged file gives no pens.
    pub fn load_text(&mut self, text: &str) {
        self.user.clear();
        if let Ok(data) = serde_json::from_str::<FileData>(text) {
            for mut p in data.pens.into_iter().take(MAX_PENS) {
                p.builtin = false;
                if p.check().is_ok() && self.get(&p.id).is_none() {
                    self.user.push(p);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_ready_made_pen_is_valid_with_its_own_id_and_every_kind_is_there() {
        let all = builtin_pens();
        let ids: std::collections::HashSet<_> = all.iter().map(|p| p.id.clone()).collect();
        assert_eq!(ids.len(), all.len());
        for p in &all {
            assert!(p.check().is_ok() && p.builtin, "{}", p.name);
        }
        for k in PenKind::ALL {
            assert!(all.iter().any(|p| p.kind == k), "{k:?}");
        }
    }

    #[test]
    fn a_chisel_draws_a_thick_line_across_its_width_and_a_thin_one_along_it() {
        let lib = PenLibrary::new();
        let chisel = lib.get("std-chisel-30").unwrap();
        // The tip is turned by 45 degrees: the width of 3 mm lies along 45 degrees.
        let along_width = chisel.thickness(45f64.to_radians());
        let across = chisel.thickness(135f64.to_radians());
        assert!((across - 3.0).abs() < 1e-9, "a line across the width is 3 mm thick: {across}");
        assert!((along_width - 1.0).abs() < 1e-9, "a line along the width is only the height thick: {along_width}");
        let in_between = chisel.thickness(0.0);
        assert!(in_between > 1.0 && in_between < 3.0);
        // A round pen is as thick as it is wide, whichever way the line goes.
        let fine = lib.get("std-fine-04").unwrap();
        assert!((fine.thickness(0.3) - 0.4).abs() < 1e-12 && (fine.thickness(2.0) - 0.4).abs() < 1e-12);
        assert_eq!((chisel.thickest(), chisel.thinnest()), (3.0, 1.0));
    }

    #[test]
    fn the_pens_of_the_person_are_added_changed_deleted_kept_and_exchanged() {
        let mut lib = PenLibrary::new();
        let mut p = lib.get("std-fine-04").unwrap().clone();
        p.name = "Fineliner 0.4 mm".into();
        let id = lib.add(p.clone()).unwrap();
        assert_eq!(lib.get(&id).unwrap().name, "Fineliner 0.4 mm (2)", "a name of its own");
        assert!(!lib.get(&id).unwrap().builtin);
        // A ready-made pen cannot be changed or deleted; the person's can.
        assert!(lib.replace(lib.get("std-fine-04").unwrap().clone()).is_err());
        assert!(!lib.delete("std-fine-04"));
        let mut changed = lib.get(&id).unwrap().clone();
        changed.width = 0.6;
        lib.replace(changed.clone()).unwrap();
        assert_eq!(lib.get(&id).unwrap().width, 0.6);
        // Kept between runs.
        let mut again = PenLibrary::new();
        again.load_text(&lib.save_text());
        assert_eq!(again.get(&id), lib.get(&id));
        // A file is read into another library; a pen with a wrong number is named, not taken.
        let mut bad = changed.clone();
        bad.opacity = 7.0;
        bad.name = "Broken".into();
        let text = serde_json::to_string(&FileData { format: 1, pens: vec![changed, bad] }).unwrap();
        let mut other = PenLibrary::new();
        let (added, problems) = other.import(&text).unwrap();
        assert_eq!(added, 1);
        assert!(problems.iter().any(|s| s.starts_with("Broken")), "{problems:?}");
        assert!(other.import("nonsense").is_err());
        assert!(other.import(&" ".repeat(MAX_FILE as usize + 1)).is_err());
        assert!(other.import(r#"{"format": 99, "pens": []}"#).is_err());
        assert!(lib.delete(&id) && lib.get(&id).is_none());
    }

    #[test]
    fn pens_a_project_carries_join_the_library_once() {
        let mut lib = PenLibrary::new();
        let mut p = lib.get("std-marker-10").unwrap().clone();
        p.id = "from-friend".into();
        p.name = "Friend's marker".into();
        lib.adopt(vec![p.clone(), p.clone()]);
        assert_eq!(lib.user.len(), 1);
        let mut wrong = p.clone();
        wrong.id = "wrong".into();
        wrong.width = f64::NAN;
        lib.adopt(vec![wrong]);
        assert_eq!(lib.user.len(), 1, "a pen with a wrong number is not taken");
    }
}
