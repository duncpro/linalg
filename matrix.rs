/// VecMatrix is a row-major heap-allocated matrix.
pub struct VecMatrix<T> { 
    values: Vec<T>, 
    rowc: usize,
    colc: usize, 
}

impl<T> VecMatrix<T> {
    pub fn new(rowc: usize, colc: usize, fill: T) -> Self
    where T: Clone 
    {
        let mut len = rowc * colc;
        let mut values = Vec::with_capacity(len);   
        for _ in 0..len { values.push(fill.clone()); }
        VecMatrix { values, rowc, colc }
    }

    pub fn rowc(&self) -> usize { self.rowc }
    pub fn colc(&self) -> usize { self.colc }
}

impl<T> std::ops::Index<usize> for VecMatrix<T> {
    type Output = [T];

    fn index<'a>(&'a self, row: usize) -> &'a Self::Output {
        let begin = row * self.colc;
        let end = begin + self.colc;
        &self.values[begin..end]
    }
}

impl<T> std::ops::IndexMut<usize> for VecMatrix<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let begin = row * self.colc;
        let end = begin + self.colc;
        &mut self.values[begin..end]
    }
}


pub fn literal<const R: usize, const C: usize, T>(arr: &[[impl Into<T> + Clone; C]; R]) -> VecMatrix<T>
where T: Default + Clone
{
    let mut matrix = VecMatrix::new(R, C, T::default());
    for i in 0..R {
        for j in 0..C {
            matrix[i][j] = arr[i][j].clone().into();
        }
    }
    return matrix;
}

impl<T> std::fmt::Display for VecMatrix<T> 
where T: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut strings: Vec<String> = Vec::with_capacity(self.values.len());
        for value in &self.values { strings.push(value.to_string()); }
        if let Some(maxlen) = strings.iter().map(|s| s.len()).max() {
            for s in &mut strings {
                let pad = maxlen - s.len();
                for _ in 0..pad { s.push(' '); }
            }
        }

        for i in 0..self.rowc {
            for j in 0..self.colc {
                write!(f, "{}", strings[(i * self.colc) + j])?;    
                write!(f, "  ")?;            
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
