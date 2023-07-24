use egui::plot::{Bar, BarChart, Plot};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use rand::seq::SliceRandom;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn shuffle_vec(arr: &mut [isize]) -> Vec<isize> {
    let length = arr.len();

    for i in (1..length).rev() {
        let j: usize = (random() * (i + 1) as f64).floor() as usize;
        arr.swap(i, j);
    }

    arr.to_vec()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn shuffle_vec(arr: &mut Vec<isize>) {
    let mut rng = rand::thread_rng();
    arr.shuffle(&mut rng);
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    algorithm: usize,
    array_size: usize,
    speed: f32,
    #[serde(skip)]
    array: Vec<isize>, // has to be isize for wasm
    dark_mode: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            algorithm: 0,
            array_size: 32,
            speed: 5.0,
            array: (1..=32).collect(),
            dark_mode: true,
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            algorithm: selected,
            array_size,
            speed,
            array,
            dark_mode,
        } = self;

        #[cfg(not(target_arch = "wasm32"))]
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Sorting Algorithms");

            ui.separator();

            ui.radio_value(selected, 0, "Bubble Sort");
            ui.radio_value(selected, 1, "Selection Sort");
            ui.radio_value(selected, 2, "Insertion Sort");
            ui.radio_value(selected, 3, "Merge Sort");
            ui.radio_value(selected, 4, "Quick Sort");
            ui.radio_value(selected, 5, "Heap Sort");

            ui.separator();

            ui.label("Array Size");
            ui.separator();
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(array_size, 2..=256).text(""));
                if ui.button("Restore").clicked() {
                    *array_size = 32;
                }
            });

            ui.separator();

            ui.label("Speed");
            ui.separator();
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(speed, 1.0..=10.0).text(""));
                if ui.button("Restore").clicked() {
                    *speed = 5.0;
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("code on ");
                    ui.hyperlink_to("github", "https://github.com/ltsbt/sorting_visualizer");
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Run").clicked() {
                    // run
                }
                if ui.button("Randomize").clicked() {
                    shuffle_vec(array);
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let icon = if *dark_mode { "☀" } else { "🌙" };
                    if ui.button(icon).clicked() {
                        if *dark_mode {
                            ui.ctx().set_visuals(egui::Visuals::light());
                            *dark_mode = false;
                        } else {
                            ui.ctx().set_visuals(egui::Visuals::dark());
                            *dark_mode = true;
                        };
                    }
                });
            });

            let bars: Vec<Bar> = (*array
                .iter()
                .enumerate()
                .map(|(i, &height)| Bar::new(i as f64, height as f64))
                .collect::<Vec<Bar>>())
            .to_vec();

            Plot::new("Sorting Visualizer")
                .allow_drag(false)
                .allow_zoom(false)
                .show_axes([false; 2])
                .show_x(false)
                .show_y(false)
                .clamp_grid(true)
                .show(ui, |plot_ui| plot_ui.bar_chart(BarChart::new(bars)))
                .response
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
