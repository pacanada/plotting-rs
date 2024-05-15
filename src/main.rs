use cas::{self, Expression};



fn main() {
    let a = cas::Operator::Add;
    println!("{:?}", a);

    let exp = Expression::new("x^2 + 3*x + 1+a".to_owned());
    Expression::print_tree(&exp, 0);
}