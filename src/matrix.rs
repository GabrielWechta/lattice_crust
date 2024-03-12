pub mod matrix {
    use std::fmt::Debug;
    use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};
    use std::vec;

    pub struct Matrix<T> {
        pub elements: Vec<Vec<T>>,
        pub rows_num: usize,
        pub cols_num: usize,
    }

    impl<T: Debug> Matrix<T> {
        pub fn print(&self) {
            for i in 0..self.rows_num {
                println!("{:?}", self.elements[i]);
            }
        }
    }

    impl<T: Copy + PartialEq> Matrix<T> {
        pub fn new(elements: Vec<Vec<T>>) -> Matrix<T> {
            let rows_num = elements.len();
            let cols_num = elements[0].len();
            for i in 0..rows_num {
                assert_eq!(
                    cols_num,
                    elements[i].len(),
                    "{}-th row does not have as many elements as the first row.",
                    i
                )
            }
            Matrix {
                elements,
                rows_num,
                cols_num,
            }
        }
    }
    impl<
            T: Copy
                + PartialEq
                + Default
                + Into<f64>
                + Add<Output = T>
                + Sub<Output = T>
                + Mul<Output = T>
                + Div<Output = T>
                + Neg<Output = T>,
        > Matrix<T>
    {
        pub fn identity(size: usize) -> Matrix<f64> {
            let mut elements = vec![vec![0.0; size]; size];
            for i in 0..size {
                elements[i][i] = 1.0
            }
            Matrix::new(elements)
        }
        pub fn transpose(&self) -> Matrix<T> {
            let mut result_elements = vec![vec![T::default(); self.rows_num]; self.cols_num];
            for i in 0..self.cols_num {
                for j in 0..self.rows_num {
                    result_elements[i][j] = self.elements[j][i]
                }
            }
            Matrix::new(result_elements)
        }
        pub fn determinant(&self) -> T {
            assert_eq!(self.rows_num, self.cols_num, "Matrix must be square");

            match self.rows_num {
                1 => self.elements[0][0],
                2 => {
                    self.elements[0][0] * self.elements[1][1]
                        - self.elements[0][1] * self.elements[1][0]
                }
                _ => {
                    let mut det = T::default();
                    for j in 0..self.cols_num {
                        let minor = self.minor(0, j);
                        let cofactor = if j % 2 == 0 {
                            minor.determinant()
                        } else {
                            -minor.determinant()
                        };
                        det = det + self.elements[0][j] * cofactor;
                    }
                    det
                }
            }
        }
        pub fn minor(&self, row: usize, col: usize) -> Matrix<T> {
            let mut minor = Matrix::new(vec![
                vec![T::default(); self.rows_num - 1];
                self.cols_num - 1
            ]);
            let mut minor_row = 0;
            let mut minor_col = 0;

            for i in 0..self.rows_num {
                if i != row {
                    minor_col = 0;
                    for j in 0..self.cols_num {
                        if j != col {
                            minor.elements[minor_row][minor_col] = self.elements[i][j];
                            minor_col += 1;
                        }
                    }
                    minor_row += 1;
                }
            }
            minor
        }
        pub fn inverse(&self) -> Option<Matrix<f64>> {
            let det = self.determinant().into();
            if det == 0.0 {
                return None;
            }

            let mut inverse = Matrix::new(vec![vec![0.0; self.rows_num]; self.cols_num]);

            for i in 0..self.rows_num {
                for j in 0..self.cols_num {
                    let minor = self.minor(i, j);
                    let cofactor = if (i + j) % 2 == 0 {
                        minor.determinant().into()
                    } else {
                        -minor.determinant().into()
                    };
                    inverse.elements[j][i] = cofactor / det;
                }
            }
            Some(inverse)
        }
    }

    impl<T: Copy + PartialEq + Add<Output = T> + Default> Add for Matrix<T> {
        type Output = Matrix<T>;
        fn add(self, rhs: Matrix<T>) -> Matrix<T> {
            assert_eq!(self.rows_num, rhs.rows_num);
            assert_eq!(self.cols_num, rhs.cols_num);
            let mut result_elements = vec![vec![T::default(); self.rows_num]; self.cols_num];
            for i in 0..self.rows_num {
                for j in 0..self.cols_num {
                    result_elements[i][j] = self.elements[i][j] + rhs.elements[i][j];
                }
            }
            Matrix::new(result_elements)
        }
    }

    impl<T: Copy + PartialEq + Mul<Output = T> + Default + AddAssign> Mul for Matrix<T> {
        type Output = Matrix<T>;
        fn mul(self, rhs: Matrix<T>) -> Matrix<T> {
            assert_eq!(self.cols_num, rhs.rows_num,);

            let mut result_elements = vec![vec![T::default(); rhs.cols_num]; self.rows_num];

            for i in 0..self.rows_num {
                for j in 0..rhs.cols_num {
                    for k in 0..self.cols_num {
                        result_elements[i][j] += self.elements[i][k] * rhs.elements[k][j];
                    }
                }
            }

            Matrix::new(result_elements)
        }
    }
}