// notcurses::examples::plane
//
//!
//

#![allow(unused_variables, unused_mut)]

use notcurses::*;

fn main() -> Result<()> {
    let mut nc = Notcurses::new_cli_silent()?;

    // # constructors

    // let's create a root plane at (1, 1), and a child at (2, 2):
    let mut p1 = Plane::new_at(&mut nc, (1, 1))?;
    let p2 = p1.new_child_at((2, 2))?;

    // check their position relative to their parent:
    assert_eq![p1.position(), Position(1, 1)];
    assert_eq![p2.position(), Position(2, 2)];

    // check their position relative to the root of their pile:
    assert_eq![p1.root_position(), Position(1, 1)];
    assert_eq![p2.root_position(), Position(3, 3)];

    // # translate position

    // let's create a square of Size(5, 5) at Position(10, 10):
    let p1 = Plane::new_sized_at(&mut nc, (5, 5), (10, 10))?;

    // check top-left and bottom-right square coordinates are inside the plane:
    assert_eq![p1.translate_root((10, 10)), (Position(0, 0), true)];
    assert_eq![p1.translate_root((14, 14)), (Position(4, 4), true)];

    // some other coordinates outside the plane:
    assert_eq![p1.translate_root((2, 2)), (Position(-8, -8), false)];
    assert_eq![p1.translate_root((20, 20)), (Position(10, 10), false)];

    // # cursor

    Ok(())
}
