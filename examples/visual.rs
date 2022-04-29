// notcurses::examples::visual
//
//!
//

#![allow(unused_mut, unused_variables)]

use notcurses::*;
use rand::{distributions::Uniform, Rng};

const H: u32 = 20;
const W: u32 = 10;
const NUMPIX: usize = (H * W) as usize;

fn main() -> Result<()> {
    let mut nc = Notcurses::new_cli_silent()?;

    // Create a byte buffer with random rgba pixels:
    let mut rng = rand::thread_rng();
    let range = Uniform::from(50..=200);
    let mut rgba_buf = Vec::<u8>::with_capacity(NUMPIX * 4);
    for _ in 0..=NUMPIX {
        rgba_buf.push(rng.sample(&range));
        rgba_buf.push(rng.sample(&range));
        rgba_buf.push(rng.sample(&range));
        rgba_buf.push(255);
    }

    // Create a visual from the rgba buffer:
    let mut visual = Visual::from_rgba(rgba_buf.as_slice(), (W, H))?;

    // Blit the visual to a new plane:
    let mut new_plane = visual.blit(&mut nc)?;
    new_plane.render()?;
    sleep![1];

    // Blit the visual to a pre-existing plane:
    let mut existing_plane = Plane::builder().position((0, 25)).build(&mut nc)?;
    visual.blit_plane(&mut nc, &mut existing_plane)?;
    existing_plane.render()?;
    sleep![1];

    // Blit the visual into a new child plane:
    let mut parent_plane = Plane::builder().position((10, 50)).build(&mut nc)?;
    let mut child = visual.blit_child(&mut nc, &mut parent_plane)?;
    parent_plane.render()?;
    // child.render()?;
    sleep![1];

    Ok(())
}
