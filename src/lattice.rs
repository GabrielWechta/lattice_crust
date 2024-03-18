pub mod lattice {
    use crate::matrix::matrix::Matrix;
    use crate::utils;

    pub struct Lattice {
        pub basis: Matrix<f64>,
    }

    impl Lattice {
        pub fn new(basis: Matrix<f64>) -> Lattice {
            Lattice { basis }
        }
        pub fn span(self, z_range: Vec<i64>) -> Matrix<f64> {
            let dimension = self.basis.rows_num;
            let mut span = vec![vec![0.0; dimension]];
            for base in &self.basis.elements {
                let mut add_span = vec![];
                for z in &z_range {
                    let add_vec = utils::scal_mul_vec(*z, &base);
                    for s in &span {
                        let v = utils::add_vecs(s, &add_vec);
                        if !add_span.contains(&v) {
                            add_span.push(v);
                        }
                    }
                }
                span.extend(add_span);
            }
            Matrix::new(span)
        }
    }
}
