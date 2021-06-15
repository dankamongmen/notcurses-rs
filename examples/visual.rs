//! Shows how you can manipulate [`Visual`]s.

use notcurses::*;
use rand::{distributions::Uniform, Rng};

const H: Dimension = 32;
const W: Dimension = 32;

fn main() -> Result<()> {

    let mut nc = Nc::new()?;

    println!("terminal detected: {}", nc.term_name());
    sleep![0, 500];

    let mut buffer = Vec::<u8>::with_capacity((W * H * 4) as usize);
    fill_buffer_rand(&mut buffer);

    let mut root_plane = Plane::build().cols_rows(W * 2, H * 2).new_pile(&mut nc)?;
    let mut visual = Visual::build()
        .from_rgba(&buffer, W, H)?
        .blitter(Blitter::PIXEL)
        .interpolate(false)
        .into_plane(&mut root_plane, Scale::SCALE)?;
    visual.render(&mut nc)?;
    root_plane.render_raster()?;
    sleep![1];

    for _ in 0..50 {
        refill_buffer_rand(&mut buffer);
        visual.set_from_rgba(&buffer, W, H)?;
        visual.render(&mut nc)?;
        root_plane.render_raster()?;
        sleep![0, 25];
    }
    Ok(())
}

/// Fills the buffer with random colors.
fn fill_buffer_rand(buffer: &mut Vec<u8>) {
    let mut rng = rand::thread_rng();
    let range = Uniform::from(50..=180);
    for _byte in 0..=(W * H) {
        buffer.push(rng.sample(&range));
        buffer.push(rng.sample(&range));
        buffer.push(rng.sample(&range));
        buffer.push(255);
    }
}

/// Refills the buffer with random colors.
fn refill_buffer_rand(buffer: &mut Vec<u8>) {
    let mut rng = rand::thread_rng();
    let range = Uniform::from(50..=180);
    for chunk in buffer.chunks_mut(4) {
        chunk[0] = rng.sample(&range);
        chunk[1] = rng.sample(&range);
        chunk[2] = rng.sample(&range);
        chunk[3] = 255;
    }
}
