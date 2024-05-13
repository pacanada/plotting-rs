#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui::{Color32, Stroke};
use egui_plot::{Legend, Line, Plot, PlotPoints};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 1000.0]),
        ..Default::default()
    };
    eframe::run_native("Plotting", options, Box::new(|cc| Box::<MyApp>::default()))
}
struct Element {
    name: String,
    a: f64,

}
enum Elements {
    Function,
    Parameter,
}

struct MyApp {
    new_function_name: String,
    elements: Vec<Element>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            new_function_name: "".to_owned(),
            elements: vec![Element {
                name: "Function 1".to_owned(),
                a: 42.0,
            },
            Element {
                name: "function 2".to_owned(),
                a: 12.0,
            }],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //let mut  new_function: String = "".to_owned();
        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.heading("Elements");
            ui.vertical( |ui|{
                for element in self.elements.iter_mut(){
                    ui.horizontal(|ui| {
                        ui.label(element.name.clone());
                        ui.add(egui::Slider::new(&mut element.a, 0.0..=20.0).step_by(0.001).text("a"));
                    });
                }
            // if clicked in add new function

            ui.horizontal( |ui| {
                
                if ui.button("Add new function").on_hover_text("Add a new function to the plot").clicked() {
                    self.elements.push(Element {
                        name: self.new_function_name.to_owned(),
                        a: 0.0,
                        });
                };
                let label = ui.label("New function: ");
                ui.text_edit_singleline(&mut self.new_function_name).labelled_by(label.id);


            });
            });
            
            
            
            

                

        });
    

        egui::CentralPanel::default().show(ctx, |ui| {
            let plot = Plot::new("custom_axes").legend(Legend::default());

            // Show the plot with lines
            plot.show(ui, |plot_ui| {
                for element in &self.elements {
                    plot_ui.line(Line::new(PlotPoints::from_parametric_callback(
                        |t| {
                            let x = t;
                            let y = t.powf(element.a);
                            (x, y)
                        },
                        0.0..=1.0,
                        1000,
                    )));
                }
                // plot_ui.line(Line::new(PlotPoints::from_parametric_callback(
                //     |t| {
                //         let x = t;
                //         let y = t.powi(self.a as i32);
                //         (x, y)
                //     },
                //     0.0..=1.0,
                //     1000,
                // )));
                // plot_ui.line(Line::new(PlotPoints::from_explicit_callback(
                //     |x| x.powf(-1.0),
                //     0.0..=1.0,
                //     1000,
                // )));
                // plot_ui.line(
                //     Line::new(PlotPoints::from_ys_f32(&vec![1.0, 2.0, 3.0, 2.0, 1.5]))
                //         .color(Color32::RED)
                //         .stroke(Stroke::new(2.0, Color32::RED))
                //         .name("Explicit"),

                // );
            })
            .response
        });


        // egui::SidePanel::right("plot").show(ctx, |ui| {
            
        // });
    }
}
