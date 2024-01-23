mod rowops;
mod echelon;
mod rational;
mod matrix;

fn main() {
    use rational::{Rational, Sign};
    use matrix::VecMatrix;
    use echelon::reduced_row_echelon;
    let mut m: VecMatrix<Rational> = matrix::literal(&[
        [1, 0, -2, 3, -5, 6, 0],
        [0, 0, 1, 0, 0, 4, 0],
        [0, 0, 0, 0, 1, -4, 0],
        [0, 0, 0, 0, 0, 0, 0]
    ]);
    println!("{}", m);
    reduced_row_echelon(&mut m);
    println!("Complete!");
    println!("{}", m);    
}
