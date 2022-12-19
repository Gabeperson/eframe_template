#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use std::collections::VecDeque;
use egui::{ScrollArea, TextStyle, FontId, Vec2, FontDefinitions, FontData, FontFamily};




static LEFTPANELWIDTH: f32 = 400.;
struct TemplateApp {
    // Example stuff:

    // this how you opt-out of serialization of a member
    threadcount: String,
    width: String,
    links: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            threadcount: "20".to_owned(),
            width: "800".to_owned(),    
            links: String::new(),
        }
    }
}


impl TemplateApp {
    /// Called once before the first frame.
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let mut fonts = FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters):
        fonts.font_data.insert("my_font".to_owned(),
        FontData::from_static(include_bytes!("../font.ttf"))); // .ttf and .otf supported

        // Put my font first (highest priority):
        fonts.families.get_mut(&FontFamily::Proportional).unwrap()
            .insert(0, "my_font".to_owned());

        // Put my font as last fallback for monospace:
        fonts.families.get_mut(&FontFamily::Monospace).unwrap()
            .push("my_font".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { threadcount, width, links} = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.label("");
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.set_min_size(Vec2 {x: LEFTPANELWIDTH, y: 600.});
            let x = ui.style_mut();
            let mut fontid = FontId::default();
            fontid.size *= 1.5;
            x.override_font_id = Some(fontid);
            ui.heading("Config");

            ui.horizontal(|ui| {
                ui.label("Threads: ");
                ui.text_edit_singleline(threadcount);
            });
            ui.separator();
            
            ui.horizontal(|ui| {
                    ui.label("Width (px): ");
                    ui.text_edit_singleline(width);

            });
            ui.separator();
            ui.vertical(|ui| {
                ui.label("Links");
                let mut vec = ui.available_size();
                vec.y *= 0.9;
                ui.add_sized(vec, egui::TextEdit::multiline(links));
            });
            
            ui.separator();
            //ui.add(egui::Slider::new(value, 1..=40).text("value"));
            let mut size = Vec2::default();
            size.x = LEFTPANELWIDTH;
            if ui.add(egui::widgets::Button::new("Download").min_size(size)).clicked() {
                todo!();
            }

        });
        

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                
            
            for text in (0..100).rev() {
                ui.label("안녕하세요".to_owned() + &text.to_string()[..]);


                egui::warn_if_debug_build(ui);
        }
        });
    });

    }
}

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2 { x: 1200., y: 700. });
    
    eframe::run_native(
        "tk_oxidized",
        native_options,
        Box::new(|cc| Box::new(TemplateApp::new(cc))),
    );
}