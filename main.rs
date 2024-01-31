mod rowops;
mod echelon;
mod rational;
mod matrix;

fn main() {
    use rational::{Rational, Sign};
    use matrix::VecMatrix;
    use echelon::reduced_row_echelon;
    let mut m: VecMatrix<Rational> = matrix::literal(&[
        [2, -1],
        [-8, 4]  
    ]);
    println!("{}", m);
    reduced_row_echelon(&mut m);
    println!("Complete!");
    println!("{}", m);    
}
