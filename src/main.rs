use crate::lattice::lattice::Lattice;
use crate::matrix::matrix::Matrix;
use crate::plotting::plot_lattice;

mod lattice;
mod matrix;
mod utils;
mod plotting;

fn main() {
    let a = Matrix::new(vec![vec![0.0, 1.0], vec![1.0, 0.0]]);
    let l = Lattice::new(a);
    let s = l.span(vec![-2,-1,0,1,2]);
    plot_lattice(s.elements);
}
