use crate::algorithms;
use egui::plot::{Bar, BarChart, Plot};
// use std::sync::{Arc, Mutex};
#[cfg(not(target_arch = "wasm32"))]
use rand::seq::SliceRandom;

#[cfg(target_arch = "wasm32")]
use eframe::web_sys::window;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

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

#[cfg(not(target_arch = "wasm32"))]
pub fn get_time() -> std::time::Instant {
    std::time::Instant::now()
}

#[cfg(target_arch = "wasm32")]
pub fn get_time() -> f64 {
    let window = window().expect("should have a Window");
    let performance = window.performance().expect("should have a Performance");
    performance.now()
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
    #[serde(skip)]
    solved: bool,
    #[serde(skip)]
    running: bool,
    #[serde(skip)]
    arr_steps: Vec<isize>,
    #[serde(skip)]
    arr_current_step: usize,
    #[cfg(not(target_arch = "wasm32"))]
    #[serde(skip)]
    last_sort_time: std::time::Instant,
    #[cfg(target_arch = "wasm32")]
    #[serde(skip)]
    last_sort_time: f64,
    selected_bar: usize,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            algorithm: 0,
            array_size: 32,
            speed: 5.0,
            array: (1..=32).collect(),
            dark_mode: true,
            solved: true,
            running: false,
            arr_steps: Vec::new(),
            arr_current_step: 0,
            last_sort_time: get_time(),
            selected_bar: 0,
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            let settings: TemplateApp =
                eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            // set style
            if !settings.dark_mode {
                cc.egui_ctx.set_visuals(egui::Visuals::light());
            }

            return settings;
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
            algorithm,
            array_size,
            speed,
            array,
            dark_mode,
            solved,
            running,
            arr_steps,
            arr_current_step,
            last_sort_time,
            selected_bar,
        } = self;

        if array_size != &array.len() {
            *array = (1..=*array_size as isize).collect();
            if !*solved {
                shuffle_vec(array);
            }
        }

        if *running {
            let now = get_time();
            #[cfg(not(target_arch = "wasm32"))]
            let elapsed = now.duration_since(*last_sort_time).as_secs_f64();
            #[cfg(target_arch = "wasm32")]
            let elapsed = now - *last_sort_time;

            let speed_factor = 0.50132 * 1.9947_f32.powf(*speed);
            let updates_per_second = *speed * speed_factor;
            let loops_per_update = (updates_per_second / 60.0).ceil() as usize;

            let mut temp_selected_bar = 0;
            if elapsed > (1.0 / updates_per_second) as f64 {
                for _ in 0..loops_per_update {
                    let j = arr_steps[*arr_current_step] as usize;

                    match *algorithm {
                        0 => algorithms::bubble_sort(array, j),
                        1 => {}
                        _ => (),
                    }

                    *last_sort_time = now;
                    temp_selected_bar = j + 1;
                    if *arr_current_step < arr_steps.len() - 1 {
                        *arr_current_step += 1;
                    } else {
                        *running = false;
                        *solved = true;
                        *arr_current_step = 0;
                    }
                }
                *selected_bar = temp_selected_bar;
            }

            ctx.request_repaint();
        }

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Sorting Algorithms");

            ui.separator();

            // make radio buttons disabled if running
            ui.add_enabled_ui(!*running, |ui| {
                ui.radio_value(algorithm, 0, "Bubble Sort");

                ui.add_enabled_ui(false, |ui| {
                    ui.radio_value(algorithm, 1, "Selection Sort");
                    ui.radio_value(algorithm, 2, "Insertion Sort");
                    ui.radio_value(algorithm, 3, "Merge Sort");
                    ui.radio_value(algorithm, 4, "Quick Sort");
                    ui.radio_value(algorithm, 5, "Heap Sort");
                });

                ui.separator();

                ui.label("Array Size");
                ui.separator();
                ui.horizontal(|ui| {
                    ui.add(egui::Slider::new(array_size, 2..=256).text(""));

                    if ui.button("Restore").clicked() {
                        *array_size = 32;
                        *array = (1..=*array_size as isize).collect();
                        if !*solved {
                            shuffle_vec(array);
                        }
                    }
                });
                ui.separator();
            });

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
                ui.add_enabled_ui(!*running, |ui| {
                    if ui.button("Run").clicked() && !*solved {
                        *running = true;
                        *selected_bar = 0;
                        let mut new_steps = Vec::new();
                        match *algorithm {
                            0 => {
                                for i in 0..array.len() - 1 {
                                    for j in 0..array.len() - i - 1 {
                                        new_steps.push(j as isize);
                                    }
                                }
                            }
                            _ => (),
                        }

                        *arr_steps = new_steps;
                        *arr_current_step = 0;
                    }

                    if ui.button("Randomize").clicked() {
                        shuffle_vec(array);
                        *solved = false;
                    }
                });

                ui.add_enabled_ui(*running, |ui| {
                    if ui.button("Stop").clicked() {
                        *running = false;
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("🗙").clicked() {
                        _frame.close();
                    }
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

            let mut bars: Vec<Bar> = (*array
                .iter()
                .enumerate()
                .map(|(i, &height)| Bar::new(i as f64, height as f64))
                .collect::<Vec<Bar>>())
            .to_vec();

            // set selected bar color to red
            if *running {
                bars[*selected_bar].fill = egui::Color32::from_rgb(255, 0, 0);
            }

            let color = if *dark_mode {
                egui::Color32::from_rgb(255, 255, 255)
            } else {
                egui::Color32::from_rgb(0, 0, 0)
            };

            Plot::new("Sorting Visualizer")
                .allow_drag(false)
                .allow_zoom(false)
                .allow_scroll(false)
                .show_axes([false; 2])
                .show_x(false)
                .show_y(false)
                .clamp_grid(true)
                .show(ui, |plot_ui| {
                    plot_ui.bar_chart(BarChart::new(bars).color(color))
                })
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
