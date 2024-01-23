use crate::rational::Rational;
use crate::matrix::VecMatrix;

pub fn scale(row: &mut [Rational], scalar: &Rational) {
    for cell in row.iter_mut() {
        cell.mul_inplace(&scalar);
    }
}

pub fn add(matrix: &mut VecMatrix<Rational>, dest_row_idx: usize, 
           src_row_idx: usize, scalar: &Rational) 
{
    for i in 0..matrix.colc() {
        let mut term = matrix[src_row_idx][i].clone();
        term.mul_inplace(scalar);
        matrix[dest_row_idx][i].add_inplace(term);
    }
}

pub fn swap(matrix: &mut VecMatrix<Rational>, a: usize, b: usize) {
    let mut temp: Vec<Rational> = Vec::with_capacity(matrix.colc());
    for i in 0..matrix.colc() {
        temp.push(matrix[a][i].clone());
    }
    for i in 0..matrix.colc() {
        matrix[a][i] = matrix[b][i].clone();
        matrix[b][i] = temp[i].clone();
    } 
}
