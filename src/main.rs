use crate::matrix::matrix::Matrix;

mod matrix;

fn main() {
    let a = Matrix::<f64>::identity(1000);
    a.print()
}
