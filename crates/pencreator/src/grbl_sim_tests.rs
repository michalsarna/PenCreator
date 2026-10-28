//! End-to-end tests of what PenCreator generates, against the real GRBL 1.1h firmware compiled for the PC (grbl-sim).
//!
//! The harness and the tests of the serial link itself (streaming, feed hold, abort, settings, `M0` pauses, spindle and arcs)
//! are in baseCreator (`base_ui::sim_test`, docs/grbl-sim.md there); these check the jobs of the plotter: that GRBL accepts
//! every line with the pen lifted by the Z axis and by a servo on the spindle output, that the pen change stops the job where
//! it must, and that the plotter ends at zero. They are `#[ignore]`d because they need the simulator binary: build it with
//! `tools/grbl-sim/build.sh` of baseCreator into `target/grbl-sim` (or point `GRBL_SIM_EXE` at it) and run
//!
//! ```sh
//! cargo test -p pencreator grbl_sim -- --ignored --test-threads=1
//! ```
use crate::device::{Device, Lift};
use crate::layers::default_layers;
use crate::pens::PenLibrary;
use crate::plot::{generate_with, to_gcode, JobOptions};
use base_core::geom::Xf;
use base_core::scene::{Kind, Scene};
use base_ui::link::{clean_gcode, Cmd};
use base_ui::sim_test::Session;

fn square(scene: &mut Scene, layer: usize, x: f64, y: f64, s: f64) {
    scene.add(layer, Kind::Rect { w: s, h: s }, Xf::translate(x, y));
}

/// The G-code the program sends for `scene` with the device `dev`.
fn job_of(scene: &Scene, dev: &Device) -> Vec<String> {
    job_with(scene, dev, JobOptions::default())
}

/// The same with the options of the Control tab.
fn job_with(scene: &Scene, dev: &Device, opts: JobOptions) -> Vec<String> {
    let layers = default_layers();
    let prog = generate_with(scene, &layers, dev, opts);
    clean_gcode(&to_gcode(&prog, dev, &layers, &PenLibrary::new(), "sim"))
}

#[test]
#[ignore = "needs grbl-sim: tools/grbl-sim/build.sh of baseCreator"]
fn grbl_sim_a_drawing_with_the_pen_on_the_z_axis_is_accepted_and_ends_at_zero() {
    let mut s = Session::connect();
    s.wait_banner();
    s.fast_machine(&[]);
    let mut scene = Scene::default();
    square(&mut scene, 0, 20.0, 20.0, 30.0);
    let dev = Device::dummy();
    let job = job_of(&scene, &dev);
    assert!(job.iter().any(|l| l.starts_with("G0 Z")), "{job:?}");
    s.run_job(job, 120.0);
    s.wait_idle_at(0.0, 0.0, 60.0);
    assert!(s.errors().is_empty(), "GRBL rejected lines: {:?}", s.errors_described());
}

#[test]
#[ignore = "needs grbl-sim: tools/grbl-sim/build.sh of baseCreator"]
fn grbl_sim_a_servo_pen_is_driven_with_m3_and_is_off_at_the_end() {
    let mut s = Session::connect();
    s.wait_banner();
    s.fast_machine(&[]);
    let mut scene = Scene::default();
    square(&mut scene, 0, 20.0, 20.0, 30.0);
    let mut dev = Device::dummy();
    dev.pen.lift = Lift::Servo;
    dev.pen.up = 300.0;
    dev.pen.down = 700.0;
    dev.pen.delay_ms = 100.0;
    let job = job_of(&scene, &dev);
    assert!(job.iter().any(|l| l == "M3 S700") && job.iter().any(|l| l == "M3 S300"), "{job:?}");
    s.run_job(job, 120.0);
    s.wait_idle_at(0.0, 0.0, 60.0);
    // The program ends with M5: the servo output is off.
    assert!(s.parser_state().contains("M5"), "the servo output is off");
    assert!(s.errors().is_empty(), "GRBL rejected lines: {:?}", s.errors_described());
}

#[test]
#[ignore = "needs grbl-sim: tools/grbl-sim/build.sh of baseCreator"]
fn grbl_sim_the_pen_change_stops_the_job_and_it_goes_on_when_told() {
    let mut s = Session::connect();
    s.wait_banner();
    s.fast_machine(&[]);
    // As the program starts a job: the pauses (M0) are the program's to hold.
    s.link.send(Cmd::Barriers(true));
    let mut scene = Scene::default();
    square(&mut scene, 0, 20.0, 20.0, 10.0);
    square(&mut scene, 1, 60.0, 20.0, 10.0);
    let job = job_of(&scene, &Device::dummy());
    assert_eq!(job.iter().filter(|l| l.starts_with("M0 (Change the pen")).count(), 1, "{job:?}");
    let n = job.len();
    s.link.send(Cmd::Job(job));
    s.wait("the pen change", 120.0, |v| !v.barriers.is_empty());
    assert!(s.seen.barriers[0].0.contains("Change the pen to: Red pen"), "{:?}", s.seen.barriers);
    // Everything before it was done: the plotter stands still and the job has not gone on.
    s.wait("idle at the pause", 60.0, |v| v.state == "Idle");
    assert!(s.seen.progress.0 < n, "{:?}", s.seen.progress);
    s.link.send(Cmd::Continue);
    s.wait("job finished", 120.0, |v| v.progress == (n, n));
    s.wait_idle_at(0.0, 0.0, 60.0);
    assert!(s.errors().is_empty(), "GRBL rejected lines: {:?}", s.errors_described());
}

#[test]
#[ignore = "needs grbl-sim: tools/grbl-sim/build.sh of baseCreator"]
fn grbl_sim_the_pen_change_goes_to_the_place_of_the_device_and_stops_there() {
    let mut s = Session::connect();
    s.wait_banner();
    s.fast_machine(&[]);
    s.link.send(Cmd::Barriers(true));
    let mut scene = Scene::default();
    square(&mut scene, 0, 20.0, 20.0, 10.0);
    square(&mut scene, 1, 60.0, 20.0, 10.0);
    let mut dev = Device::dummy();
    // A mechanism at (120, 150): the lines of the change are sent there, then the stop.
    dev.pen_change = crate::device::PenChange { enabled: true, x: 120.0, y: 150.0, pause: true, gcode: "G4 P0.1".into() };
    let job = job_of(&scene, &dev);
    assert!(job.iter().any(|l| l == "G0 X120 Y150"), "{job:?}");
    let n = job.len();
    s.link.send(Cmd::Job(job));
    s.wait("the pen change", 120.0, |v| !v.barriers.is_empty());
    assert!(s.seen.barriers[0].0.contains("Change the pen to: Red pen"), "{:?}", s.seen.barriers);
    // The plotter stands at the place of the change with everything before the stop done.
    s.wait_idle_at(120.0, 150.0, 60.0);
    assert!(s.seen.progress.0 < n, "{:?}", s.seen.progress);
    s.link.send(Cmd::Continue);
    s.wait("job finished", 120.0, |v| v.progress == (n, n));
    s.wait_idle_at(0.0, 0.0, 60.0);
    assert!(s.errors().is_empty(), "GRBL rejected lines: {:?}", s.errors_described());
}

#[test]
#[ignore = "needs grbl-sim: tools/grbl-sim/build.sh of baseCreator"]
fn grbl_sim_one_element_at_a_time_runs_with_the_same_pen() {
    let mut s = Session::connect();
    s.wait_banner();
    s.fast_machine(&[]);
    // Two groups, each a square and a smaller one inside it, on layers with the same pen (the layers are the same colour).
    let mut scene = Scene::default();
    for x in [20.0, 100.0] {
        let outer = scene.add(0, Kind::Rect { w: 30.0, h: 30.0 }, Xf::translate(x, 20.0));
        let inner = scene.add(0, Kind::Rect { w: 10.0, h: 10.0 }, Xf::translate(x + 10.0, 30.0));
        let g = scene.new_group_id();
        for sh in scene.shapes.iter_mut().filter(|sh| sh.id == outer || sh.id == inner) {
            sh.group = Some(g);
        }
    }
    let job = job_with(&scene, &Device::dummy(), JobOptions { one_by_one: true });
    assert!(!job.iter().any(|l| l.starts_with("M0")), "the same pen all through: no stop");
    s.run_job(job, 120.0);
    s.wait_idle_at(0.0, 0.0, 60.0);
    assert!(s.errors().is_empty(), "GRBL rejected lines: {:?}", s.errors_described());
}
