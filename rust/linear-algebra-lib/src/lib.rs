use std::ops::Add;

#[derive(Debug)]
pub struct Matrix<const N: usize> {
    pub cols: usize,
    pub rows: usize,
    pub data: [f64; N],
}

impl<const N: usize> Matrix<N> {
    pub fn new(cols: usize, rows: usize, data: [f64; N]) -> Matrix<N> {
        Matrix { cols, rows, data }
    }
    // this is 0-based indexing
    pub fn index(&self, i: usize, j: usize) -> &f64 {
        &self.data[i * self.rows + j]
    }
}

impl<const N: usize> Add<Matrix<N>> for Matrix<N> {
    type Output = Matrix<N>;
    fn add(self, rhs: Matrix<N>) -> Self::Output {
        let mut return_data: [f64; N] = [0.; N];
        for (i, item) in return_data.iter_mut().enumerate().take(self.data.len()) {
            *item = self.data[i] + rhs.data[i];
        }
        Matrix::new(self.cols, self.rows, return_data)
    }
}
