//! A CPU stats monitor.
//!
//! based on: <https://github.com/plotters-rs/plotters-piston/>
//!
//! Run with:
//! ```sh
//! cargo re plotters-cpustat --features="plotters, systemstat"
//! ```

// TODO: handle resize

use notcurses::*;
use plotters::prelude::*;
use std::collections::vec_deque::VecDeque;
use systemstat::{CPULoad, DelayedMeasurement, Platform, System};

const FPS: u32 = 10;
const LENGTH: u32 = 20;
const N_DATA_POINTS: usize = (FPS * LENGTH) as usize;

struct State {
    load_measurement: Vec<DelayedMeasurement<Vec<CPULoad>>>,
    epoch: usize,
    data: Vec<VecDeque<f32>>,
    system: System,
    width: u32,
    height: u32,
}

fn main() -> NResult<()> {
    let mut nc = Notcurses::new()?;

    let (cols, rows) = nc.term_size();
    let geom = nc.term_pixelgeometry();

    let mut plane = Plane::build().cols_rows(cols, rows).new_pile(&mut nc)?;
    let mut buffer = vec![0; geom.max_bitmap_x as usize * geom.max_bitmap_y as usize * 3];

    let system = System::new();
    let mut state = State {
        load_measurement: (0..FPS).map(|_| system.cpu_load().unwrap()).collect(),
        epoch: 0,
        data: vec![],
        system: system,
        width: geom.max_bitmap_x,
        height: geom.max_bitmap_y,
    };

    let mut input = sys::NcInput::new_empty();

    loop {
        plot(&mut buffer, &mut state).expect("plotting error");

        let key = sys::notcurses_getc_nblock(nc.as_nc_mut(), &mut input);
        match key {
            // WIP
            // sys::NCKEY_RESIZE => break Err(NotcursesError::ExitMessage("resize".into())),
            'q' => break Ok(()),
            _ => {}
        }

        let mut visual = Visual::build()
            .from_rgb(&buffer, geom.max_bitmap_x, geom.max_bitmap_y, 255)?
            .blitter(Blitter::Pixel)
            .plane(&mut plane)
            .finish()?;

        visual.render_plane(&mut nc)?;
        plane.show()?;
        sleep![0, 1000 / FPS as u64];
    }
}

fn plot(buffer: &mut Vec<u8>, state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::with_buffer(buffer, (state.width, state.height)).into_drawing_area();
    root.fill(&WHITE)?;

    let cpu_loads = state.load_measurement[state.epoch % FPS as usize].done()?;

    if state.data.len() < cpu_loads.len() {
        for _ in state.data.len()..cpu_loads.len() {
            state
                .data
                .push(VecDeque::from(vec![0f32; N_DATA_POINTS + 1]));
        }
    }

    for (core_load, target) in cpu_loads.into_iter().zip(state.data.iter_mut()) {
        if target.len() == N_DATA_POINTS + 1 {
            target.pop_front();
        }
        target.push_back(1.0 - core_load.idle);
    }

    let mut cc = ChartBuilder::on(&root)
        .margin(10)
        .caption("Real Time CPU Usage", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0..N_DATA_POINTS as u32, 0f32..1f32)?;

    cc.configure_mesh()
        .x_label_formatter(&|x| format!("{}", -(LENGTH as f32) + (*x as f32 / FPS as f32)))
        .y_label_formatter(&|y| format!("{}%", (*y * 100.0) as u32))
        .x_labels(15)
        .y_labels(5)
        .x_desc("Seconds")
        .y_desc("% Busy")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    for (idx, data) in (0..).zip(state.data.iter()) {
        cc.draw_series(LineSeries::new(
            (0..).zip(data.iter()).map(|(a, b)| (a, *b)),
            &Palette99::pick(idx),
        ))?
        .label(format!("CPU {}", idx))
        .legend(move |(x, y)| {
            Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(idx))
        });
    }

    cc.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    state.load_measurement[state.epoch % FPS as usize] = state.system.cpu_load()?;
    state.epoch += 1;
    Ok(())
}
