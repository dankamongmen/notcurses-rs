//! A 3d graph.
//!
//! based on: <https://github.com/38/plotters/blob/master/examples/3d-plot.rs>
//!
//! Run with:
//! ```sh
//! cargo re plotters-3d --features="plotters"
//! ```

use notcurses::*;
use plotters::prelude::*;

fn main() -> NResult<()> {
    let mut nc = Notcurses::new()?;

    let (cos, rows) = nc.term_size();
    let geom = nc.term_pixelgeometry();

    let mut buffer = vec![0; geom.max_bitmap_x as usize * geom.max_bitmap_y as usize * 3];
    plot(&mut buffer, geom.max_bitmap_x, geom.max_bitmap_y).expect("plotting failed");

    let mut plane = Plane::build().cols_rows(cols, rows).new_pile(&mut nc)?;
    let mut visual = Visual::build()
        .from_rgb(&buffer, geom.max_bitmap_x, geom.max_bitmap_y, 255)?
        .blitter(Blitter::Pixel)
        .interpolate(false)
        .plane(&mut plane)
        .finish()?;

    visual.render_plane(&mut nc)?;
    plane.display()?;
    sleep![5];
    Ok(())
}

fn plot(buffer: &mut Vec<u8>, max_x: u32, max_y: u32) -> Result<(), Box<dyn std::error::Error>> {
    let area = BitMapBackend::with_buffer(buffer, (max_x, max_y)).into_drawing_area();

    area.fill(&WHITE)?;

    let x_axis = (-3.0..3.0).step(0.1);
    let z_axis = (-3.0..3.0).step(0.1);

    let mut chart = ChartBuilder::on(&area)
        .caption(format!("3D Plot Test"), ("sans", 20))
        .build_cartesian_3d(x_axis.clone(), -3.0..3.0, z_axis.clone())?;

    chart.with_projection(|mut pb| {
        pb.yaw = 0.5;
        pb.scale = 0.9;
        pb.into_matrix()
    });

    chart.configure_axes().draw()?;

    chart
        .draw_series(
            SurfaceSeries::xoz(
                (-30..30).map(|f| f as f64 / 10.0),
                (-30..30).map(|f| f as f64 / 10.0),
                |x, z| (x * x + z * z).cos(),
            )
            .style(BLUE.mix(0.2).filled()),
        )?
        .label("Surface")
        .legend(|(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled()));

    chart
        .draw_series(LineSeries::new(
            (-100..100)
                .map(|y| y as f64 / 40.0)
                .map(|y| ((y * 10.0).sin(), y, (y * 10.0).cos())),
            &BLACK,
        ))?
        .label("Line")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
