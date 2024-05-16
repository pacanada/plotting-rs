use meval::{Expr, Context};
fn main(){

    let e: Expr = "a^x+1+sin(x)".parse().unwrap();
    println!("{:?}", e.to_vec());
    let f = e.bind2("a", "x").unwrap();

    println!("{}", f(1.0,2.0));

}