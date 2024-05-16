#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui::{Color32, Stroke};
use egui_plot::{Legend, Line, Plot, PlotPoints};
use meval::{Expr, Context};

use meval::tokenizer::Token;


#[derive(Debug, Clone)]
struct Function {
    name: String,
    expression: String,
    parsed_expression: Expr,
    vars_names: Vec<String>,
    vars_values: Vec<f64>, //bind_expression:
}
impl Function {
    pub fn new(expression: String, name: String) -> Self {
        let parsed_expression: Expr = expression.parse().expect("Could not parse the expression");
        let (vars_names, vars_values) = Self::extract_vars(&parsed_expression);
        Self {
            name,
            expression,
            parsed_expression,
            vars_names,
            vars_values,
        }
    }
    pub fn assign_value_to_var(&mut self, var_name: String, value: f64) {
        let index = self.vars_names.iter().position(|x| x == &var_name);
        match index {
            Some(i) => self.vars_values[i] = value,
            None => println!("Variable not found"),
        }
    }
    pub fn eval(&mut self, x: f64) -> f64 {
        // if it is slow, look at mapping only one var x like in the example
        // Apparently the library uses bind2, bind3, depending on the number of variables, investigate bindn
        let parsed_expression = self.parsed_expression.clone();
        match self.vars_names.len() {
            0 => {
                let f = parsed_expression.bind("x").unwrap();
                return f(x);
            }
            1 => {
                let f = parsed_expression
                    .bind2("x", &self.vars_names[0])
                    .unwrap();
                return f(x, self.vars_values[0]);
            }
            2 => {
                let f = 
                    parsed_expression
                    .bind3("x", &self.vars_names[0], &self.vars_names[1])
                    .unwrap();
                return f(x, self.vars_values[0], self.vars_values[1]);
            }
            3 => {
                let f = parsed_expression
                    .bind4(
                        "x",
                        &self.vars_names[0],
                        &self.vars_names[1],
                        &self.vars_names[2],
                    )
                    .unwrap();
                return f(
                    x,
                    self.vars_values[0],
                    self.vars_values[1],
                    self.vars_values[2],
                );
            }
            4 => {
                let f = parsed_expression
                    .bind5(
                        "x",
                        &self.vars_names[0],
                        &self.vars_names[1],
                        &self.vars_names[2],
                        &self.vars_names[3],
                    )
                    .unwrap();
                return f(
                    x,
                    self.vars_values[0],
                    self.vars_values[1],
                    self.vars_values[2],
                    self.vars_values[3],
                );
            }
            _ => panic!("Too many number of parameters, maximum supported is 4 + x"),
        }
    }
    pub fn extract_vars(parsed_expression: &Expr) -> (Vec<String>, Vec<f64>) {
        let vars = parsed_expression.to_vec();
        // keep only the names of Token::Var
        let mut vars_names: Vec<String> = vars
            .iter()
            .filter_map(|x| match x {
                Token::Var(name) => Some(name.to_string()),
                _ => None,
            })
            .collect();
        if !vars_names.contains(&"x".to_owned()) {
            println!("It cannot plot anything without an x var")
        }
        vars_names.retain(|element| element != "x");
        vars_names.sort();
        vars_names.dedup();
        // default var values to 1.0
        let vars_values: Vec<f64> = vars_names.iter().map(|_| 1.0).collect();
        return (vars_names, vars_values);
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 1000.0]),
        ..Default::default()
    };
    eframe::run_native("Plotting", options, Box::new(|cc| Box::<MyApp>::default()))
}

struct MyApp {
    new_function_name: String,
    elements: Vec<Function>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            new_function_name: "".to_owned(),
            elements: vec![
                Function::new("x^2+a".to_owned(), "x^2+a".to_owned()),
                Function::new("x^a+c^4".to_owned(), "x^a+c*4".to_owned()),
                Function::new("sin(x)*sin(x)*a+b*cos(x)".to_owned(), "sin(x)*sin(x)*a+b*cos(x)".to_owned()),

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
                                    for (i, name) in element.vars_names.iter().enumerate() {
                                        ui.add(
                                            egui::DragValue::new(&mut element.vars_values[i])
                                                .speed(0.1)
                                                .clamp_range(-10.0..=10.0)
                                                .prefix(format!("{}: ", name)),
                                        );
                                    }

    
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
                        self.elements.push(
                            Function::new(self.new_function_name.to_owned(),
                            self.new_function_name.to_owned(),
                        )
                        );
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
                                //let expr = fun);
                                // let f = element.assign_value_to_var("a", );
                                let x = t;
                                let y = element.clone().eval(x);
                                (x, y)
                            },
                            -10.0..=10.0,
                            500,
                        ))
                        .name(&element.name),
                    );
                }
            })
            .response
        });

        // egui::SidePanel::right("plot").show(ctx, |ui| {

        // });
    }
}

