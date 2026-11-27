//! The performance test of the program on the shared test project of baseCreator (`testdata/perf/perf-block.cre`: a block with six
//! faces of drawings and 5000 objects beside the work areas). Run before every release by `scripts/perf.sh` of baseCreator, which
//! keeps the results (`testdata/perf/RESULTS.md`):
//! `cargo test --release -p pencreator perf_ -- --ignored --nocapture --test-threads=1`.
use crate::{App, Screen};
use eframe::egui;
use std::time::Instant;

/// The test project: `CREATOR_PERF_FILE`, or the one of the baseCreator beside this repository.
fn perf_file() -> std::path::PathBuf {
    std::env::var_os("CREATOR_PERF_FILE").map(Into::into).unwrap_or_else(|| std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../baseCreator/testdata/perf/perf-block.cre"))
}

/// One result, as `scripts/perf.sh` reads it.
fn report(what: &str, ms: f64, detail: &str) {
    println!("PERF\tPenCreator\t{}\t{what}\t{ms:.0}\t{detail}", crate::VERSION);
}

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}

/// The pen layers of the test project: outlines, fills and both (written into its `pen` section). PenCreator has no blocks: it
/// works on face A, the drawing of the project.
#[test]
#[ignore = "makes the performance test project; run by hand"]
fn perf_prepare_the_project() {
    let p = perf_file();
    let mut a = App::build(&egui::Context::default(), None, false);
    a.open(p.clone());
    assert!(!a.scene.shapes.is_empty(), "the project is read: {}", a.status);
    for (i, l) in a.layers.iter_mut().enumerate().take(13) {
        l.enabled = true;
        l.visible = true;
        l.draw = match i {
            1 | 6 => crate::layers::Draw::Fill,
            4 => crate::layers::Draw::OutlineAndFill,
            _ => crate::layers::Draw::Outline,
        };
        l.hatch_gap = l.hatch_gap.max(0.5);
    }
    std::fs::write(&p, a.to_project().to_json()).expect("written");
}

#[test]
#[ignore = "performance; run by scripts/perf.sh of baseCreator"]
fn perf_open_gcode_and_preview() {
    let p = perf_file();
    let ctx = egui::Context::default();
    let mut a = App::build(&ctx, None, false);
    let t = Instant::now();
    a.open(p);
    report("open", ms(t), &format!("{} shapes on face A", a.scene.shapes.len()));
    assert!(!a.scene.shapes.is_empty(), "the project is read: {}", a.status);
    a.screen = Screen::Editor;
    // No blocks in PenCreator: face A.
    let faces = 1;
    // The G-code of every layer, of every face.
    let t = Instant::now();
    let mut bytes = 0;
    for _ in 0..faces {
        bytes += a.gcode_text("perf").map_or(0, |(g, _)| g.len());
    }
    report("gcode", ms(t), &format!("{faces} faces, {} kB", bytes / 1024));
    // The preview of every face: the program and a frame of its tab (the plot).
    let t = Instant::now();
    a.open_preview();
    for _ in 0..faces {
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            a.preview_window(ui.ctx());
            egui::CentralPanel::no_frame().show(ui, |ui| a.preview_work_panel(ui));
        });
        out.textures_delta.clear();
    }
    report("preview", ms(t), &format!("{faces} faces"));
}
