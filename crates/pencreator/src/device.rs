//! A device: the pen plotter a project is made for. It holds the size of the work area, how the pen is lifted, the speed
//! of the moves with the pen up and the cameras. The devices are kept in one file; a device can be exported to a file and imported.
use base_core::camimg::{CameraKind, CameraSource};
use base_core::control::Control;
use base_core::machine::{Connection, LinkKind};
use base_core::units::Units;
use serde::{Deserialize, Serialize};

/// The name of the stand-in device for working without a machine. It is the same in every language, so the one device is
/// found again after the language is changed.
pub const DUMMY_NAME: &str = "Dummy";
pub const MAX_DEVICES: usize = 200;
pub const MAX_FILE: u64 = 2_000_000;
pub const FILE_FORMAT: u32 = 1;

/// The size of the paper the pen reaches: the work area (mm).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Area {
    pub x: f64,
    pub y: f64,
}

/// How the pen is lifted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lift {
    /// The Z axis moves the pen: `up` and `down` are heights in mm.
    Z,
    /// A servo on the spindle output (`M3 S…`): `up` and `down` are the S values (0 to 1000).
    Servo,
    /// G-code of one's own for up and for down (`Device::pen_gcode`): a solenoid, a servo of Marlin (`M280`), anything else.
    Custom,
}

impl Lift {
    pub const ALL: [Lift; 3] = [Lift::Z, Lift::Servo, Lift::Custom];
    pub fn label(self) -> &'static str {
        match self {
            Lift::Z => "Z axis",
            Lift::Servo => "Servo (M3 S…)",
            Lift::Custom => "Own G-code",
        }
    }
}

/// The pen mechanism: how it goes up and down and how long it takes.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PenLift {
    pub lift: Lift,
    /// The value with the pen up (mm for the Z axis, S for a servo).
    pub up: f64,
    /// The value with the pen on the paper.
    pub down: f64,
    /// Milliseconds to wait after the pen moved, before the next move.
    pub delay_ms: f64,
}

/// Where the pen is changed when the next layer needs another one: the plotter takes the pen up, drives to this place, runs the
/// G-code of the change (for a mechanism that changes the pen by itself) and stops there (M0) so that the pen can be changed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PenChange {
    pub enabled: bool,
    /// The place, in the coordinates of the work area (they may lie outside of it), in mm.
    pub x: f64,
    pub y: f64,
    /// Stop there with M0 and wait for the person (or the mechanism that answers); off for a mechanism that needs no waiting.
    #[serde(default = "yes")]
    pub pause: bool,
    /// Lines of G-code sent at the place before the stop (at most 20 lines of 100 characters).
    #[serde(default)]
    pub gcode: String,
}

fn yes() -> bool {
    true
}

impl Default for PenChange {
    fn default() -> PenChange {
        PenChange { enabled: false, x: 0.0, y: 0.0, pause: true, gcode: String::new() }
    }
}

impl PenChange {
    /// The lines of the G-code of the change, without the empty ones.
    pub fn lines(&self) -> Vec<&str> {
        self.gcode.lines().map(str::trim).filter(|l| !l.is_empty()).collect()
    }
}

/// The corner of the work area that is the machine zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Origin {
    FrontLeft,
    FrontRight,
    BackLeft,
    BackRight,
}

impl Origin {
    pub const ALL: [Origin; 4] = [Origin::FrontLeft, Origin::FrontRight, Origin::BackLeft, Origin::BackRight];
    pub fn label(self) -> &'static str {
        match self {
            Origin::FrontLeft => "Front left",
            Origin::FrontRight => "Front right",
            Origin::BackLeft => "Back left",
            Origin::BackRight => "Back right",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub units: Units,
    pub area: Area,
    pub origin: Origin,
    /// The speed of the moves with the pen up (mm/min).
    pub rapid: f64,
    pub pen: PenLift,
    /// Where the pen is changed, if it is changed at a place of the machine.
    #[serde(default)]
    pub pen_change: PenChange,
    #[serde(default)]
    pub cameras: Vec<CameraSource>,
    #[serde(default)]
    pub active_camera: usize,
    #[serde(default)]
    pub notes: String,
    /// How the program reaches the machine (a GRBL controller on a serial port or a network socket).
    #[serde(default)]
    pub connection: Connection,
    /// Jogging, macros, statistics and maintenance of the machine.
    #[serde(default)]
    pub control: Control,
    /// The keyboard or pad of this machine; without one the default one drives it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub control_device: Option<String>,
    /// How the camera view opens: the way, the zoom and where its window of its own was (not written while it was never changed).
    #[serde(default, skip_serializing_if = "is_unset")]
    pub camera_view: base_core::camimg::CameraView,
    /// The very start and the very end of the file.
    #[serde(default, skip_serializing_if = "is_unset")]
    pub frame: base_core::gcode_out::Frame,
    /// The last pass over the finished program, for controllers that read G-code in their own way (all off by default).
    #[serde(default, skip_serializing_if = "is_unset")]
    pub flavour: base_core::gcode_out::Flavour,
    /// The G-code of the pen with `Lift::Custom` (not written while unset).
    #[serde(default, skip_serializing_if = "PenGcode::is_empty")]
    pub pen_gcode: PenGcode,
    /// The tool is a drag knife (cutting vinyl, paper, card): its outlines are cut with the offset of the blade made good, inside
    /// first, and nothing is filled (not written while it is a pen).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub knife: Option<DragKnife>,
    /// LinuxCNC runs the program by itself: Start saves the G-code file for it, nothing is streamed (not written while off).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub linuxcnc: bool,
}

/// The G-code of one's own that takes the pen up and down (`Lift::Custom`): one or more lines each.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PenGcode {
    #[serde(default)]
    pub up: String,
    #[serde(default)]
    pub down: String,
}

impl PenGcode {
    pub fn is_empty(&self) -> bool {
        self.up.trim().is_empty() && self.down.trim().is_empty()
    }

    /// A start for a pen on the spindle output switched on and off (a solenoid).
    pub fn suggestion() -> PenGcode {
        PenGcode { up: "M5".into(), down: "M3 S1000".into() }
    }
}

/// A setting that was never changed is not written: files made before the setting existed stay the same.
fn is_unset<T: Default + PartialEq>(v: &T) -> bool {
    *v == T::default()
}

/// The variables of the own G-code that do not depend on the place in the program.
pub const FILE_VARS: &[&str] = base_core::gcode_out::COMMON_VARS;
/// The variables of the G-code of the pen change: the common ones and the layer.
pub const CHANGE_VARS: &[&str] = &["project_name", "project_path", "date", "time", "machine_name", "min_x", "max_x", "min_y", "max_y", "min_z", "max_z", "width", "height", "depth", "layer_name", "layer_index", "total_layers"];

impl Device {
    /// The first unknown variable (or a brace that does not close) in the own G-code, said with the name of the field.
    pub fn check_variables(&self) -> Result<(), String> {
        base_core::gcode_out::check_texts(&[("Very start of the file", &self.frame.very_start), ("Very end of the file", &self.frame.very_end)], FILE_VARS)?;
        base_core::gcode_out::check_texts(&[("G-code of the pen change", &self.pen_change.gcode)], CHANGE_VARS)
    }
}

impl Default for Device {
    /// The plain defaults: an A4 sheet, the pen lifted by the Z axis.
    fn default() -> Device {
        Device {
            name: String::new(),
            units: Units::Mm,
            area: Area { x: 297.0, y: 210.0 },
            origin: Origin::FrontLeft,
            rapid: 6000.0,
            pen: PenLift { lift: Lift::Z, up: 3.0, down: 0.0, delay_ms: 150.0 },
            pen_change: PenChange::default(),
            cameras: vec![],
            active_camera: 0,
            notes: String::new(),
            connection: Connection::default(),
            control: Control::default(),
            control_device: None,
            camera_view: Default::default(),
            frame: Default::default(),
            flavour: Default::default(),
            pen_gcode: PenGcode::default(),
            linuxcnc: false,
            knife: None,
        }
    }
}

impl Device {
    /// The stand-in device: the defaults under the name Dummy.
    pub fn dummy() -> Device {
        Device { name: DUMMY_NAME.to_string(), ..Device::default() }
    }

    pub fn is_dummy(&self) -> bool {
        self.name == DUMMY_NAME
    }

    pub fn camera(&self) -> Option<&CameraSource> {
        self.cameras.get(self.active_camera).filter(|c| c.enabled && c.is_valid())
    }

    /// Are the numbers usable? Used for what is read from a file and before a device is saved.
    pub fn check(&self) -> Result<(), String> {
        let ok = |v: f64, lo: f64, hi: f64| v.is_finite() && v >= lo && v <= hi;
        if self.name.trim().is_empty() || self.name.len() > 120 {
            return Err("the name is empty or too long".into());
        }
        if !ok(self.area.x, 10.0, 10_000.0) || !ok(self.area.y, 10.0, 10_000.0) {
            return Err("the work area is out of range".into());
        }
        if let Some(k) = self.knife {
            if !ok(k.offset, 0.0, 3.0) || !ok(k.swivel_deg, 1.0, 90.0) || !ok(k.overcut, 0.0, 5.0) {
                return Err("the settings of the drag knife are out of range".into());
            }
        }
        if !ok(self.rapid, 1.0, 100_000.0) {
            return Err("the speed of the moves is out of range".into());
        }
        let p = &self.pen;
        if p.lift == Lift::Custom {
            let g = &self.pen_gcode;
            if g.up.trim().is_empty() || g.down.trim().is_empty() || g.up.trim() == g.down.trim() || !ok(p.delay_ms, 0.0, 10_000.0) {
                return Err("the G-code of the pen up and down is empty or the same".into());
            }
            for t in [&g.up, &g.down] {
                if t.lines().count() > 10 || t.lines().any(|l| l.len() > 100 || l.chars().any(|c| c.is_control() || !c.is_ascii())) {
                    return Err("the G-code of the pen has too many lines, lines that are too long or characters that are not plain ASCII".into());
                }
            }
            if self.connection.grbl_version.controller().plotter_language().is_some() {
                return Err("a plotter that does not speak G-code (AxiDraw, HPGL) lifts the pen by the Z axis or the servo, not by G-code of one's own".into());
            }
        } else {
            let (lo, hi) = if p.lift == Lift::Z { (-100.0, 200.0) } else { (0.0, 1000.0) };
            if !ok(p.up, lo, hi) || !ok(p.down, lo, hi) || p.up == p.down || !ok(p.delay_ms, 0.0, 10_000.0) {
                return Err("the pen up and down values are out of range or the same".into());
            }
        }
        let c = &self.pen_change;
        if !ok(c.x, -10_000.0, 10_000.0) || !ok(c.y, -10_000.0, 10_000.0) {
            return Err("the place of the pen change is out of range".into());
        }
        if c.gcode.lines().count() > 20 || c.gcode.lines().any(|l| l.len() > 100 || l.chars().any(char::is_control)) {
            return Err("the G-code of the pen change has too many lines, lines that are too long or control characters".into());
        }
        for t in [&self.frame.very_start, &self.frame.very_end] {
            if t.lines().count() > 20 || t.lines().any(|l| l.len() > 100 || l.chars().any(char::is_control)) {
                return Err("the very start or the very end of the file has too many lines, lines that are too long or control characters".into());
            }
        }
        let cv = &self.camera_view;
        if cv.mode > 3 || !(cv.zoom.is_finite() && (0.5..=20.0).contains(&cv.zoom)) || [cv.win_pos, cv.win_size, cv.float_pos, cv.float_size].iter().flatten().flatten().any(|v| !v.is_finite() || v.abs() > 100_000.0) {
            return Err("the camera view settings are out of range".into());
        }
        if self.cameras.len() > 8 || self.active_camera > 8 || self.notes.len() > 20_000 {
            return Err("too many cameras or too long notes".into());
        }
        for c in &self.cameras {
            if c.name.len() > 120 || c.rotation > 3 {
                return Err("a camera is not valid".into());
            }
            if let CameraKind::Url(u) = &c.kind {
                if !u.is_empty() && !base_core::camimg::valid_url(u) {
                    return Err("a camera address is not an http, https, rtsp, udp, rtp or rtmp address".into());
                }
            }
        }
        let c = &self.connection;
        if c.baud < 300 || c.baud > 4_000_000 || c.host.len() > 253 || c.port.len() > 200 || c.usb_id.len() > 9 || (!c.usb_id.is_empty() && base_core::machine::parse_usb_id(&c.usb_id).is_none()) {
            return Err("the connection settings are out of range".into());
        }
        self.control.check()?;
        self.connection.plotter.check()?;
        Ok(())
    }

    /// Is there somewhere to connect to?
    pub fn has_target(&self) -> bool {
        match self.connection.link {
            LinkKind::Serial => !self.connection.port.trim().is_empty(),
            LinkKind::Tcp => !self.connection.host.trim().is_empty(),
        }
    }

    /// A free name: the wanted one, or with a number after it.
    pub fn free_name(wanted: &str, taken: &[String]) -> String {
        if !taken.iter().any(|t| t == wanted) {
            return wanted.to_string();
        }
        (2..).map(|n| format!("{wanted} ({n})")).find(|n| !taken.contains(n)).unwrap_or_else(|| wanted.to_string())
    }
}

#[derive(Serialize, Deserialize)]
struct OneFile {
    format: u32,
    device: Device,
}

#[derive(Serialize, Deserialize)]
struct ManyFile {
    format: u32,
    devices: Vec<Device>,
}

/// One device as a file.
pub fn export(d: &Device) -> String {
    serde_json::to_string_pretty(&OneFile { format: FILE_FORMAT, device: d.clone() }).unwrap_or_default()
}

/// A device from a file. It is checked strictly; a newer format is refused.
pub fn import(text: &str) -> Result<Device, String> {
    if text.len() as u64 > MAX_FILE {
        return Err("the file is too large".into());
    }
    let f: OneFile = serde_json::from_str(text).map_err(|e| format!("this is not a device file: {e}"))?;
    if f.format > FILE_FORMAT {
        return Err("the file is from a newer version of the program".into());
    }
    f.device.check()?;
    Ok(f.device)
}

/// All devices as they are kept between runs.
pub fn save_all(devices: &[Device]) -> String {
    serde_json::to_string_pretty(&ManyFile { format: FILE_FORMAT, devices: devices.to_vec() }).unwrap_or_default()
}

/// The devices that were kept. A device that does not pass the check is left out.
pub fn load_all(text: &str) -> Vec<Device> {
    match serde_json::from_str::<ManyFile>(text) {
        Ok(f) if f.format <= FILE_FORMAT => f.devices.into_iter().take(MAX_DEVICES).filter(|d| d.check().is_ok()).collect(),
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_defaults_and_the_dummy_are_valid() {
        assert!(Device::dummy().check().is_ok());
        assert!(Device::dummy().is_dummy());
        assert!(Device::default().check().is_err(), "a device needs a name");
    }

    #[test]
    fn the_place_of_the_pen_change_is_checked_and_an_older_device_has_none() {
        let mut d = Device::dummy();
        assert!(!d.pen_change.enabled && d.pen_change.pause);
        let json = serde_json::to_string(&d).unwrap();
        let old: String = {
            let mut v: serde_json::Value = serde_json::from_str(&json).unwrap();
            v.as_object_mut().unwrap().remove("pen_change");
            v.to_string()
        };
        let back: Device = serde_json::from_str(&old).unwrap();
        assert_eq!(back.pen_change, PenChange::default());
        d.pen_change = PenChange { enabled: true, x: -20.0, y: 50.0, pause: false, gcode: "M8\nG4 P1".into() };
        assert!(d.check().is_ok());
        assert_eq!(d.pen_change.lines(), vec!["M8", "G4 P1"]);
        let again: Device = serde_json::from_str(&serde_json::to_string(&d).unwrap()).unwrap();
        assert_eq!(again.pen_change, d.pen_change);
        d.pen_change.x = f64::NAN;
        assert!(d.check().is_err());
        d.pen_change.x = 0.0;
        d.pen_change.gcode = "x".repeat(101);
        assert!(d.check().is_err(), "a line that is too long");
        d.pen_change.gcode = (0..21).map(|i| format!("G4 P{i}")).collect::<Vec<_>>().join("\n");
        assert!(d.check().is_err(), "too many lines");
        d.pen_change.gcode = "M8\u{7}".into();
        assert!(d.check().is_err(), "a control character");
    }

    #[test]
    fn wrong_numbers_are_refused() {
        let mut d = Device::dummy();
        d.area.x = f64::NAN;
        assert!(d.check().is_err());
        let mut d = Device::dummy();
        d.pen.up = d.pen.down;
        assert!(d.check().is_err(), "up and down must differ");
        let mut d = Device::dummy();
        d.pen.lift = Lift::Servo;
        d.pen.up = 2000.0;
        assert!(d.check().is_err(), "S above 1000");
        d.pen.up = 800.0;
        d.pen.down = 300.0;
        assert!(d.check().is_ok());
        d.cameras.push(CameraSource::url("c", "ftp://x", 0));
        assert!(d.check().is_err());
        d.cameras[0] = CameraSource::url("c", "http://pi.local:8080/s.mjpg", 1);
        assert!(d.check().is_ok());
    }

    #[test]
    fn a_device_goes_to_a_file_and_back_and_a_bad_file_is_refused() {
        let mut d = Device::dummy();
        d.name = "My router".into();
        d.area.x = 420.0;
        d.cameras.push(CameraSource::url("top", "http://pi.local/s.mjpg", 2));
        let back = import(&export(&d)).unwrap();
        assert_eq!(back, d);
        assert!(import("nonsense").is_err());
        assert!(import(r#"{"format": 9, "device": {}}"#).is_err());
        let mut bad = d.clone();
        bad.area.y = -1.0;
        assert!(import(&export(&bad)).is_err());
    }

    #[test]
    fn the_kept_devices_come_back_and_a_damaged_file_gives_none() {
        let mut a = Device::dummy();
        a.name = "A".into();
        let all = vec![a.clone(), Device::dummy()];
        assert_eq!(load_all(&save_all(&all)), all);
        assert!(load_all("{ broken").is_empty());
        // A device with wrong numbers is left out, the rest stay.
        let mut bad = a.clone();
        bad.area.x = 0.0;
        assert_eq!(load_all(&save_all(&[bad, a.clone()])), vec![a]);
    }

    #[test]
    fn a_free_name_gets_a_number() {
        let taken = vec!["A".to_string(), "A (2)".to_string()];
        assert_eq!(Device::free_name("B", &taken), "B");
        assert_eq!(Device::free_name("A", &taken), "A (3)");
    }
}

/// A drag knife on the plotter: its blade turns freely and its tip trails `offset` behind the axis.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DragKnife {
    /// The tip behind the axis (mm).
    pub offset: f64,
    /// A turn sharper than this (degrees) is made by turning the blade round the corner.
    pub swivel_deg: f64,
    /// How far a closed outline is cut on past its start (mm).
    pub overcut: f64,
    /// The cutter makes good the offset itself (most vinyl cutters do): the program cuts the outlines as they are.
    #[serde(default)]
    pub by_controller: bool,
}

impl Default for DragKnife {
    fn default() -> DragKnife {
        let k = base_core::drag_knife::Knife::default();
        DragKnife { offset: k.offset, swivel_deg: k.swivel_deg, overcut: k.overcut, by_controller: false }
    }
}

impl DragKnife {
    /// The knife the program makes good, or `None` when the cutter does it.
    pub fn compensated(&self) -> Option<base_core::drag_knife::Knife> {
        (!self.by_controller).then_some(base_core::drag_knife::Knife { offset: self.offset, swivel_deg: self.swivel_deg, overcut: self.overcut })
    }
}
