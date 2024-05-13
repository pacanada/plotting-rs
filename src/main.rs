#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui::{Color32, Stroke};
use egui_plot::{Legend, Line, Plot, PlotPoints};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native("Plotting", options, Box::new(|cc| Box::<MyApp>::default()))
}

struct MyApp {
    name: String,
    a: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            a: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal( |ui|{
            ui.add(egui::Slider::new(&mut self.a, 0..=120).text("a"));
            ui.add(egui::Slider::new(&mut self.a, 0..=120).text("b"));
            });
            let plot = Plot::new("custom_axes").legend(Legend::default());

            // Show the plot with lines
            plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::from_parametric_callback(
                    |t| {
                        let x = t;
                        let y = t.powi(self.a as i32);
                        (x, y)
                    },
                    0.0..=1.0,
                    1000,
                )));
                plot_ui.line(Line::new(PlotPoints::from_explicit_callback(
                    |x| x.powf(-1.0),
                    0.0..=1.0,
                    1000,
                )));
                plot_ui.line(
                    Line::new(PlotPoints::from_ys_f32(&vec![1.0, 2.0, 3.0, 2.0, 1.5]))
                        .color(Color32::RED)
                        .stroke(Stroke::new(2.0, Color32::RED))
                        .name("Explicit"),

                );
            })
            .response
        });
    }
}
