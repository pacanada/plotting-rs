#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Pow,
}
impl Operator {
    fn from_string(string: char) -> Self {
        match string {
            '+' => Self::Add,
            '-' => Self::Subtract,
            '*' => Self::Multiply,
            '/' => Self::Divide,
            '^' => Self::Pow,
            _ => panic!("Invalid operator"),
        }
    }
    fn to_char(&self) -> char {
        match self {
            Self::Add => '+',
            Self::Subtract => '-',
            Self::Multiply => '*',
            Self::Divide => '/',
            Self::Pow => '^',
        }
    }
    fn get_operator_list() -> Vec<char> {
        vec!['+', '-', '*', '/', '^']
    }

}

pub struct Expression {
    value: String,
    children: Option<Vec<Expression>>,
    operator: Option<Operator>
    
}
impl Expression {
    pub fn new(string_expression: String) -> Self {
        // remove spaces
        let string_expression = string_expression.replace(" ", "");
        let (index, op) = Expression::get_index_and_operator_char(&string_expression);
        if index.is_none() {
            return Self {
                value: string_expression.clone(),
                children: None,
                operator: None
            };
        }
        let (lh, rh) = string_expression.split_at(index.expect("something went worng"));
        let cropped_rh = &rh[1..];
        println!(" ({})     {}    ({})", lh, op.expect("a"), &rh[1..]);
        Self {
            value: string_expression.clone(),
            children: Some(vec![Expression::new(lh.to_owned()), Expression::new(cropped_rh.to_owned()) ]),
            operator: Some(Operator::from_string(op.expect("could not parse operator"))),
        }
    }
    fn get_index_and_operator_char(string_expression: &String) -> (Option<usize>, Option<char>) {
        let mut index: Option<usize> = None;
        let mut operator_char: Option<char> = None;

        for (i, c) in string_expression.chars().enumerate() {
            if Operator::get_operator_list().contains(&c) {
                index = Some(i);
                operator_char = Some(c);
                break;
            }
        }
        return (index, operator_char);
    }
    pub fn print_tree(expression: &Expression, depth: usize) {
        // Create an indentation string based on the current depth
        let indent = "  ".repeat(depth*2);
    
        if let Some(operator) = &expression.operator {
            println!(" {}{}", indent, operator.to_char());
        }
    
        if let Some(children) = &expression.children {

            // Print the branch and left child with increased indentation
            println!("{}|  \\", indent);
            println!("{}{}  {}", indent, children[0].value, children[1].value);

            // Recursively call print_tree on the right child, increasing the depth
            Expression::print_tree(&children[1], depth + 1);
            }
        }
    
    
}


// pub struct Expression {
//     current: Option<String>,
//     left: Option<Box<Expression>>, // also principal
//     right: Option<Box<Expression>>,
//     operator: Option<Operator>,
//     value: Option<String>,
// }
// impl Expression {
//     pub fn new(string_expression: String) -> Self {
//         // remove spaces
//         let string_expression = string_expression.replace(" ", "");
//         let (index, op) = Expression::get_index_and_operator_char(&string_expression);
//         if index.is_none() {
//             return Self {
//                 current: Some(string_expression.clone()),
//                 left: None,
//                 right: None,
//                 operator: None,
//                 value: Some(string_expression),
//             };
//         }
//         let (lh, rh) = string_expression.split_at(index.expect("something went worng"));
//         let cropped_rh = &rh[1..];
//         println!(" ({})     {}    ({})", lh, op.expect("a"), &rh[1..]);
//         Self {
//             current: Some(string_expression.clone()),
//             left: Some(Box::new(Expression::new(lh.to_owned()))),
//             right: Some(Box::new(Expression::new(cropped_rh.to_owned()))),
//             operator: Some(Operator::from_string(op.expect("could not parse operator"))),
//             value: None,
//         }
//     }
//     fn get_index_and_operator_char(string_expression: &String) -> (Option<usize>, Option<char>) {
//         let mut index: Option<usize> = None;
//         let mut operator_char: Option<char> = None;

//         for (i, c) in string_expression.chars().enumerate() {
//             if Operator::get_operator_list().contains(&c) {
//                 index = Some(i);
//                 operator_char = Some(c);
//                 break;
//             }
//         }
//         return (index, operator_char);
//     }
//     pub fn print_tree(expresion: Expression) {
//         //if let Some(operator) = expresion.operator;
//         if expresion.operator.is_some(){
//             println!("  {}", expresion.operator.expect("failed").to_char());
//         }
//         if expresion.left.is_some(){
//             if expresion.right.is_some(){
//                 println!("|    \\");
//                 println!("{}  {}", expresion.left.expect("").current.expect(""), expresion.right.expect("").current.expect(""));
//                 //let right =;
//                 match expresion.right {
//                     Some(exp_right) => Expression::print_tree(*exp_right),
//                     None => return
                    
//                 };
                
//             }

//         }

        

//     }
// }
