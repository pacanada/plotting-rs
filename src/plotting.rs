use eframe::egui;

use egui::{Color32, Stroke, Vec2b};
use egui_plot::{Legend, Line, Plot, PlotPoints};
use meval::{Context, Expr};

use meval::tokenizer::Token;

const DEFAULT_VALUE_PARAMETER: f64 = 1.0;
const DEFAULT_PARAMETERS_LIMIT: (f64, f64) = (-10.0, 10.0);

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
    pub fn reparse(&mut self){
        self.parsed_expression = self.expression.parse().expect("Could not parse the expression");
        let vars = Self::extract_vars(&self.parsed_expression);
        self.vars_names = vars.0;
        self.vars_values = vars.1;

    } 
    pub fn eval(&self, x: f64) -> f64 {
        // if it is slow, look at mapping only one var x like in the example
        // Apparently the library uses bind2, bind3, depending on the number of variables, investigate bindn
        let parsed_expression = self.parsed_expression.clone();
        match self.vars_names.len() {
            0 => {
                let f = parsed_expression.bind("x").unwrap();
                return f(x);
            }
            1 => {
                let f = parsed_expression.bind2("x", &self.vars_names[0]).unwrap();
                return f(x, self.vars_values[0]);
            }
            2 => {
                let f = parsed_expression
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
        let vars_values: Vec<f64> = vars_names.iter().map(|_| DEFAULT_VALUE_PARAMETER).collect();
        return (vars_names, vars_values);
    }
}

pub struct PlottingApp {
    new_function_name: String,
    new_parameter_name: String,
    functions: Vec<Function>,
    parameters_names: Vec<String>,
    parameters_values: Vec<f64>,
    parameters_lim: Vec<(f64,f64)>,
    xlim: (f64, f64),
    //ylim: (f64, f64)
}
impl PlottingApp {
    pub fn update_parameters_names_from_functions(&mut self) {
        let mut parameters_names = self.parameters_names.clone();
        let mut parameters_values = self.parameters_values.clone();
        let mut parameters_lim = self.parameters_lim.clone();

        for function in &self.functions {
            let (function_parameter_names, function_parameter_values) =
                Function::extract_vars(&function.parsed_expression);
            parameters_names.append(&mut function_parameter_names.clone());
            parameters_values.append(&mut function_parameter_values.clone());
            parameters_lim.push(DEFAULT_PARAMETERS_LIMIT);
        }
        parameters_names.sort();
        parameters_names.dedup();
        // TODO: how to get rid of duplicates names without removing duplicated values
        //parameters_values.sort();
        //parameters_names.dedup();

        self.parameters_names = parameters_names;
        self.parameters_values = parameters_values;
        self.parameters_lim = parameters_lim;
    }
    fn add_new_function(&mut self, new_function: Function) {
        self.functions.push(new_function);
        self.update_parameters_names_from_functions()
    }
}

impl Default for PlottingApp {
    fn default() -> Self {
        let f1 = Function::new("sin(x*a)*b".to_owned(), "f1".to_owned());
        let f2 = Function::new("cos(x*a)/b".to_owned(), "f2".to_owned());
        // let parameters_names = Self::get_parameters_names_from_functions(functions)
        let mut out = Self {
            new_function_name: "".to_owned(),
            new_parameter_name: "".to_owned(),
            xlim: (-10.0,10.0),
            //ylim: (-10.0,10.0),
            functions: vec![f1, f2],
            parameters_names: vec![],
            parameters_values: vec![],
            parameters_lim: vec![]
            };
        out.update_parameters_names_from_functions();
        out
    }
}

impl eframe::App for PlottingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //let idx_function_to_re
        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            let mut is_update_app_necessary = false;
            let mut show_parameter_name_warning: bool = false;
            ui.heading("Elements");
            ui.vertical(|ui| {
                let mut id_to_remove = None;
                for (i, function) in self.functions.iter_mut().enumerate() {
                    let frame = egui::Frame::default()
                        .inner_margin(4.0)
                        .stroke(egui::Stroke::new(1.0, egui::Color32::LIGHT_GRAY)); // Black border
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
                                    egui::TextEdit::singleline(&mut function.name)
                                        .desired_width(85.0),
                                );
                            });
                            // get id to avoid collisions based on same name
                            let id = ui.make_persistent_id(i);
                            egui::CollapsingHeader::new("Parameters")
                                .id_source(id)
                                .show(ui, |ui| {
                                    // ui.push_id("a", |ui| {
                                    // add edit of function
                                    ui.horizontal( |ui| {
                                        ui.label("f(x) = ");
                                    let modified_response = ui.add(
                                        egui::TextEdit::singleline(&mut function.expression)
                                            .desired_width(100.0),
                                    );
                                    if modified_response.clicked() {
                                        // re-create function
                                        function.reparse();
                                        is_update_app_necessary=true;
                                        //self.update_parameters_names_from_functions();

                                        //self.functions[i] = Function::new(function.expression.to_owned(), function.name.to_owned())
                                    }
                                });
                                    
                                    
                                    for (i, name) in function.vars_names.iter().enumerate() {
                                        ui.add(
                                            egui::DragValue::new(&mut function.vars_values[i])
                                                .speed(0.1)
                                                .clamp_range(self.parameters_lim[i].0..=self.parameters_lim[i].1)
                                                .prefix(format!("{}: ", name)),
                                        );
                                        //TODO:  should we this update the app vars?
                                    }
                                });

                            // add some space between elements
                            ui.add_space(10.0);
                        })
                    });
                }
                //let idx = 0;
                let mut param_id_to_remove = None;
                for (i, param_name) in self.parameters_names.iter().enumerate() {
                    // let mut_values = &mut self.parameters_values.unwrap();
                    let frame = egui::Frame::default()
                        .inner_margin(4.0)
                        .stroke(egui::Stroke::new(1.0, egui::Color32::LIGHT_BLUE)); // Black border
                    frame.show(ui, |ui: &mut egui::Ui| {
                        ui.horizontal(|ui| {

                        if ui
                                    .button("⊗")
                                    .on_hover_text("Remove this parameter")
                                    .clicked()
                                {
                                    param_id_to_remove = Some(i);
                                }
                    ui.add(
                        egui::DragValue::new(&mut self.parameters_values[i])
                            .speed(0.1)
                            .clamp_range(self.parameters_lim[i].0..=self.parameters_lim[i].1)
                            .prefix(format!("{}: ", param_name)),
                    )});
                    ui.vertical(|ui| {
                    let id = ui.make_persistent_id(i+100);
                    egui::CollapsingHeader::new("Parameters limits")
                        .id_source(id)
                        .show(ui, |ui| {
                            // ui.push_id("a", |ui| {
                            ui.horizontal(|ui| {
                                ui.add(
                                    egui::DragValue::new(&mut self.parameters_lim[i].0)
                                        .speed(0.1)
                                        .speed(0.1)
                                        .clamp_range(f64::NEG_INFINITY..=f64::INFINITY)
                                );
                                ui.label(format!("≤ {} ≤ ", self.parameters_names[i]));
                                //ui.wrap_text()
                                ui.add(
                                    egui::DragValue::new(&mut self.parameters_lim[i].1)
                                        .speed(0.1)
                                        .clamp_range(f64::NEG_INFINITY..=f64::INFINITY)
                                        
                                );
        
                                
                            }

                            )
                        });
                });
                });
                    // for all the functions, modify the var name values
                    for function in self.functions.iter_mut() {
                        if function.vars_names.contains(param_name) {
                            function.assign_value_to_var(
                                param_name.to_owned(),
                                self.parameters_values[i],
                            )
                        }
                    }
                }

                // remove id_to_remove from elements
                if let Some(index) = id_to_remove {
                    self.functions.remove(index);
                }
                if let Some(index) = param_id_to_remove {
                    self.parameters_names.remove(index);
                    self.parameters_values.remove(index);
                }
                if is_update_app_necessary {
                    self.update_parameters_names_from_functions();
                }
                
                ui.horizontal(|ui| {
                    if ui
                        .button("➕")
                        .on_hover_text("Add a new function to the plot")
                        .clicked()
                    {
                        self.add_new_function(Function::new(
                            self.new_function_name.to_owned(),
                            self.new_function_name.to_owned(),
                        ));
                        self.new_function_name = "".to_owned();
                    };
                    let label = ui.label("f(x): ");
                    ui.text_edit_singleline(&mut self.new_function_name)
                        .labelled_by(label.id);
                });
                ui.horizontal(|ui| {
                    // parameter
                    if ui
                        .button("➕")
                        .on_hover_text("Add a new parameter")
                        .clicked()
                    {
                        if self.parameters_names.contains(&self.new_parameter_name) | show_parameter_name_warning{
                            // TODO: warning only showing first time
                            show_parameter_name_warning = true;
                            ui.colored_label(egui::Color32::RED, format!("{} is already defined, choose another name.", &self.new_parameter_name));
        
                        } else {
                            self.parameters_names.push(self.new_parameter_name.to_owned());
                            self.parameters_values.push(DEFAULT_VALUE_PARAMETER);
                            self.parameters_lim.push(DEFAULT_PARAMETERS_LIMIT);
                            self.new_parameter_name = "".to_owned();
                            show_parameter_name_warning = false;

                        }
                    };
                    let label = ui.label("Param: ");
                    ui.text_edit_singleline(&mut self.new_parameter_name)
                        .labelled_by(label.id);
                });
            });
        });
        //let frame = egui::Frame::default().inner_margin(40.0).outer_margin(50.0);

        egui::CentralPanel::default().show(ctx, |ui| {
            let plot = Plot::new("custom_axes")
                //.width(100.0)
                .legend(Legend::default())
                .show_axes(true)
                //.view_aspect(1.0)
                .auto_bounds(Vec2b::new(false, false))
                .allow_double_click_reset(false);


            plot.show(ui, |plot_ui| {
                let plot_bounds = plot_ui.plot_bounds();
                //plot_ui.set_plot_bounds(plot_bounds)
                self.xlim.0 = plot_bounds.min()[0];
                self.xlim.1 = plot_bounds.max()[0];
                for function in &self.functions {
                    plot_ui.line(
                        Line::new(PlotPoints::from_parametric_callback(
                            |t| {
                                let x = t;
                                let y = function.eval(t);
                                (x, y)
                            },
                            // make the plot adaptative to reduce number of points for high resolution. We can use also from_explicit_callback but I can figure out the static requirements
                            self.xlim.0..=self.xlim.1,
                            500,
                        ))
                        .name(&function.name),
                    );
                }

            })
            .response
        });

        // egui::SidePanel::right("plot").show(ctx, |ui| {

        // });
    }
}
