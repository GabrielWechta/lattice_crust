pub mod lattice {
    use crate::matrix::matrix::Matrix;
    use crate::utils;
    use plotters::backend::BitMapBackend;
    use plotters::chart::{ChartBuilder, LabelAreaPosition};
    use plotters::drawing::IntoDrawingArea;
    use plotters::element::Circle;
    use plotters::prelude::*;
    use plotters::prelude::{Color, CYAN, WHITE};

    pub struct Lattice {
        pub dimension: usize,
        pub basis: Matrix<f64>,
        pub spanned_elements: Matrix<f64>,
        pub minkowski_first_theorem_distance: f64,
    }

    impl Lattice {
        pub fn new(basis: Matrix<f64>) -> Lattice {
            let spanned_elements = span(&basis, (-10, 10));
            if basis.check_independence() == false {
                panic!("Basis is not independent");
            }
            let minkowski_first_theorem_distance = self.get_minkowski_first_theorem_distance();
            println!("Minkowski first theorem distance: {}", radius);
            Lattice {
                dimension: basis.rows_num,
                basis: basis,
                spanned_elements: spanned_elements,
            }
        }

        pub fn plot_lattice(&self) {
            let data = self.spanned_elements.elements.clone();
            let radius = 30.0 *

            let root_drawing_area =
                BitMapBackend::new("lattice_plot.png", (600, 600)).into_drawing_area();
            let x_range = -10.0..10.0;
            let y_range = -10.0..10.0;

            root_drawing_area.fill(&WHITE).unwrap();
            let mut ctx = ChartBuilder::on(&root_drawing_area)
                .caption(
                    format!("Lattice plot (B={:?})", self.basis.elements),
                    ("Arial", 10),
                )
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .build_cartesian_2d(x_range.clone(), y_range.clone())
                .unwrap();

            // Draw a circle centered at (0.0, 0.0) with the given radius, radius is scaled by the dimension
            ctx.draw_series(std::iter::once(Circle::new(
                (0.0, 0.0),
                radius,
                Into::<RGBColor>::into(YELLOW).filled(),
            )));

            // plot all spanned elements
            ctx.draw_series(
                data.iter()
                    .filter(|point| x_range.contains(&point[0]) && y_range.contains(&point[1]))
                    .map(|point| Circle::new((point[0], point[1]), 2, CYAN.filled())),
            );
            ctx.draw_series(
                self.basis
                    .elements
                    .iter()
                    .filter(|point| x_range.contains(&point[0]) && y_range.contains(&point[1]))
                    .map(|point| Circle::new((point[0], point[1]), 2, MAGENTA.filled())),
            );


            ctx.configure_mesh().draw().unwrap();
        }
        pub fn get_volume(&self) -> f64 {
            self.basis.determinant().abs()
        }
        pub fn get_minkowski_first_theorem_distance(&self) -> f64 {
            let dim_f = self.dimension as f64;
            dim_f.sqrt() * self.get_volume().powf(1.0 / dim_f)
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
