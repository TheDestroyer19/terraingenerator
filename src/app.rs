use egui::{Color32, Vec2, ColorImage};
use egui_extras::RetainedImage;

use crate::{map::Map, noise::simplex2d_octaves};

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq)]
enum View {
    Elevation,
    StandingWater,
    Composite,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    view: View,
    size: usize,
    //elevation noise inputs
    seed: i64,
    frequency: f64,
    amplitude: f32,
    octaves: u32,
    persistance: f32,
    //standing water inputs
    ocean_level: f32,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    elevation: Map<f32>,
    #[serde(skip)]
    standing_water: Map<f32>,
    #[serde(skip)]
    image: RetainedImage,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let size = 128;
        let elevation = Map::<f32>::new(size);
        let standing_water = Map::<f32>::new(size);
        let color_image = ColorImage::new([size, size], Color32::default());

        Self {
            view: View::Composite,
            seed: 12345,
            size,
            frequency: 16.0,
            amplitude: 256.0,
            octaves: 6,
            persistance: 0.8,
            ocean_level: 32.0,
            elevation,
            standing_water,
            image: RetainedImage::from_color_image("display", color_image),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            let mut loaded: Self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            loaded.generate();

            return loaded;
        }

        Default::default()
    }

    fn generate(&mut self) {
        let generator = simplex2d_octaves(
            self.seed,
            self.size,
            self.frequency,
            self.amplitude,
            self.octaves,
            self.persistance,
        );
        self.elevation = Map::with_generator(self.size, generator);

        crate::water::calculate_lakes(&self.elevation, &mut self.standing_water, self.ocean_level);

        self.update_display();
    }

    fn update_display(&mut self) {
        let elevation_func = |&v| Color32::from_gray(v as u8);
        let water_func = |&v| {
            let v = (v as f32 * 4.0).min(255.0) as u8;
            let iv = 255 - v;
            Color32::from_rgb(iv, iv, 255)
        };
        assert_eq!(self.size, self.elevation.size());
        assert_eq!(self.size, self.standing_water.size());

        let size = [self.size, self.size];
        let pixels: Vec<Color32> = match self.view {
            View::Elevation => self.elevation.values().map(elevation_func).collect(),
            View::StandingWater => self.standing_water.values()
                .map(water_func).collect(),
            View::Composite => self.elevation.values().zip(self.standing_water.values())
                .map(|(&e, &w)| {
                    let gray = e as u8;
                    let blue = (w * 4.0) as u8;
                    Color32::from_rgb(gray, gray, gray.saturating_add(blue))
                }).collect(),
        };

        assert_eq!(self.size * self.size, pixels.len());

        let color_image = ColorImage {
            size,
            pixels,
        };

        self.image = RetainedImage::from_color_image("preview", color_image);
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
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Settings");
            egui::Grid::new("settings").num_columns(2).show(ui, |ui| {
                ui.label("Seed");
                ui.add(egui::DragValue::new(&mut self.seed));
                ui.end_row();
                ui.label("size");
                ui.add(egui::DragValue::new(&mut self.size).clamp_range(16..=2048));
                ui.end_row();
            });
            ui.separator();
            ui.heading("Elevation settings");
            egui::Grid::new("settings-elevation")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("frequency");
                    ui.add(egui::DragValue::new(&mut self.frequency));
                    ui.end_row();
                    ui.label("amplitude");
                    ui.add(egui::DragValue::new(&mut self.amplitude).clamp_range(0.0..=256.0));
                    ui.end_row();
                    ui.label("octaves");
                    ui.add(egui::DragValue::new(&mut self.octaves).clamp_range(0..=16));
                    ui.end_row();
                    ui.label("persistance");
                    ui.add(
                        egui::DragValue::new(&mut self.persistance)
                            .clamp_range(0.0..=1.0)
                            .speed(0.05),
                    );
                    ui.end_row();
                });
            ui.separator();
            ui.heading("Water settings");
            egui::Grid::new("settings-water")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Ocean level");
                ui.add(egui::DragValue::new(&mut self.ocean_level));
                ui.end_row();
            });
            if ui.button("Generate").clicked() {
                self.generate();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(self.view == View::Elevation, "Elevation").clicked() {
                    self.view = View::Elevation;
                    self.update_display();
                }
                if ui.selectable_label(self.view == View::StandingWater, "Water").clicked() {
                    self.view = View::StandingWater;
                    self.update_display();
                }
                if ui.selectable_label(self.view == View::Composite, "Composite").clicked() {
                    self.view = View::Composite;
                    self.update_display();
                }
            });
            ui.image(self.image.texture_id(ctx), Vec2::new(512., 512.));

            egui::warn_if_debug_build(ui);
        });
    }
}
