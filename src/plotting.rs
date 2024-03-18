use plotters::prelude::*;

pub fn plot_lattice(data: Vec<Vec<f64>>) -> Result<(), Box<dyn std::error::Error>> {
    // Define your data
    // Create a new plot
    let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Define x and y ranges
    let x_min = -10.0;
    let x_max = 10.0;
    let y_min = -10.0;
    let y_max = 10.0;

    // Create a chart context
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(5)
        .caption("2D Axis with Points", ("Arial", 30))
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    // Draw points
    chart
        .draw_series(data.iter().map(|point| {
            Circle::new(
                (point[0], point[1]), // x, y coordinates
                5,                    // radius
                &RED.mix(0.5),        // color
            )
        }))?
        .label("Points")
        .legend(|(x, y)| Circle::new((x, y), 5, RED.mix(0.5)));

    // Display legend
    chart.configure_series_labels().draw()?;
    Ok(())
}
