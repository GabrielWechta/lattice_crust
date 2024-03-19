pub mod lattice {
    use crate::matrix::matrix::Matrix;
    use crate::utils;
    use plotters::backend::BitMapBackend;
    use plotters::chart::{ChartBuilder, LabelAreaPosition};
    use plotters::drawing::IntoDrawingArea;
    use plotters::element::{Circle,};
    use plotters::prelude::{BLACK, Color, CYAN, WHITE};
    use plotters::prelude::*;

    pub struct Lattice {
        pub basis: Matrix<f64>,
        pub spanned_elements: Matrix<f64>,
    }

    impl Lattice {
        pub fn new(basis: Matrix<f64>) -> Lattice {
            let spanned_elements = span(&basis, (-10, 10));
            Lattice {
                basis,
                spanned_elements,
            }
        }

        pub fn plot_lattice(self) {
            let data = self.spanned_elements.elements;
            let root_drawing_area = BitMapBackend::new("plot.png", (600, 400)).into_drawing_area();
            let x_range = -10.0..10.0;
            let y_range = -10.0..10.0;

            root_drawing_area.fill(&WHITE).unwrap();

            let mut ctx = ChartBuilder::on(&root_drawing_area)
                .caption("Figure Sample", ("Arial", 30))
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .build_cartesian_2d(x_range.clone(), y_range.clone())
                .unwrap();

            // plot all spanned elements
            ctx.draw_series(
                data.iter()
                    .filter(|point| x_range.contains(&point[0]) && y_range.contains(&point[1]))
                    .map(|point| Circle::new((point[0], point[1]), 2, CYAN.filled())),
            );
            ctx.draw_series(
                self.basis.elements.iter()
                    .filter(|point| x_range.contains(&point[0]) && y_range.contains(&point[1]))
                    .map(|point| Circle::new((point[0], point[1]), 2, MAGENTA.filled())),
            );
            

            ctx.configure_mesh().draw().unwrap();
        }
    }

    pub fn span(basis: &Matrix<f64>, z_range: (i32, i32)) -> Matrix<f64> {
        let z_left = z_range.0;
        let z_right = z_range.1;
        let dimension = basis.rows_num;
        let mut span = vec![vec![0.0; dimension]];
        for base in &basis.elements {
            let mut add_span = vec![];
            for z in z_left..z_right {
                let add_vec = utils::scal_mul_vec(z, &base);
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
