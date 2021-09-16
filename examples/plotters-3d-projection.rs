//! A 3d graphic where you can control the projection
//!
//! sources: <https://github.com/38/plotters/tree/master/examples/3d-plot2.rs>
//!
//! Run with:
//! ```sh
//! cargo re plotters-3d-projection --features="plotters"
//! ```

// FIX:
// - yaw & pitch, negative values and wrap
// - switch 3d graph with another key
// - change scale with another key?
// - solve the problem of input delay
//   (join multiple inputs until there are no more, and sum the results)

use notcurses::*;
use plotters::prelude::*;

const FPS: u32 = 20;
const MIN_MOVE: f64 = 0.1;

struct State {
    pitch: f64,
    yaw: f64,
    width: u32,
    height: u32,
}

fn main() -> NResult<()> {
    let mut nc = Notcurses::new()?;

    let geom = nc.geometry();

    let mut plane = Plane::build()
        .cols_rows(geom.cols, geom.rows)
        .new_pile(&mut nc)?;
    let mut buffer = vec![0; geom.bx as usize * geom.by as usize * 3];

    let mut state = State {
        pitch: 0.7,
        yaw: 0.7,
        width: geom.bx,
        height: geom.by,
    };

    let mut info_plane = Plane::build()
        .cols_rows(20, 3)
        .xy(3, 1)
        .into_pile(&mut plane)?;
    info_plane.set_base(" ", Style::BOLD, Channels::new(Rgb::BLACK, Rgb::YELLOW))?;
    info_plane.scrolling(true);

    let mut input = sys::NcInput::new_empty();

    loop {
        plot(&mut buffer, &mut state).expect("plotting error");

        let key = sys::notcurses_getc_nblock(nc.as_nc_mut(), &mut input);
        match key {
            sys::NCKEY_UP => {
                state.pitch -= MIN_MOVE;
            }
            sys::NCKEY_DOWN => {
                state.pitch += MIN_MOVE;
            }
            sys::NCKEY_LEFT => {
                state.yaw -= MIN_MOVE;
            }
            sys::NCKEY_RIGHT => {
                state.yaw += MIN_MOVE;
            }
            'q' => break Ok(()),
            _ => {}
        }

        let mut visual = Visual::build()
            .from_rgb(&buffer, geom.bx, geom.by, 255)?
            .blitter(Blitter::Pixel)
            .plane(&mut plane)
            .finish()?;

        info_plane.putstr_xy(
            0,
            0,
            &format![
                "↓↑ pitch: {:.2}\n←→ yaw: {:.2}\n'q' to quit",
                state.pitch, state.yaw
            ],
        )?;

        visual.render_plane(&mut nc)?;
        plane.display()?;
        sleep![0, 1000 / FPS as u64];
    }
}

fn plot(buffer: &mut Vec<u8>, state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::with_buffer(buffer, (state.width, state.height)).into_drawing_area();
    root.fill(&BLACK)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("2D Guassian PDF", ("sans-serif", 20))
        .build_cartesian_3d(-3.0..3.0, 0.0..6.0, -3.0..3.0)?;

    chart.with_projection(|mut p| {
        p.pitch = state.pitch as f64;
        p.yaw = state.yaw as f64;
        // p.pitch = 1.57 - (1.57 - state.pitch as f64 / 50.0).abs();
        p.scale = 0.7;
        p.into_matrix() // build the projection matrix
    });

    chart.configure_axes().draw()?;

    chart.draw_series(
        SurfaceSeries::xoz(
            (-15..=15).map(|x| x as f64 / 5.0),
            (-15..=15).map(|x| x as f64 / 5.0),
            pdf,
        )
        .style_func(&|&v| (&HSLColor(240.0 / 360.0 - 240.0 / 360.0 * v / 5.0, 1.0, 0.7)).into()),
    )?;
    root.present()?;
    Ok(())
}

fn plotb(buffer: &mut Vec<u8>, state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    let area = BitMapBackend::with_buffer(buffer, (state.width, state.height)).into_drawing_area();

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

fn pdf(x: f64, y: f64) -> f64 {
    const SDX: f64 = 0.1;
    const SDY: f64 = 0.1;
    const A: f64 = 5.0;
    let x = x as f64 / 10.0;
    let y = y as f64 / 10.0;
    A * (-x * x / 2.0 / SDX / SDX - y * y / 2.0 / SDY / SDY).exp()
}
