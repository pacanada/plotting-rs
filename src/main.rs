#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui::{Color32, Stroke};
use egui_plot::{Legend, Line, Plot, PlotPoints};
use meval::{Expr, Context};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 1000.0]),
        ..Default::default()
    };
    eframe::run_native("Plotting", options, Box::new(|cc| Box::<MyApp>::default()))
}
struct Element {
    name: String,
    //type_: Elements,
    a: f64,
}
impl Element {
    pub fn get_expr(text: String)->Expr{
        let expr: Expr = text.parse().unwrap();
        expr
    }
}
enum Elements {
    Function,
    Parameter,
    List,
}

struct MyApp {
    new_function_name: String,
    elements: Vec<Element>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            new_function_name: "".to_owned(),
            elements: vec![
                Element {
                    name: "a*x+1".to_owned(),
                    a: 5.0,
                },
                Element {
                    name: "a^2+1".to_owned(),
                    a: 6.0,
                },
            ],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.heading("Elements");
            ui.vertical(|ui| {
                let mut id_to_remove = None;
                for (i, element) in self.elements.iter_mut().enumerate() {
                    let frame = egui::Frame::default()
                        .inner_margin(4.0)
                        .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY)); // Black border
                    frame.show(ui, |ui: &mut egui::Ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                if ui
                                    .button("⊗")
                                    .on_hover_text("Remove this function")
                                    .clicked()
                                {
                                    id_to_remove = Some(i);
                                }

                                //ui.label(element.name.clone());
                                ui.add(
                                    egui::TextEdit::singleline(&mut element.name)
                                        .desired_width(70.0),
                                );
                                // rename if clicked and typed
                                //ui.text_edit_singleline(&mut element.name);
                            });
                            // get id
                            let id = ui.make_persistent_id(i);
                            egui::CollapsingHeader::new("Parameters").id_source(id).show(ui, |ui| {
                                // ui.push_id("a", |ui| {
                                    ui.add(
                                        egui::DragValue::new(&mut element.a)
                                            .speed(0.1)
                                            .clamp_range(0.0..=10.0)
                                            .prefix("a: "),
                                    );
    
                            });
                            
                            // add some space between elements
                            ui.add_space(10.0);
                        })
                    });

                }
                // remove id_to_remove from elements
                if let Some(index) = id_to_remove {
                    self.elements.remove(index);
                }

                ui.horizontal(|ui| {
                    if ui
                        .button("➕")
                        .on_hover_text("Add a new function to the plot")
                        .clicked()
                    {
                        self.elements.push(Element {
                            name: self.new_function_name.to_owned(),
                            a: 0.0,
                        });
                        self.new_function_name = "".to_owned();
                    };
                    let label = ui.label("f(x): ");
                    ui.text_edit_singleline(&mut self.new_function_name)
                        .labelled_by(label.id);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let plot = Plot::new("custom_axes")
                .legend(Legend::default())
                .show_axes(true);

            // Show the plot with lines
            plot.show(ui, |plot_ui| {
                for element in &self.elements {
                    plot_ui.line(
                        Line::new(PlotPoints::from_parametric_callback(
                            |t| {
                                let expr = Element::get_expr(element.name.clone());
                                let f = expr.bind2("a", "x").expect("error binding");
                                let x = t;
                                let y = f(element.a, t);
                                (x, y)
                            },
                            0.0..=1.0,
                            1000,
                        ))
                        .name(&element.name),
                    );
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

