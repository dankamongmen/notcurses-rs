//! displays how you can manipulate [`Visual`]s.

use notcurses::*;
use rand::{distributions::Uniform, Rng};

const H: u32 = 32;
const W: u32 = 32;

fn main() -> NResult<()> {
    let mut nc = Notcurses::new()?;

    println!("terminal detected: {}", nc.term_name());
    sleep![1];

    let mut buffer = Vec::<u8>::with_capacity((W * H * 4) as usize);
    fill_buffer(&mut buffer, true);

    let mut plane = Plane::build().cols_rows(W * 2, H * 2).new_pile(&mut nc)?;
    let mut visual = Visual::build()
        .from_rgba(&buffer, W, H)?
        .blitter(Blitter::Pixel)
        .interpolate(false)
        .scale(Scale::Scale)
        .plane(&mut plane)
        .finish()?;
    visual.render_plane(&mut nc)?;
    plane.display()?;
    sleep![0, 500];

    let mut rng_house = rand::thread_rng();
    for n in 0..50 {
        // display random house every 10 frames
        if n % 10 == 9 {
            let random_house = rng_house.gen_range(0..=2);
            match random_house {
                0 => visual.set_from_file(&path("examples/img/house0.png"))?,
                1 => visual.set_from_file(&path("examples/img/house1.png"))?,
                2 | _ => visual.set_from_file(&path("examples/img/house2.png"))?,
            }

            visual.render_plane(&mut nc)?;
            plane.display()?;
            sleep![0, 200];
        } else {
            fill_buffer(&mut buffer, false);
            visual.set_from_rgba(&buffer, W, H)?;
            visual.render_plane(&mut nc)?;
            plane.display()?;
            sleep![0, 25];
        }
    }
    sleep![1];
    Ok(())
}

/// Fills the buffer with random colors.
fn fill_buffer(buffer: &mut Vec<u8>, empty: bool) {
    let mut rng = rand::thread_rng();
    let range = Uniform::from(50..=180);
    if empty {
        for _byte in 0..=(W * H) {
            buffer.push(rng.sample(&range));
            buffer.push(rng.sample(&range));
            buffer.push(rng.sample(&range));
            buffer.push(255);
        }
    } else {
        for chunk in buffer.chunks_mut(4) {
            chunk[0] = rng.sample(&range);
            chunk[1] = rng.sample(&range);
            chunk[2] = rng.sample(&range);
            chunk[3] = 255;
        }
    }
}

fn path(relative: &str) -> String {
    let mut path = project_root::get_project_root().unwrap();
    path.push(relative);
    path.to_str().unwrap().to_owned()
}
