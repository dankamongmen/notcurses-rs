//! A slideshow of ten graphs examples from the `plotters` crate.
//!
//! sources: <https://github.com/38/plotters/tree/master/examples>
//!
//! Run with:
//! ```sh
//! cargo re plotters-slideshow --features="plotters"
//! ```

use notcurses::*;

fn main() -> NResult<()> {
    let mut nc = Notcurses::new()?;

    let (cols, rows) = nc.term_size();
    let geom = nc.term_pixelgeometry();

    let mut plot_plane = Plane::build().cols_rows(cols, rows).new_pile(&mut nc)?;
    let mut plot_buffer = vec![0; geom.max_bitmap_x as usize * geom.max_bitmap_y as usize * 3];

    // FIXME: BUG with wezterm, plane is black after updating the underlying visual
    let mut info_plane = Plane::build()
        .cols_rows(10, 1)
        .xy(2, 1)
        .into_pile(&mut plot_plane)?;
    info_plane.set_base(" ", Style::BOLD, Channels::new(Rgb::BLACK, Rgb::YELLOW))?;

    // the list of functions, shuffled
    let pfunctions = {
        use plotter_examples::*;
        use rand::{seq::SliceRandom, thread_rng};
        let mut rng = thread_rng();
        let mut flist = [
            plot_3d,
            plot_chart,
            plot_histogram,
            plot_mandelbrot,
            plot_normal_dist,
            plot_matshow,
            plot_two_scales,
            plot_sierpinski,
            plot_normal_dist2,
            errorbar,
        ];
        flist.shuffle(&mut rng);
        flist
    };
    let mut fcounter = 0;

    for f in pfunctions {
        f(&mut plot_buffer, geom.max_bitmap_x, geom.max_bitmap_y).expect("plotting failed");

        let mut plot_visual = Visual::build()
            .from_rgb(&plot_buffer, geom.max_bitmap_x, geom.max_bitmap_y, 255)?
            .blitter(Blitter::Pixel)
            .interpolate(false)
            .plane(&mut plot_plane)
            .finish()?;

        fcounter += 1;
        info_plane.putstr_xy(0, 0, &format!["plot {}/{}", fcounter, pfunctions.len()])?;

        plot_visual.render_plane(&mut nc)?;
        plot_plane.display()?;
        sleep![2];
    }
    Ok(())
}

mod plotter_examples {
    use itertools::Itertools;
    use num_traits::sign::Signed;
    use plotters::{coord::Shift, prelude::*};
    use rand::SeedableRng;
    use rand_distr::{Distribution, Normal};
    use rand_xorshift::XorShiftRng;
    use std::ops::Range;

    type PResult = Result<(), Box<dyn std::error::Error>>;

    /// <https://github.com/38/plotters/blob/master/examples/3d-plot.rs>.
    pub fn plot_3d(buffer: &mut Vec<u8>, max_x: u32, max_y: u32) -> PResult {
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
            .legend(|(x, y)| {
                Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled())
            });

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

    /// <https://github.com/38/plotters/blob/master/examples/chart.rs>
    pub fn plot_chart(buffer: &mut Vec<u8>, max_x: u32, max_y: u32) -> PResult {
        let root_area = BitMapBackend::with_buffer(buffer, (max_x, max_y)).into_drawing_area();

        root_area.fill(&WHITE)?;

        let root_area = root_area.titled("Image Title", ("sans-serif", 60))?;

        let (upper, lower) = root_area.split_vertically(512);

        let x_axis = (-3.4f32..3.4).step(0.1);

        let mut cc = ChartBuilder::on(&upper)
            .margin(5)
            .set_all_label_area_size(50)
            .caption("Sine and Cosine", ("sans-serif", 40))
            .build_cartesian_2d(-3.4f32..3.4, -1.2f32..1.2f32)?;

        cc.configure_mesh()
            .x_labels(20)
            .y_labels(10)
            .disable_mesh()
            .x_label_formatter(&|v| format!("{:.1}", v))
            .y_label_formatter(&|v| format!("{:.1}", v))
            .draw()?;

        cc.draw_series(LineSeries::new(x_axis.values().map(|x| (x, x.sin())), &RED))?
            .label("Sine")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        cc.draw_series(LineSeries::new(
            x_axis.values().map(|x| (x, x.cos())),
            &BLUE,
        ))?
        .label("Cosine")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        cc.configure_series_labels().border_style(&BLACK).draw()?;

        cc.draw_series(PointSeries::of_element(
            (-3.0f32..2.1f32).step(1.0).values().map(|x| (x, x.sin())),
            5,
            ShapeStyle::from(&RED).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord)
                    + Circle::new((0, 0), size, style)
                    + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
            },
        ))?;

        let drawing_areas = lower.split_evenly((1, 2));

        for (drawing_area, idx) in drawing_areas.iter().zip(1..) {
            let mut cc = ChartBuilder::on(&drawing_area)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .margin_right(20)
                .caption(format!("y = x^{}", 1 + 2 * idx), ("sans-serif", 40))
                .build_cartesian_2d(-1f32..1f32, -1f32..1f32)?;
            cc.configure_mesh().x_labels(5).y_labels(3).draw()?;

            cc.draw_series(LineSeries::new(
                (-1f32..1f32)
                    .step(0.01)
                    .values()
                    .map(|x| (x, x.powf(idx as f32 * 2.0 + 1.0))),
                &BLUE,
            ))?;
        }

        Ok(())
    }

    /// <https://github.com/38/plotters/blob/master/examples/histogram.rs>
    pub fn plot_histogram(buffer: &mut Vec<u8>, max_x: u32, max_y: u32) -> PResult {
        let root = BitMapBackend::with_buffer(buffer, (max_x, max_y)).into_drawing_area();

        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption("Histogram Test", ("sans-serif", 50.0))
            .build_cartesian_2d((0u32..10u32).into_segmented(), 0u32..10u32)?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(&WHITE.mix(0.3))
            .y_desc("Count")
            .x_desc("Bucket")
            .axis_desc_style(("sans-serif", 15))
            .draw()?;

        let data = [
            0u32, 1, 1, 1, 4, 2, 5, 7, 8, 6, 4, 2, 1, 8, 3, 3, 3, 4, 4, 3, 3, 3,
        ];

        chart.draw_series(
            Histogram::vertical(&chart)
                .style(RED.mix(0.5).filled())
                .data(data.iter().map(|x: &u32| (*x, 1))),
        )?;
        Ok(())
    }

    /// <https://github.com/38/plotters/blob/master/examples/mandelbrot.rs>
    pub fn plot_mandelbrot(buffer: &mut Vec<u8>, max_x: u32, max_y: u32) -> PResult {
        let root = BitMapBackend::with_buffer(buffer, (max_x, max_y)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .x_label_area_size(10)
            .y_label_area_size(10)
            .build_cartesian_2d(-2.1f64..0.6f64, -1.2f64..1.2f64)?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()?;

        let plotting_area = chart.plotting_area();

        let range = plotting_area.get_pixel_range();

        let (pw, ph) = (range.0.end - range.0.start, range.1.end - range.1.start);
        let (xr, yr) = (chart.x_range(), chart.y_range());

        for (x, y, c) in mandelbrot_set(xr, yr, (pw as usize, ph as usize), 100) {
            if c != 100 {
                plotting_area.draw_pixel((x, y), &HSLColor(c as f64 / 100.0, 1.0, 0.5))?;
            } else {
                plotting_area.draw_pixel((x, y), &BLACK)?;
            }
        }

        Ok(())
    }
    fn mandelbrot_set(
        real: Range<f64>,
        complex: Range<f64>,
        samples: (usize, usize),
        max_iter: usize,
    ) -> impl Iterator<Item = (f64, f64, usize)> {
        let step = (
            (real.end - real.start) / samples.0 as f64,
            (complex.end - complex.start) / samples.1 as f64,
        );
        return (0..(samples.0 * samples.1)).map(move |k| {
            let c = (
                real.start + step.0 * (k % samples.0) as f64,
                complex.start + step.1 * (k / samples.0) as f64,
            );
            let mut z = (0.0, 0.0);
            let mut cnt = 0;
            while cnt < max_iter && z.0 * z.0 + z.1 * z.1 <= 1e10 {
                z = (z.0 * z.0 - z.1 * z.1 + c.0, 2.0 * z.0 * z.1 + c.1);
                cnt += 1;
            }
            return (c.0, c.1, cnt);
        });
    }

    /// <https://github.com/38/plotters/blob/master/examples/normal-dist.rs>
    pub fn plot_normal_dist(buffer: &mut Vec<u8>, max_x: u32, max_y: u32) -> PResult {
        let root = BitMapBackend::with_buffer(buffer, (max_x, max_y)).into_drawing_area();
        root.fill(&WHITE)?;
        let sd = 0.13;
        let random_points: Vec<(f64, f64)> = {
            let norm_dist = Normal::new(0.5, sd).unwrap();
            let mut x_rand = XorShiftRng::from_seed(*b"MyFragileSeed123");
            let mut y_rand = XorShiftRng::from_seed(*b"MyFragileSeed321");
            let x_iter = norm_dist.sample_iter(&mut x_rand);
            let y_iter = norm_dist.sample_iter(&mut y_rand);
            x_iter.zip(y_iter).take(5000).collect()
        };

        let areas = root.split_by_breakpoints([944], [80]);

        let mut x_hist_ctx = ChartBuilder::on(&areas[0])
            .y_label_area_size(40)
            .build_cartesian_2d((0.0..1.0).step(0.01).use_round().into_segmented(), 0..250)?;
        let mut y_hist_ctx = ChartBuilder::on(&areas[3])
            .x_label_area_size(40)
            .build_cartesian_2d(0..250, (0.0..1.0).step(0.01).use_round())?;
        let mut scatter_ctx = ChartBuilder::on(&areas[2])
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;
        scatter_ctx
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()?;
        scatter_ctx.draw_series(
            random_points
                .iter()
                .map(|(x, y)| Circle::new((*x, *y), 2, GREEN.filled())),
        )?;
        let x_hist = Histogram::vertical(&x_hist_ctx)
            .style(GREEN.filled())
            .margin(0)
            .data(random_points.iter().map(|(x, _)| (*x, 1)));
        let y_hist = Histogram::horizontal(&y_hist_ctx)
            .style(GREEN.filled())
            .margin(0)
            .data(random_points.iter().map(|(_, y)| (*y, 1)));
        x_hist_ctx.draw_series(x_hist)?;
        y_hist_ctx.draw_series(y_hist)?;
        Ok(())
    }

    /// <https://github.com/38/plotters/blob/master/examples/matshow.rs>
    pub fn plot_matshow(buffer: &mut Vec<u8>, max_x: u32, max_y: u32) -> PResult {
        let root = BitMapBackend::with_buffer(buffer, (max_x, max_y)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("Matshow Example", ("sans-serif", 80))
            .margin(5)
            .top_x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0i32..15i32, 15i32..0i32)?;

        chart
            .configure_mesh()
            .x_labels(15)
            .y_labels(15)
            .x_label_offset(35)
            .y_label_offset(25)
            .disable_x_mesh()
            .disable_y_mesh()
            .label_style(("sans-serif", 20))
            .draw()?;

        let mut matrix = [[0; 15]; 15];

        for i in 0..15 {
            matrix[i][i] = i + 4;
        }

        chart.draw_series(
            matrix
                .iter()
                .zip(0..)
                .map(|(l, y)| l.iter().zip(0..).map(move |(v, x)| (x as i32, y as i32, v)))
                .flatten()
                .map(|(x, y, v)| {
                    Rectangle::new(
                        [(x, y), (x + 1, y + 1)],
                        HSLColor(
                            240.0 / 360.0 - 240.0 / 360.0 * (*v as f64 / 20.0),
                            0.7,
                            0.1 + 0.4 * *v as f64 / 20.0,
                        )
                        .filled(),
                    )
                }),
        )?;
        Ok(())
    }

    /// <https://github.com/38/plotters/blob/master/examples/two-scales.rs>
    pub fn plot_two_scales(buffer: &mut Vec<u8>, max_x: u32, max_y: u32) -> PResult {
        let root = BitMapBackend::with_buffer(buffer, (max_x, max_y)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .right_y_label_area_size(40)
            .margin(5)
            .caption("Dual Y-Axis Example", ("sans-serif", 50.0).into_font())
            .build_cartesian_2d(0f32..10f32, (0.1f32..1e10f32).log_scale())?
            .set_secondary_coord(0f32..10f32, -1.0f32..1.0f32);

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .y_desc("Log Scale")
            .y_label_formatter(&|x| format!("{:e}", x))
            .draw()?;

        chart
            .configure_secondary_axes()
            .y_desc("Linear Scale")
            .draw()?;

        chart
            .draw_series(LineSeries::new(
                (0..=100).map(|x| (x as f32 / 10.0, (1.02f32).powf(x as f32 * x as f32 / 10.0))),
                &BLUE,
            ))?
            .label("y = 1.02^x^2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        chart
            .draw_secondary_series(LineSeries::new(
                (0..=100).map(|x| (x as f32 / 10.0, (x as f32 / 5.0).sin())),
                &RED,
            ))?
            .label("y = sin(2x)")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&RGBColor(128, 128, 128))
            .draw()?;
        Ok(())
    }

    /// <https://github.com/38/plotters/blob/master/examples/sierpinski.rs>
    pub fn plot_sierpinski(buffer: &mut Vec<u8>, max_x: u32, max_y: u32) -> PResult {
        let root = BitMapBackend::with_buffer(buffer, (max_x, max_y)).into_drawing_area();
        root.fill(&WHITE)?;
        let root = root
            .titled("Sierpinski Carpet Demo", ("sans-serif", 60))?
            .shrink(((1024 - 700) / 2, 0), (700, 700));
        sierpinski_carpet(5, &root)?;
        Ok(())
    }
    fn sierpinski_carpet(depth: u32, drawing_area: &DrawingArea<BitMapBackend, Shift>) -> PResult {
        if depth > 0 {
            let sub_areas = drawing_area.split_evenly((3, 3));
            for (idx, sub_area) in (0..).zip(sub_areas.iter()) {
                if idx != 4 {
                    sub_area.fill(&BLUE)?;
                    sierpinski_carpet(depth - 1, sub_area)?;
                } else {
                    sub_area.fill(&WHITE)?;
                }
            }
        }
        Ok(())
    }

    /// <https://github.com/38/plotters/blob/master/examples/normal-dist2.rs>
    pub fn plot_normal_dist2(buffer: &mut Vec<u8>, max_x: u32, max_y: u32) -> PResult {
        let sd = 0.60;

        let random_points: Vec<f64> = {
            let norm_dist = Normal::new(0.0, sd).unwrap();
            let mut x_rand = XorShiftRng::from_seed(*b"MyFragileSeed123");
            let x_iter = norm_dist.sample_iter(&mut x_rand);
            x_iter.take(5000).filter(|x| x.abs() <= 4.0).collect()
        };

        let root = BitMapBackend::with_buffer(buffer, (max_x, max_y)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .margin(5)
            .caption("1D Gaussian Distribution Demo", ("sans-serif", 30))
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 60)
            .set_label_area_size(LabelAreaPosition::Right, 60)
            .build_cartesian_2d(-4f64..4f64, 0f64..0.1)?
            .set_secondary_coord(
                (-4f64..4f64).step(0.1).use_round().into_segmented(),
                0u32..500u32,
            );

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .y_label_formatter(&|y| format!("{:.0}%", *y * 100.0))
            .y_desc("Percentage")
            .draw()?;

        chart.configure_secondary_axes().y_desc("Count").draw()?;

        let actual = Histogram::vertical(chart.borrow_secondary())
            .style(GREEN.filled())
            .margin(3)
            .data(random_points.iter().map(|x| (*x, 1)));

        chart
            .draw_secondary_series(actual)?
            .label("Observed")
            .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], GREEN.filled()));

        let pdf = LineSeries::new(
            (-400..400).map(|x| x as f64 / 100.0).map(|x| {
                (
                    x,
                    (-x * x / 2.0 / sd / sd).exp() / (2.0 * std::f64::consts::PI * sd * sd).sqrt()
                        * 0.1,
                )
            }),
            &RED,
        );

        chart
            .draw_series(pdf)?
            .label("PDF")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.filled()));

        chart.configure_series_labels().draw()?;
        Ok(())
    }

    /// <https://github.com/38/plotters/blob/master/examples/errorbar.rs>
    pub fn errorbar(buffer: &mut Vec<u8>, max_x: u32, max_y: u32) -> PResult {
        let data = generate_random_data();
        let down_sampled = down_sample(&data[..]);

        let root = BitMapBackend::with_buffer(buffer, (max_x, max_y)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("Linear Function with Noise", ("sans-serif", 60))
            .margin(10)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(-10f64..10f64, -10f64..10f64)?;

        chart.configure_mesh().draw()?;

        chart
            .draw_series(LineSeries::new(data, &GREEN.mix(0.3)))?
            .label("Raw Data")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

        chart.draw_series(LineSeries::new(
            down_sampled.iter().map(|(x, _, y, _)| (*x, *y)),
            &BLUE,
        ))?;

        chart
            .draw_series(
                down_sampled.iter().map(|(x, yl, ym, yh)| {
                    ErrorBar::new_vertical(*x, *yl, *ym, *yh, BLUE.filled(), 20)
                }),
            )?
            .label("Down-sampled")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        chart
            .configure_series_labels()
            .background_style(WHITE.filled())
            .draw()?;

        Ok(())
    }
    fn generate_random_data() -> Vec<(f64, f64)> {
        let norm_dist = Normal::new(0.0, 1.0).unwrap();
        let mut x_rand = XorShiftRng::from_seed(*b"MyFragileSeed123");
        let x_iter = norm_dist.sample_iter(&mut x_rand);
        x_iter
            .take(20000)
            .filter(|x| x.abs() <= 4.0)
            .zip(-10000..10000)
            .map(|(yn, x)| {
                (
                    x as f64 / 1000.0,
                    x as f64 / 1000.0 + yn * x as f64 / 10000.0,
                )
            })
            .collect()
    }
    fn down_sample(data: &[(f64, f64)]) -> Vec<(f64, f64, f64, f64)> {
        let down_sampled: Vec<_> = data
            .iter()
            .group_by(|x| (x.0 * 1.0).round() / 1.0)
            .into_iter()
            .map(|(x, g)| {
                let mut g: Vec<_> = g.map(|(_, y)| *y).collect();
                g.sort_by(|a, b| a.partial_cmp(b).unwrap());
                (
                    x,
                    g[0],
                    g.iter().sum::<f64>() / g.len() as f64,
                    g[g.len() - 1],
                )
            })
            .collect();
        down_sampled
    }

}
