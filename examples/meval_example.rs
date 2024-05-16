use meval::tokenizer::Token;
use meval::{Context, Expr};

#[derive(Debug)]
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
fn main() {
    let e: Expr = "a^x+1+sin(x)".parse().unwrap();
    println!("{:?}", e.to_vec());
    let f = e.bind2("a", "x").unwrap();

    println!("{}", f(1.0, 2.0));
    let mut function = Function::new("a^x+1".to_owned(), "test".to_owned());
    //dbg!(function);
    // eval
    let out = function.eval(1.0);
    println!("{}", out);

    // assign value to var "a"

    function.assign_value_to_var("a".to_owned(), 2.0);
    let out = function.eval(1.0);
    println!("{}", out)

    // eval
}
