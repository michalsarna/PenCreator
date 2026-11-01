//! The machine: the link to a GRBL plotter (from baseCreator) and the buttons that use it (connect, home, frame, start,
//! stop, pause, start from a line, check the program). The state of the link, the pauses of a job, the probe, the macros
//! and the statistics are in `base_ui::machine_ctl`; here is what depends on the device and the project.
use crate::device::Device;
use crate::plot;
use crate::App;
use base_core::control;
use base_core::machine::LinkKind;
use base_ui::i18n::{tr, trf};
use base_ui::link::{self, Target};
use base_ui::machine_ctl::Notice;
use base_ui::theme;
use eframe::egui::{self, Color32, RichText};

pub use base_ui::machine_ctl::Machine as MachineLink;

/// Where the device says to connect, if anywhere.
pub fn target_of(dev: &Device) -> Option<Target> {
    if !dev.has_target() {
        return None;
    }
    Some(match dev.connection.link {
        LinkKind::Serial => Target::Serial { port: link::resolve_port(&dev.connection.port, &dev.connection.usb_id, &link::list_port_info()), baud: dev.connection.baud },
        LinkKind::Tcp => Target::Tcp { host: dev.connection.host.trim().to_string(), port: dev.connection.tcp_port },
    })
}

/// A job that waits for the check run to find nothing wrong.
pub struct AfterCheck {
    pub lines: Vec<String>,
    pub before: Vec<String>,
    pub after: Vec<String>,
}

impl App {
    pub(crate) fn connect_machine(&mut self) {
        let Some(target) = target_of(&self.device) else {
            self.status = tr("Choose a port or a host in the device settings first.").to_string();
            return;
        };
        self.machine.set_grbl_version(self.device.connection.grbl_version);
        self.machine.connect(target, self.device.connection.streaming);
    }

    pub(crate) fn disconnect_machine(&mut self) {
        self.machine.disconnect();
    }

    /// Copy the statistics and the other things the machine changes from the device in use to its entry in the list, so
    /// they are kept with it.
    pub(crate) fn keep_control(&mut self) {
        if let Some(slot) = self.active.and_then(|i| self.devices.get_mut(i)) {
            slot.control = self.device.control.clone();
        }
    }

    /// Take what the machine reports; called every frame.
    pub(crate) fn poll_machine(&mut self) {
        for n in self.machine.poll() {
            match n {
                Notice::Connected => {
                    self.machine.read_all();
                    if self.device.connection.unlock_on_connect {
                        self.machine.line("$X");
                    } else if self.device.connection.home_on_connect {
                        self.machine.line("$H");
                    }
                }
                Notice::Disconnected { lost } => {
                    if lost && self.device.connection.auto_reconnect {
                        self.machine.reconnect_at = Some(std::time::Instant::now() + std::time::Duration::from_secs(3));
                    }
                    self.after_check = None;
                }
                Notice::ReconnectDue => self.connect_machine(),
                Notice::Barrier(b) => self.status = trf("The job is paused: {}", &[&b.message()]),
                Notice::JobFinished { seconds, issues, checked } => {
                    if checked {
                        self.finish_check(issues);
                    } else {
                        for _ in 0..self.layer_watch.finished() {
                            self.cam_rec.layer_done(std::time::Instant::now());
                        }
                        self.device.control.stats.job_done(seconds);
                        self.keep_control();
                        self.status = if issues == 0 { tr("The job is finished.").to_string() } else { trf("The job is finished, with {} problems (see the Console).", &[&issues]) };
                    }
                }
                Notice::Alarm(code) => {
                    self.device.control.stats.alarm(code);
                    self.keep_control();
                    self.status = match base_core::grbl_codes::alarm(code) {
                        Some(t) => trf("Alarm {}: {}", &[&code, &tr(t)]),
                        None => trf("Alarm {}", &[&code]),
                    };
                    base_ui::alert_ui::raise_alarm(&mut self.machine, code, &base_core::grbl_codes::alarm(code).map(tr).unwrap_or(""));
                }
                Notice::Error { code, line } => {
                    // The errors a check run finds are the program's, not the machine's.
                    if !self.machine.checking() {
                        self.device.control.stats.error(code);
                        self.keep_control();
                    }
                    let text = base_core::grbl_codes::error(code).map(tr).unwrap_or("");
                    self.status = match line {
                        Some(n) => trf("Error {} in line {}: {}", &[&code, &(n + 1), &text]),
                        None => trf("Error {}: {}", &[&code, &text]),
                    };
                    // An error in the middle of a job stops it: the person must see that.
                    if line.is_some() && !self.machine.checking() {
                        base_ui::alert_ui::raise_job_stopped(&mut self.machine, &self.status.clone());
                    }
                }
                Notice::Stalled { secs, state } => {
                    self.status = trf("No answer from the controller for {} s while a job is running (state: {}).", &[&secs, &state]);
                    base_ui::alert_ui::raise_no_answer(&mut self.machine, &self.status.clone());
                }
                Notice::Read => {
                    // Something changed in the controller since the copy kept with the device? Say so, and make a new copy.
                    if let Some(line) = self.machine.compare_copies(&mut self.device.control.fw_copies) {
                        self.status = line;
                    }
                    self.keep_control();
                }
                // The tool length sensor is for milling machines.
                Notice::Probed { .. } | Notice::TlsReference { .. } | Notice::TlsTool { .. } | Notice::TlsTest { .. } | Notice::TlsFailed | Notice::CenterFound { .. } | Notice::CenterFailed => {}
            }
        }
        // The layers of the job that are done by now: a picture each, for the time-lapse of the camera.
        for _ in 0..self.layer_watch.progress(self.machine.progress.0) {
            self.cam_rec.layer_done(std::time::Instant::now());
        }
    }

    /// The windows of the machine: firmware settings, calibration, diagnostics, start from a line, the pause of a job.
    pub(crate) fn machine_windows(&mut self, ctx: &egui::Context) {
        let mut say = None;
        say = base_ui::job_ui::barrier_window(ctx, &mut self.machine, None).or(say);
        say = base_ui::firmware_ui::window(ctx, &mut self.machine, &mut self.device.control.fw_copies, &mut self.fw_open).or(say);
        self.keep_control();
        say = base_ui::tools_ui::calibration_window(ctx, &mut self.cal, &mut self.machine).or(say);
        if self.diag.open {
            let info = base_ui::tools_ui::DiagInfo { title: "PenCreator", version: crate::VERSION, device: base_core::control::Control::default().check().map(|_| serde_json::to_string_pretty(&self.device).unwrap_or_default()).unwrap_or_default(), gcode: self.diag_gcode.as_deref() };
            say = base_ui::tools_ui::diagnostics_window(ctx, &mut self.diag, &mut self.machine, &mut self.device.control, &info).or(say);
            self.keep_control();
        }
        let safe_z = self.safe_z();
        if let Some(from) = base_ui::job_ui::start_from_window(ctx, &mut self.start_from, &self.start_lines, safe_z, 0.0) {
            self.start_job_from(from);
        }
        if let Some(s) = say {
            self.status = s;
        }
    }

    pub(crate) fn can_start(&self) -> bool {
        self.machine.connected && !self.machine.job_active() && !self.machine.job_running()
    }

    /// Frame: the tool goes round the box where the work will be done, at the safe height, with the spindle off.
    pub(crate) fn frame_job(&mut self) {
        if !self.machine.connected {
            self.status = tr("Connect to the machine first (Device tab)").to_string();
            return;
        }
        if self.machine.job_running() {
            return;
        }
        let prog = self.make_program();
        let Some(b) = prog.bounds() else {
            self.status = tr("There is nothing to frame: put shapes on layers that are switched on.").to_string();
            return;
        };
        self.machine.lines([plot::frame_gcode(b, &self.device)]);
        self.status = tr("Framing the work area").to_string();
    }

    /// The lines of the whole program, or `None` (with the reason in the status bar) when there is nothing to cut.
    pub(crate) fn job_lines(&mut self) -> Option<(Vec<String>, f64)> {
        if !self.gcode_variables_ok() {
            return None;
        }
        let prog = self.make_program();
        if prog.is_empty() {
            self.status = self.nothing_to_draw();
            return None;
        }
        let name = self.path.as_ref().and_then(|p| p.file_stem()).map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| "untitled".into());
        let lines = link::clean_gcode(&plot::to_gcode_info(&prog, &self.device, &self.layers, &self.pens, &self.gcode_info(&name)));
        Some((lines, prog.estimate_seconds(&self.device, &self.layers)))
    }

    /// The G-code the person wrote for the start and the end of every job, with its variables filled in.
    fn hook_lines(&mut self) -> Option<(Vec<String>, Vec<String>)> {
        let vars = self.machine.macro_vars(self.safe_z(), self.job_box());
        let h = &self.device.control.hooks;
        match (control::expand(&h.before_job, &vars), control::expand(&h.after_job, &vars)) {
            (Ok(b), Ok(a)) => Some((link::clean_gcode(&b.join("\n")), link::clean_gcode(&a.join("\n")))),
            (Err(e), _) | (_, Err(e)) => {
                self.status = trf("The G-code around the job: {}", &[&e]);
                None
            }
        }
    }

    /// The box of the job for the variables of macros (x and y of the drawing, z unknown).
    pub(crate) fn job_box(&self) -> Option<([f64; 3], [f64; 3])> {
        self.make_program().bounds().map(|b| ([b[0], b[1], f64::NAN], [b[2], b[3], f64::NAN]))
    }

    /// The height of the pen when it is up, where a Z axis lifts it; a plain 5 mm otherwise.
    pub(crate) fn safe_z(&self) -> f64 {
        if self.device.pen.lift == crate::device::Lift::Z { self.device.pen.up.max(0.1) } else { 5.0 }
    }

    /// Start the job.
    pub(crate) fn start_job(&mut self) {
        self.start_job_from(0);
    }

    /// Start the job at line `from` of the program (0-based; 0 is the whole program).
    pub(crate) fn start_job_from(&mut self, from: usize) {
        if !self.can_start() {
            self.status = tr("Connect to the machine first (Device tab)").to_string();
            return;
        }
        let Some((all, secs)) = self.job_lines() else { return };
        let lines = match base_core::resume::plan(&all, from, self.safe_z(), 0.0) {
            Ok(l) => l,
            Err(e) => {
                self.status = trf("Cannot start here: {}", &[&tr(e)]);
                return;
            }
        };
        let Some((before, after)) = self.hook_lines() else { return };
        self.machine.info(trf("Starting job: {} lines, ~{}", &[&lines.len(), &base_ui::widgets::fmt_time(secs)]));
        if self.device.control.check_first && from == 0 {
            // The real job waits for a clean check run.
            self.after_check = Some(AfterCheck { lines: lines.clone(), before, after });
            self.machine.start_job(lines, vec![], vec![], true);
            self.status = tr("Checking the program first…").to_string();
            return;
        }
        // A picture after each layer of the time-lapse of the camera, when it is on.
        self.layer_watch.start(link::layer_marks(&lines));
        self.machine.start_job(lines, before, after, false);
        self.status = tr("The job is running.").to_string();
    }

    /// Run the program through the controller's check mode: nothing moves, the lines it does not understand are reported.
    pub(crate) fn check_program(&mut self) {
        if !self.can_start() {
            self.status = tr("Connect to the machine first (Device tab)").to_string();
            return;
        }
        let Some((lines, _)) = self.job_lines() else { return };
        self.after_check = None;
        self.machine.start_job(lines, vec![], vec![], true);
        self.status = tr("Checking the program…").to_string();
    }

    /// The check run ended: go on with the job if it found nothing, otherwise say where.
    fn finish_check(&mut self, issues: usize) {
        let pending = self.after_check.take();
        if issues == 0 {
            match pending {
                Some(job) => {
                    self.layer_watch.start(link::layer_marks(&job.lines));
                    self.machine.start_job(job.lines, job.before, job.after, false);
                    self.status = tr("The check found nothing. The job is running.").to_string();
                }
                None => self.status = tr("The check found nothing wrong.").to_string(),
            }
            return;
        }
        let first = self.machine.issues.first().map(|i| i.line + 1).unwrap_or(0);
        self.status = trf("The check found {} problems, the first in line {}. The job was not started.", &[&issues, &first]);
    }

    pub(crate) fn abort_job(&mut self) {
        self.after_check = None;
        self.layer_watch.stop();
        self.machine.stop_job();
    }

    pub(crate) fn pause_job(&mut self) {
        self.machine.pause();
    }

    pub(crate) fn resume_job(&mut self) {
        self.machine.resume();
    }

    pub(crate) fn machine_line(&mut self, line: &str) {
        self.machine.line(line);
    }

    /// START, wide: streams the job to the machine (it was in the bar over the work area).
    pub(crate) fn start_button(&mut self, ui: &mut egui::Ui) {
        let can = self.can_start();
        let go = egui::Button::new(RichText::new(tr("Start").to_uppercase()).color(Color32::WHITE).strong()).fill(theme::accent());
        let tip = if can { tr("Stream the job to the connected machine") } else { tr("Connect to the machine first (Device tab)") };
        if ui.add_enabled_ui(can, |ui| ui.add_sized([ui.available_width(), 40.0], go)).inner.on_hover_text(tip).on_disabled_hover_text(tip).clicked() {
            self.start_job();
        }
    }

    /// Frame (always the left one) and From line…, in a row of two with icons.
    pub(crate) fn job_buttons(&mut self, ui: &mut egui::Ui) {
        let half = ((ui.available_width() - ui.spacing().item_spacing.x) / 2.0).max(60.0);
        let running = self.machine.job_running();
        let tip = if self.machine.connected { tr("Go round the box where the work will be done") } else { tr("Connect to the machine first (Device tab)") };
        let frame_on = self.machine.connected && !running;
        let from_on = self.can_start();
        let ink = |on: bool| if on { theme::text() } else { theme::text_dim() };
        egui::Grid::new("job_buttons").num_columns(2).show(ui, |ui| {
            let frame = egui::Button::image_and_text(base_ui::icons::slot(Some("square-dashed"), ink(frame_on)), RichText::new(tr("Frame")).size(15.0)).min_size(egui::vec2(half, 32.0));
            if ui.add_enabled(frame_on, frame).on_hover_text(tip).on_disabled_hover_text(tip).clicked() {
                self.frame_job();
            }
            let from = egui::Button::image_and_text(base_ui::icons::slot(Some("play"), ink(from_on)), RichText::new(tr("From line…")).size(15.0)).min_size(egui::vec2(half, 32.0));
            if ui.add_enabled(from_on, from).on_hover_text(tr("Start the job at a line of the program")).clicked() {
                let ctx = ui.ctx().clone();
                base_ui::menu::MenuHost::run_act(self, &ctx, crate::menu::Act::StartFrom);
            }
            ui.end_row();
        });
        // The options of the job, under the buttons that start things: this session only, they change the G-code.
        ui.add_enabled_ui(!running, |ui| {
            ui.checkbox(&mut self.one_by_one, tr("One element at a time")).on_hover_text(tr("Each element (a group, or a shape that is in none) is drawn completely, all its layers and all their passes, before the next one; the nearest element goes first. Off: layer after layer, pass after pass."));
            ui.checkbox(&mut self.selection_only, tr("Selection only")).on_hover_text(tr("Only the shapes that are selected on the work area when the G-code is saved or the job is started are drawn."));
        });
        ui.add_space(4.0);
        ui.separator();
        ui.add_space(4.0);
    }

    /// What shows or hides something and stands in the bar over the work area: the toolpaths, the preview and the camera.
    pub(crate) fn view_buttons(&mut self, ui: &mut egui::Ui) {
        // Green while the toolpaths are shown on the work area, grey otherwise.
        let on = self.toolpaths_on;
        let (fill, ink) = if on { (Color32::from_rgb(0x2e, 0xa0, 0x4f), Color32::WHITE) } else { (theme::panel_dark(), theme::text_dim()) };
        let btn = egui::Button::image_and_text(base_ui::icons::slot(Some("route"), ink), RichText::new(tr("Toolpaths")).color(ink)).fill(fill);
        if ui.add(btn).on_hover_text(tr("Show the toolpaths on the work area")).clicked() {
            self.toolpaths_on = !self.toolpaths_on;
        }
        if base_ui::widgets::window_toggle(ui, &tr("Preview…"), &tr("Open the preview window"), self.show_plot, self.screen == crate::Screen::Editor) {
            if self.show_plot {
                self.show_plot = false;
            } else {
                self.open_preview();
            }
        }
        if self.device.camera().is_some() && base_ui::widgets::window_toggle(ui, &tr("Camera view"), &tr("Show the live camera picture of this machine"), self.show_camera, true) {
            self.show_camera = !self.show_camera;
            self.cam_ws_shown = self.show_camera;
            if self.show_camera {
                self.pv_tab_shown = false;
            }
        }
    }

    /// Home, Unlock, Pause and Resume, 2 × 2 with icons: in the Device tab under the machine and at the top of the Control tab.
    pub(crate) fn quick_row(&mut self, ui: &mut egui::Ui) {
        use base_ui::control_ui::Quick;
        let moves = self.machine.connected && !self.machine.job_running();
        match base_ui::control_ui::quick_buttons(ui, moves, self.machine.connected) {
            Some(Quick::Home) => self.machine.home(),
            Some(Quick::Unlock) => self.machine.unlock(),
            Some(Quick::Pause) => self.pause_job(),
            Some(Quick::Resume) => self.resume_job(),
            None => {}
        }
    }

    /// Raise the pen and Lower the pen, side by side: the pen goes where it stands, by the Z axis or the servo of the device.
    pub(crate) fn pen_buttons(&mut self, ui: &mut egui::Ui) {
        let on = self.machine.connected && !self.machine.job_running();
        let tip = tr("Connect to the machine first (Device tab)");
        let half = ((ui.available_width() - ui.spacing().item_spacing.x) / 2.0).max(60.0);
        let ink = if on { theme::text() } else { theme::text_dim() };
        let mut go = None;
        egui::Grid::new("pen_buttons").num_columns(2).show(ui, |ui| {
            for (down, icon, text) in [(false, "arrow-up", tr("Raise the pen")), (true, "arrow-down", tr("Lower the pen"))] {
                let b = egui::Button::image_and_text(base_ui::icons::slot(Some(icon), ink), RichText::new(text).size(15.0)).min_size(egui::vec2(half, 32.0));
                let r = ui.add_enabled(on, b);
                if !self.machine.connected {
                    r.on_disabled_hover_text(tip);
                } else if r.clicked() {
                    go = Some(down);
                }
            }
            ui.end_row();
        });
        if let Some(down) = go {
            self.machine.lines([crate::plot::pen_gcode(&self.device, down)]);
        }
    }

    /// The Connection part of the Device tab: connect, the state, the positions and the buttons that need the link.
    pub(crate) fn machine_section(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new(tr("Machine")).strong());
        ui.horizontal_wrapped(|ui| {
            if self.machine.connected {
                if ui.button(tr("Disconnect")).clicked() {
                    self.disconnect_machine();
                }
            } else if ui.add_enabled(self.device.has_target(), egui::Button::new(tr("Connect"))).on_hover_text(tr("Open the link to the controller set in the device settings")).clicked() {
                self.connect_machine();
            }
            let (text, color) = if self.machine.connected { (tr("Connected"), Color32::from_rgb(0x2e, 0xa0, 0x4f)) } else { (tr("Disconnected"), theme::text_dim()) };
            ui.label(RichText::new(text).color(color));
            if !self.machine.state.is_empty() {
                ui.label(RichText::new(&self.machine.state).strong());
            }
        });
        if self.machine.connected && self.machine.pos.iter().all(|v| v.is_finite()) {
            let p = self.machine.pos;
            ui.label(RichText::new(format!("X {:.3}   Y {:.3}   Z {:.3}", p[0], p[1], p[2])).monospace());
        }
        ui.add_space(4.0);
        ui.separator();
        ui.add_space(4.0);
        self.quick_row(ui);
        if self.machine.job_running() {
            let (d, t) = self.machine.progress;
            ui.add(egui::ProgressBar::new(d as f32 / t.max(1) as f32).text(format!("{d} / {t}")));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::time::{Duration, Instant};

    /// A GRBL that says `ok` to every line and reports Idle.
    fn fake_grbl(listener: TcpListener) {
        std::thread::spawn(move || {
            let (mut s, _) = listener.accept().unwrap();
            let _ = s.set_read_timeout(Some(Duration::from_millis(10)));
            let start = Instant::now();
            let (mut buf, mut chunk) = (Vec::new(), [0u8; 64]);
            while start.elapsed() < Duration::from_secs(20) {
                match s.read(&mut chunk) {
                    Ok(0) => break,
                    Ok(n) => {
                        for &b in &chunk[..n] {
                            match b {
                                b'?' => {
                                    let _ = s.write_all(b"<Idle|MPos:0.000,0.000,0.000|WCO:0.000,0.000,0.000>\n");
                                }
                                0x18 => {
                                    let _ = s.write_all(b"\r\nGrbl 1.1h ['$' for help]\r\n");
                                }
                                b'\n' => {
                                    buf.clear();
                                    let _ = s.write_all(b"ok\n");
                                }
                                _ => buf.push(b),
                            }
                        }
                    }
                    _ => {}
                }
            }
        });
    }

    #[test]
    fn a_job_stops_when_the_pen_is_to_be_changed_and_goes_on_when_told() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        fake_grbl(listener);
        let ctx = egui::Context::default();
        let mut a = App::build(&ctx, None, false);
        a.device.connection.link = LinkKind::Tcp;
        a.device.connection.host = "127.0.0.1".into();
        a.device.connection.tcp_port = port;
        a.devices = vec![a.device.clone()];
        a.active = Some(0);
        crate::screenshots::demo_job(&mut a);
        a.connect_machine();
        let until = |a: &mut App, secs: f64, mut f: Box<dyn FnMut(&mut App) -> bool>| {
            let end = Instant::now() + Duration::from_secs_f64(secs);
            while Instant::now() < end {
                a.poll_machine();
                if f(a) {
                    return true;
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            false
        };
        assert!(until(&mut a, 3.0, Box::new(|a| a.machine.connected)));
        a.start_job();
        let mut paused = false;
        assert!(until(&mut a, 10.0, Box::new(move |a| {
            if a.machine.barrier.is_some() {
                paused = true;
                a.machine.continue_job();
            }
            paused && !a.machine.job_active()
        })), "the job stops at the pen change and then ends");
        assert_eq!(a.device.control.stats.jobs, 1);
        assert_eq!(a.devices[0].control.stats.jobs, 1);
        // The safe height is the pen's height with a Z axis and a plain 5 mm with a servo.
        assert_eq!(a.safe_z(), a.device.pen.up.max(0.1));
        a.device.pen.lift = crate::device::Lift::Servo;
        assert_eq!(a.safe_z(), 5.0);
    }

    #[test]
    fn a_device_without_a_port_or_host_has_no_target_and_a_port_gives_one() {
        let mut d = Device::dummy();
        assert!(target_of(&d).is_none());
        d.connection.port = "/dev/ttyUSB0".into();
        assert!(matches!(target_of(&d), Some(Target::Serial { baud: 115_200, .. })));
        d.connection.link = LinkKind::Tcp;
        assert!(target_of(&d).is_none());
        d.connection.host = "pi.local".into();
        assert!(matches!(target_of(&d), Some(Target::Tcp { port: 3333, .. })));
    }
}
