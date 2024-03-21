use crate::lattice::lattice::Lattice;
use crate::matrix::matrix::Matrix;

mod lattice;
mod matrix;
mod utils;

fn main() {
    let mut b = Matrix::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
    // b.gram_schmidt_orthonormalization();
    // println!("{:?}", b.elements);
    let l = Lattice::new(b);
    println!("{:?}", l.get_volume());
    l.plot_lattice(20);
}
