use std::collections::VecDeque;

use crate::algorithms;
use egui::plot::{Bar, BarChart, Plot};
#[cfg(not(target_arch = "wasm32"))]
use rand::seq::SliceRandom;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
    #[wasm_bindgen(js_namespace = Date, js_name = now)]
    fn now() -> f64;
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
    now()
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
    sorted: bool,
    #[serde(skip)]
    running: bool,
    #[serde(skip)]
    arr_steps: VecDeque<algorithms::Step>,
    #[cfg(not(target_arch = "wasm32"))]
    #[serde(skip)]
    last_sort_time: std::time::Instant,
    #[cfg(target_arch = "wasm32")]
    #[serde(skip)]
    last_sort_time: f64,
    #[serde(skip)]
    selected_bars: Vec<usize>,
    #[serde(skip)]
    green_bars: Vec<usize>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            algorithm: 0,
            array_size: 32,
            speed: 5.0,
            array: (1..=32).collect(),
            dark_mode: true,
            sorted: true,
            running: false,
            arr_steps: VecDeque::new(),
            last_sort_time: get_time(),
            selected_bars: Vec::new(),
            green_bars: Vec::new(),
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            let settings: TemplateApp =
                eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();

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
            sorted,
            running,
            arr_steps,
            last_sort_time,
            selected_bars,
            green_bars,
        } = self;

        if array_size != &array.len() {
            *array = (1..=*array_size as isize).collect();
            if !*sorted {
                shuffle_vec(array);
            }
        }

        if *running {
            let now = get_time();
            #[cfg(not(target_arch = "wasm32"))]
            let elapsed = now.duration_since(*last_sort_time).as_secs_f64() * 1000.0;
            #[cfg(target_arch = "wasm32")]
            let elapsed = now - *last_sort_time;

            let speed_factor = 0.50132 * (1.5 * 1.9947_f32).powf(*speed);
            let updates_per_second = *speed * speed_factor;
            let loops_per_update = (updates_per_second / 60.0).ceil() as usize;

            if elapsed > (1000.0 / updates_per_second) as f64 {
                for _ in 0..loops_per_update {
                    selected_bars.clear();
                    green_bars.clear();

                    let step = arr_steps.pop_front().unwrap();

                    match step {
                        algorithms::Step::Compare(i, j) => {
                            selected_bars.push(i);
                            selected_bars.push(j);
                        }
                        algorithms::Step::Swap(i, j) => {
                            array.swap(i, j);
                            green_bars.push(i);
                            green_bars.push(j);
                        }
                        algorithms::Step::Merge(start, mid, end) => {
                            let mut left = array[start..=mid].to_vec();
                            let mut right = array[mid + 1..=end].to_vec();
                            let mut merged = vec![];

                            left.push(isize::MAX);
                            right.push(isize::MAX);

                            let mut i = 0;
                            let mut j = 0;

                            for k in start..=end {
                                if left[i] <= right[j] {
                                    merged.push(left[i]);
                                    i += 1;
                                } else {
                                    merged.push(right[j]);
                                    j += 1;
                                }
                                array[k] = merged[k - start];
                            }
                        }
                    }

                    *last_sort_time = now;
                    if arr_steps.is_empty() {
                        *running = false;
                        *sorted = true;
                        break;
                    }
                }
            }

            ctx.request_repaint();
        }

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Sorting Algorithms");

            ui.separator();

            // make radio buttons disabled if running
            ui.add_enabled_ui(!*running, |ui| {
                if ui.radio_value(algorithm, 0, "Bubble Sort").clicked() && *array_size > 1024 {
                    *array_size = 1024;
                }
                if ui.radio_value(algorithm, 1, "Selection Sort").clicked() && *array_size > 1024 {
                    *array_size = 1024;
                }
                if ui.radio_value(algorithm, 2, "Insertion Sort").clicked() && *array_size > 1024 {
                    *array_size = 1024;
                }

                ui.radio_value(algorithm, 3, "Merge Sort");
                ui.radio_value(algorithm, 4, "Quick Sort");

                ui.add_enabled_ui(false, |ui| {
                    ui.radio_value(algorithm, 5, "Heap Sort");
                });

                ui.separator();

                ui.label("Array Size");
                ui.separator();
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Slider::new(array_size, 2..=if *algorithm < 3 { 1024 } else { 4096 })
                            .text(""),
                    );

                    if ui.button("Restore").clicked() {
                        *array_size = 32;
                        *array = (1..=*array_size as isize).collect();
                        if !*sorted {
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
                    if ui.button("Run").clicked() && !*sorted {
                        *running = true;
                        selected_bars.clear();

                        let mut new_steps: VecDeque<algorithms::Step> = VecDeque::new();
                        match *algorithm {
                            0 => algorithms::bubble_sort(
                                array.to_owned().as_mut_slice(),
                                &mut new_steps,
                            ),
                            1 => algorithms::selection_sort(
                                array.to_owned().as_mut_slice(),
                                &mut new_steps,
                            ),
                            2 => algorithms::insertion_sort(
                                array.to_owned().as_mut_slice(),
                                &mut new_steps,
                            ),
                            3 => algorithms::merge_sort(
                                array.to_owned().as_mut_slice(),
                                0,
                                array.len() - 1,
                                &mut new_steps,
                            ),
                            4 => algorithms::quick_sort(
                                array.to_owned().as_mut_slice(),
                                &mut new_steps,
                            ),
                            5 => {}
                            _ => (),
                        }

                        *arr_steps = new_steps;
                    }

                    if ui.button("Shuffle").clicked() {
                        shuffle_vec(array);
                        *sorted = false;
                    }
                });

                ui.add_enabled_ui(*running, |ui| {
                    if ui.button("Stop").clicked() {
                        *running = false;
                    }
                });

                ui.add_enabled_ui(!*sorted && !*running, |ui| {
                    if ui.button("Clear").clicked() {
                        *array = (1..=*array_size as isize).collect();
                        *sorted = true;
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

            // Colors
            let red = egui::Color32::from_rgb(255, 0, 0);
            let red_light = egui::Color32::from_rgb(255, 100, 100);
            let green = egui::Color32::from_rgb(0, 255, 0);
            let green_light = egui::Color32::from_rgb(100, 255, 100);
            let black = egui::Color32::from_rgb(0, 0, 0);
            let black_light = egui::Color32::from_rgb(100, 100, 100);
            // let white = egui::Color32::from_rgb(255, 255, 255);
            let white_light = egui::Color32::from_rgb(200, 200, 200);

            let stroke_width = 1.5;

            for bar in bars.iter_mut() {
                bar.fill = if *dark_mode { black_light } else { white_light };
                bar.stroke.color = if *dark_mode { white_light } else { black };
                bar.stroke.width = stroke_width;
            }

            if *running {
                for bar in selected_bars.iter() {
                    bars[*bar].fill = red_light;
                    bars[*bar].stroke.color = red;
                }
                for bar in green_bars.iter() {
                    bars[*bar].fill = green_light;
                    bars[*bar].stroke.color = green;
                }
            }

            Plot::new("Sorting Visualizer")
                .allow_drag(false)
                .allow_zoom(false)
                .allow_scroll(false)
                .show_axes([false; 2])
                .show_x(false)
                .show_y(false)
                .clamp_grid(true)
                .show(ui, |plot_ui| plot_ui.bar_chart(BarChart::new(bars)))
                .response
        });
    }
}
