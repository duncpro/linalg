use crate::matrix::VecMatrix;
use crate::rational::Rational;

/// Computes the row echelon form of the matrix in-place.
/// Recall the properties of a matrix in row-echelon form...
/// - All rows having only zero entries occur subsequent to all rows having
///   at least one non-zero entry.
/// - The leading entry (the leftmost nonzero entry) of every nonzero row,
///   also called the pivot of a row, occurs in a column subsequent to 
///   the column of the leading entry of the last nonzero row.
pub fn row_echelon(matrix: &mut VecMatrix<Rational>) {
    // This algorithm starts with the leftmost topmost cell in the matrix
    // and moves down and right.
    let mut i: usize = 0;
    let mut j: usize = 0;
    
    while i < matrix.rowc() && j < matrix.colc() {
        // If this cell has a value of zero it cannot be used to zero
        // the rows below it. Thus, it is swapped for a row which has a nonzero
        // entry in this same column.    
        if matrix[i][j].numerator() == 0 {
            let mut nonzero_row: Option<usize> = None;
            
            'find_swap: 
            for k in (i + 1)..matrix.rowc() {
                if matrix[k][j].numerator() != 0 {
                    nonzero_row = Some(k);
                    break 'find_swap;
                }
            }
            
            if let Some(nonzero_row) = nonzero_row {
                crate::rowops::swap(matrix, i, nonzero_row);
                println!("Swap #{} and #{}", i, nonzero_row);
                println!("{}", matrix);
            } else {
                // All rows have a value of zero in this column.
                // This rows pivot must then be in the subsequent column.
                // Advance the cursor one column to the right.
                i += 1;
            }
            continue;
        }

        // Now that it is assured the (i, j) cell contains a nonzero value,
        // it can be used to zero all the entries below it. Recall that this
        // is a requirement since the pivot of subsequent rows **must** occur
        // right to the pivot of the previous rows.

        {
            let minv = Rational::multiplicative_inverse(&matrix[i][j]);     
            crate::rowops::scale(&mut matrix[i], &minv); 
            println!("Scale #{} by {}", i, minv);
            println!("{}", matrix);
        }
        
        for k in (i + 1)..matrix.rowc() {
            if matrix[k][j].numerator() == 0 { continue; }
            let scalar = Rational::additive_inverse(&matrix[k][j]);
            crate::rowops::add(matrix, k, i, &scalar);
            
            println!("Add: {} * #{} -> {}", scalar, i, k);
            println!("{}", matrix);
        }
        i += 1; j += 1;
    }
}

/// Computes the reduced row echelon form of the matrix in place.
/// Recall the properties of a matrix in reduced row echelon form.
/// - Matrix is in row echelon form.
/// - The first nonzero entry in each row is equal to 1.
/// - All entries above pivots are equal to zero.
pub fn reduced_row_echelon(matrix: &mut VecMatrix<Rational>) {
    row_echelon(matrix);

    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < matrix.rowc() && j < matrix.colc() {
        // Since the matrix is already in row echelon form, all
        // entries below this one are guaranteed to be zero.
        if matrix[i][j].numerator() == 0 {
            // Advance the cursor to the next column perhaps the pivot is there.
            j += 1;
            continue;
        }
        
        // Found the pivot! 
        
        // Zero out all cells above this one,
        for k in 0..i {
            if matrix[k][j].numerator() == 0 { continue; }
            let scalar = Rational::additive_inverse(&matrix[k][j]);
            crate::rowops::add(matrix, k, i, &scalar);
        }
        i += 1; j += 1;
    }   
}
