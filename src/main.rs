use crate::lattice::lattice::Lattice;
use crate::matrix::matrix::Matrix;
use crate::utils::dot_prod_vecs;

mod lattice;
mod matrix;
mod utils;

fn main() {
    let mut b = Matrix::new(vec![vec![6.0, 0.6], vec![2.5, 2.5]]);
    // let mut c = Matrix::new(vec![vec![1.0, 0.6], vec![2.5, 2.5]]);
    b.gram_schmidt_orthogonalization(true);
    println!("{:?}", dot_prod_vecs(&b.elements[0], &b.elements[1]));
    let l = Lattice::new(b);
    // println!("{:?}", l.get_volume());
    l.plot_lattice(20);
}
