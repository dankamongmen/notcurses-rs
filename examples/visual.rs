//! Shows how you can manipulate [`Visual`]s.
//!
//! WIP

use notcurses::*;
use rand::{distributions::Uniform, rngs::ThreadRng, Rng};

const H: Dimension = 32;
const W: Dimension = 32;

fn main() -> Result<()> {
    let mut rng = rand::thread_rng();

    let mut nc = Nc::new()?;

    println!("terminal detected: {}", nc.term_name());
    s![0, 500];

    // fill the buffer with random color pixels
    let mut buffer = Vec::<u8>::with_capacity((W * H * 4) as usize);
    fill_buffer_rand(&mut buffer, &mut rng);

    // TEST1: WORKS with the standard plane
    // let v1 = sys::NcVisual::from_rgba(buffer.as_slice(), H, W * 4, W)?;
    // let v1o = sys::NcVisualOptions::without_plane(1, 2, 0, 0, H, W, sys::NCBLIT_PIXEL, 0, 0);
    // v1.render(&mut nc.raw, &v2o)?;
    // nc.raw.render()?;

    // TEST2: WORKS in another pile
    // let mut p2 = sys::NcPlane::new(&mut nc.raw, 0, 0, 32, 32)?;
    // let v2 = sys::NcVisual::from_rgba(buffer.as_slice(), H, W * 4, W)?;
    // let vo = sys::NcVisualOptions::with_plane(&mut p2, sys::NCSCALE_NONE, 0, 0, 0, 0, 0, 0, sys::NCBLIT_PIXEL, 0, 0);
    // v2.render(nc.raw, &vo)?;
    // p2.render()?;
    // p2.rasterize()?;

    // TEST3: WORKS. same as TEST2 but using `Plane`
    // let mut p3 = Plane::build().cols_rows(W, H).new_pile(&mut nc)?;
    // let v3 = sys::NcVisual::from_rgba(buffer.as_slice(), H, W * 4, W)?;
    // let vo = sys::NcVisualOptions::with_plane(p3.raw, sys::NCSCALE_NONE, 0, 0, 0, 0, 0, 0, sys::NCBLIT_PIXEL, 0, 0);
    // v3.render(nc.raw, &vo)?;
    // p3.render_raster()?;

    // TEST4: WORKS. same as TEST3 but using `Visual`.
    // let mut p4 = Plane::build().cols_rows(W, H).new_pile(&mut nc)?;
    // let mut v4 = Visual::from_rgba(&buffer, W, H)?;
    // let vo = VisualOptions::with_plane(p4.raw, Scale::NONE, Blitter::PIXEL);
    // v4.render(&mut nc, vo)?;
    // p4.render_raster()?;

    // TEST5: WORKS. same as TEST4 but without accesing the `raw` field in VisualOptions
    // let mut p5 = Plane::build().cols_rows(W, H).new_pile(&mut nc)?;
    // let mut v5 = Visual::from_rgba(&buffer, W, H)?;
    // let vo = VisualOptions::with_plane(&mut p5, Scale::NONE, Blitter::PIXEL);
    // v5.render(&mut nc, vo)?;
    // p5.render_raster()?;

    // FINAL_TEST: WORKS
    let mut root_plane = Plane::build().cols_rows(W * 2, H * 2).new_pile(&mut nc)?;
    let mut visual = Visual::build()
        .from_rgba(&buffer, W, H)?
        .blitter(Blitter::PIXEL)
        .interpolate(false)
        .into_plane(&mut root_plane, Scale::SCALE)?;
    visual.render(&mut nc)?;
    root_plane.render_raster()?;
    s![1];

    for _ in 0..50 {
        refill_buffer_rand(&mut buffer, &mut rng);
        visual.set_from_rgba(&buffer, W, H)?;
        visual.render(&mut nc)?;
        root_plane.render_raster()?;
        s![0, 25];
    }

    Ok(())
}

/// Fills the buffer with random colors.
fn fill_buffer_rand(buffer: &mut Vec<u8>, rng: &mut ThreadRng) {
    let range = Uniform::from(50..=180);
    for _byte in 0..=(W * H) {
        buffer.push(rng.sample(&range));
        buffer.push(rng.sample(&range));
        buffer.push(rng.sample(&range));
        buffer.push(255);
    }
}

/// Refills the buffer with random colors.
fn refill_buffer_rand(buffer: &mut Vec<u8>, rng: &mut ThreadRng) {
    let range = Uniform::from(50..=180);
    for chunk in buffer.chunks_mut(4) {
        chunk[0] = rng.sample(&range);
        chunk[1] = rng.sample(&range);
        chunk[2] = rng.sample(&range);
        chunk[3] = 255;
    }

    // MAYBE bench the speed difference
    // for (n, _) in (0..buffer.len()).enumerate() {
    //     if n % 4 == 3 {
    //         buffer[n] = 255;
    //     } else {
    //         buffer[n] = rng.sample(&range);
    //     }
    // }
}

/// Refills the buffer white
fn refill_buffer_white(buffer: &mut Vec<u8>) {
    for (n, _) in (0..buffer.len()).enumerate() {
        buffer[n] = 195;
    }
}
