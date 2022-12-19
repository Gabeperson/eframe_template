#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use std::collections::VecDeque;
use egui::{ScrollArea, TextStyle, FontId, Vec2};


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

struct TemplateApp {
    // Example stuff:

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    threadcount: String,
    width: String,
    vector: VecDeque<String>,
    links: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            threadcount: "20".to_owned(),
            width: "800".to_owned(),    
            vector: VecDeque::new(),
            links: String::new(),
        }
    }
}


impl TemplateApp {
    /// Called once before the first frame.
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { threadcount, width, vector, links} = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui


        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            let x = ui.style_mut();
            let mut fontid = FontId::default();
            fontid.size *= 2.;
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
                ui.text_edit_multiline(links);
            });
            

            //ui.add(egui::Slider::new(value, 1..=40).text("value"));
            if ui.button("Download").clicked() {
                todo!();
            }

        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                
            
            for text in (0..100).rev() {
                ui.label(text.to_string());


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