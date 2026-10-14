//! Renders the README screenshots headlessly (no window, no display needed).
//!
//! `cargo test -p pencreator render_screenshots -- --ignored --nocapture` writes PNG files into `docs/screenshots/`.
use crate::App;
use base_ui::editor::EditorCommands;
use egui_kittest::Harness;
use std::path::PathBuf;

type Shot = Harness<'static, Option<App>>;

fn save(h: &mut Shot, name: &str) {
    h.run_steps(5);
    let img = h.render().expect("render");
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/screenshots");
    std::fs::create_dir_all(&dir).expect("create screenshot dir");
    let p = dir.join(name);
    img.save(&p).expect("write png");
    println!("wrote {}", p.display());
}

/// A small badge: a frame in black, a blue circle, a red star and a filled black star.
fn demo_job(a: &mut App) {
    use base_core::bezier::Contour;
    use base_core::geom::Xf;
    use base_core::scene::Kind;
    a.new_design();
    a.scene.add(0, Kind::Rect { w: 150.0, h: 100.0 }, Xf::translate(70.0, 55.0));
    a.scene.add(2, Kind::Ellipse { w: 46.0, h: 46.0 }, Xf::translate(85.0, 70.0));
    a.scene.add(1, Kind::Bezier(vec![Contour::star(5, 0.45, 40.0, 40.0)]), Xf::translate(145.0, 68.0));
    a.scene.add(4, Kind::Bezier(vec![Contour::star(5, 0.45, 30.0, 30.0)]), Xf::translate(100.0, 118.0));
    a.scene.add(4, Kind::Rect { w: 70.0, h: 14.0 }, Xf::translate(140.0, 122.0));
    a.layers[0].name = "Frame".into();
    a.layers[4].hatch_gap = 0.8;
    a.description = "Badge, A4 paper, four pens.\nBlack frame first, then blue, red and the black fill.".into();
    a.touch_rev();
}

fn app(h: &mut Shot) -> &mut App {
    h.state_mut().as_mut().expect("app built")
}

#[test]
#[ignore = "writes docs/screenshots; run explicitly"]
fn render_screenshots() {
    let mut h: Shot = Harness::builder().with_size([1280.0, 800.0]).build_ui_state(
        |ui, state: &mut Option<App>| {
            let a = state.get_or_insert_with(|| App::build(ui.ctx(), None, false));
            a.draw(ui);
        },
        None,
    );
    h.run_steps(2);
    {
        let a = app(&mut h);
        a.devices.clear();
        let mut plotter = crate::device::Device::dummy();
        plotter.name = "A4 plotter".into();
        a.devices.push(plotter);
        a.use_dummy_device();
        a.start_sel = 0;
    }
    save(&mut h, "01-start.png");
    {
        let a = app(&mut h);
        a.start_add_device();
        let e = a.dev_edit.as_mut().expect("window");
        e.draft.name = "A3 plotter".into();
        e.draft.area.x = 420.0;
        e.draft.area.y = 297.0;
    }
    save(&mut h, "02-add-device.png");
    {
        let a = app(&mut h);
        let e = a.dev_edit.as_mut().expect("window");
        e.page = crate::device_ui::Page::Pen;
        e.draft.pen.lift = crate::device::Lift::Servo;
        e.draft.pen.up = 800.0;
        e.draft.pen.down = 300.0;
    }
    save(&mut h, "03-device-pen.png");
    {
        let a = app(&mut h);
        a.dev_edit = None;
        a.start_sel = 0;
        demo_job(a);
        a.select_all();
        a.status = "Badge: four pens in use".into();
    }
    save(&mut h, "04-editor-properties.png");
    app(&mut h).side_tab = crate::SideTab::Layers;
    app(&mut h).ed.active_layer = 4;
    save(&mut h, "05-layers.png");
    app(&mut h).side_tab = crate::SideTab::Device;
    save(&mut h, "06-device-tab.png");
    app(&mut h).side_tab = crate::SideTab::Clipart;
    save(&mut h, "07-clipart.png");
    app(&mut h).side_tab = crate::SideTab::Description;
    save(&mut h, "08-description.png");
    app(&mut h).side_tab = crate::SideTab::Layers;
    app(&mut h).open_preview();
    save(&mut h, "09-plot.png");
    {
        let a = app(&mut h);
        a.view.progress = 0.45;
    }
    save(&mut h, "10-plot-progress.png");
    {
        use base_ui::link::{ConsoleLine, Dir};
        let a = app(&mut h);
        let line = |dir, text: &str| ConsoleLine { dir, text: text.into(), poll: false };
        a.machine.console.clear();
        a.machine.console.push(ConsoleLine::info("Connected to /dev/ttyUSB0 @ 115200"));
        a.machine.console.push(line(Dir::Rx, "Grbl 1.1h ['$' for help]"));
        for (t, r) in [("$H", "ok"), ("G21 G90", "ok"), ("G0 Z5", "ok"), ("G0 X10 Y10", "ok")] {
            a.machine.console.push(line(Dir::Tx, t));
            a.machine.console.push(line(Dir::Rx, r));
        }
        a.show_plot = false;
        a.side_tab = crate::SideTab::Console;
    }
    save(&mut h, "11-console.png");
}
