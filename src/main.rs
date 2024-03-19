use crate::lattice::lattice::Lattice;
use crate::matrix::matrix::Matrix;

mod lattice;
mod matrix;
mod utils;

fn main() {
    let b = Matrix::new(vec![vec![7.1, 0.6], vec![-6.1, 0.2]]);
    let l = Lattice::new(b);
    l.plot_lattice();
}
