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
pub(crate) fn demo_job(a: &mut App) {
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
    {
        let a = app(&mut h);
        a.machine.connected = true;
        a.machine.state = "Idle".into();
        a.machine.mpos = [-80.5, -12.25, 0.0];
        a.machine.wpos = [19.5, 37.75, 0.0];
        a.machine.pos = a.machine.wpos;
        a.machine.fw.info = base_core::firmware::FirmwareInfo { name: "Grbl".into(), version: "1.1h".into(), options: "V".into(), raw: vec!["[VER:1.1h.20190825:]".into(), "[OPT:V,15,128]".into()] };
        a.device.control.macros = vec![base_core::control::Macro { name: "Pen up".into(), code: "G0 Z[safez]".into() }, base_core::control::Macro { name: "Park".into(), code: "G0 Z[safez]\nG0 X0 Y0".into() }];
        a.side_tab = crate::SideTab::Control;
    }
    save(&mut h, "12-control-tab.png");
    {
        let a = app(&mut h);
        a.machine.fw.values = [(100, "80.000"), (101, "80.000"), (102, "80.000"), (110, "6000.000"), (111, "6000.000"), (112, "1000.000"), (130, "297.000"), (131, "210.000"), (132, "20.000")].into_iter().map(|(k, v)| (k, v.to_string())).collect();
        a.fw_open = true;
    }
    save(&mut h, "13-firmware.png");
    {
        let a = app(&mut h);
        a.fw_open = false;
        a.machine.barrier = Some(base_ui::machine_ctl::Barrier { text: "M0 (Change the pen to: Red pen, 0.4 mm)".into(), line: 40 });
    }
    save(&mut h, "14-job-paused.png");
}

#[cfg(test)]
mod context_menu_tests {
    use super::*;
    use egui::{Event, Modifiers, PointerButton};
    use base_ui::editor::EditorHost;
    use egui_kittest::kittest::Queryable;

    #[test]
    fn a_right_click_on_a_shape_opens_the_menu_and_its_commands_work() {
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
            a.use_dummy_device();
            a.screen = crate::Screen::Editor;
            a.new_design();
            a.scene.add(0, base_core::scene::Kind::Rect { w: 30.0, h: 30.0 }, base_core::geom::Xf::translate(40.0, 40.0));
            a.ed.view.need_fit = false;
        }
        h.run_steps(3);
        let at = {
            let a = app(&mut h);
            let o = a.ed.canvas_origin;
            a.w2s(o, base_core::geom::Pt::new(55.0, 55.0))
        };
        let button = |pressed| Event::PointerButton { pos: at, button: PointerButton::Secondary, pressed, modifiers: Modifiers::NONE };
        h.event(Event::PointerMoved(at));
        h.run_steps(2);
        h.event(button(true));
        h.run_steps(2);
        h.event(button(false));
        h.run_steps(4);
        assert_eq!(app(&mut h).ed.sel.len(), 1, "the shape under the pointer is selected");
        if let Ok(p) = std::env::var("SHOT") {
            h.render().unwrap().save(p).unwrap();
        }
        // (the submenus are looked at on the picture; their labels are not found by the test harness)
        // Duplicate is in the menu and works.
        h.get_by_label("Duplicate").click();
        h.run_steps(4);
        assert_eq!(app(&mut h).scene.shapes.len(), 2, "Duplicate from the context menu");
    }
}

#[cfg(test)]
mod trace_tests {
    use super::*;
    use base_ui::editor::{EditorHost, EditorImages};
    use base_ui::menu::StdAct;
    use egui_kittest::kittest::Queryable;

    /// The way a person does it: a picture is imported with the dialog, then traced into outlines with the Trace dialog.
    #[test]
    fn a_picture_is_imported_and_traced_into_outlines_through_the_dialogs() {
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
            a.use_dummy_device();
            a.screen = crate::Screen::Editor;
            a.new_design();
            // 40 x 40 pixels: a black square on white.
            let mut gray = vec![255u8; 40 * 40];
            for y in 10..30 {
                for x in 10..30 {
                    gray[y * 40 + x] = 0;
                }
            }
            let im = base_core::image::ImageData { name: "square".into(), px_w: 40, px_h: 40, gray, w: 40.0, h: 40.0, invert: false };
            a.open_import_dialog_with(im);
        }
        h.run_steps(4);
        h.get_by_label("Import").click();
        h.run_steps(4);
        assert_eq!(app(&mut h).scene.shapes.len(), 1, "the picture is on the page");
        assert!(app(&mut h).ed.sel.len() == 1, "and selected");
        // Trace image…: the button of the Properties tab (the menu command does the same).
        assert!(StdAct::TraceImage.enabled(app(&mut h)));
        h.get_by_label("Trace image…").click();
        h.run_steps(4);
        assert!(app(&mut h).ed.trace_dlg.is_some(), "the trace dialog opens");
        h.get_by_label("Trace").click();
        h.run_steps(4);
        let a = app(&mut h);
        assert!(a.ed.trace_dlg.is_none(), "the dialog closes");
        assert!(a.scene.shapes.len() >= 2, "outlines were added: {} shapes", a.scene.shapes.len());
        assert!(a.scene.shapes.iter().any(|s| matches!(s.kind, base_core::scene::Kind::Bezier(_))), "as curves");
    }
}

#[cfg(test)]
mod window_size_tests {
    use super::*;

    #[test]
    fn the_preview_keeps_its_size_and_does_not_grow_to_the_screen() {
        let mut h: Shot = Harness::builder().with_size([1600.0, 1000.0]).build_ui_state(
            |ui, state: &mut Option<App>| {
                let a = state.get_or_insert_with(|| App::build(ui.ctx(), None, false));
                a.draw(ui);
            },
            None,
        );
        h.run_steps(2);
        {
            let a = app(&mut h);
            a.use_dummy_device();
            a.screen = crate::Screen::Editor;
            demo_job(a);
            a.show_plot = true;
        }
        let mut seen = vec![];
        for _ in 0..14 {
            h.run_steps(1);
            seen.push(h.ctx.memory(|m| m.area_rect(egui::Id::new(Some("preview_plot")))).map(|r| (r.width(), r.height())));
        }
        let (first, last) = (seen[2], *seen.last().unwrap());
        assert!(last.is_some(), "the window is shown");
        assert_eq!(first, last, "the window keeps its size: {seen:?}");
    }
}
