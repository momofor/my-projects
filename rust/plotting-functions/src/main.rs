use plotters::{coord::types::RangedCoordf32, prelude::*};

const PLOT_WIDTH: u32 = 1920;
const PLOT_HEIGHT: u32 = 1080;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dot_and_label()
}

fn dot_and_label() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("noiceu.png", (PLOT_WIDTH, PLOT_HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .x_label_area_size(20)
        .y_label_area_size(20)
        .build_cartesian_2d(0f32..1f32, 0f32..1f32)?;
    chart.configure_mesh().draw()?;

    let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf32, RangedCoordf32>::new(
        0f32..1f32,
        0f32..1f32,
        (0..PLOT_WIDTH as i32, 0..PLOT_HEIGHT as i32),
    ));

    let dot_and_label = |x: f32, y: f32| {
        return EmptyElement::at((x, y))
            + Circle::new((0, 0), 3, ShapeStyle::from(&RED).filled())
            + Text::new(
                format!("({:.2},{:.2})", x, y),
                (10, 0),
                ("sans-serif", 18.0).into_font().color(&GREEN),
            );
    };
    root.draw(&dot_and_label(0.5, 0.6))?;
    root.draw(&dot_and_label(0.70, 0.66))?;
    root.draw(&dot_and_label(0.43, 0.27))?;
    root.draw(&dot_and_label(0.914, 0.074))?;
    root.present()?;

    Ok(())
}

fn _simple_plot() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("noice.png", (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &GREEN,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 30, y)], &BLACK));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}
