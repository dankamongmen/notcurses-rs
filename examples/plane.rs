// notcurses::examples::plane
//
//!
//

#![allow(unused_variables, unused_mut)]

use notcurses::*;

fn main() -> Result<()> {
    let mut nc = Notcurses::new_cli()?;

    // # constructors

    // create a root plane at (1, 1), with a child at (2, 2)
    let mut rootp = Plane::new_at(&mut nc, (1, 1))?;
    let child = rootp.new_child_at((2, 2))?;

    // check their position relative to their parent
    assert_eq![rootp.position(), Position(1, 1)];
    assert_eq![child.position(), Position(2, 2)];

    // check their position relative to the root of their pile
    assert_eq![rootp.root_position(), Position(1, 1)]; // same for a root plane
    assert_eq![child.root_position(), Position(3, 3)];

    // # translate position coordinates

    // create a square of Size(5, 5) at Position(10, 10)
    let size = Size(5, 5);
    let top_left = Position(10, 10);
    let p1 = Plane::new_sized_at(&mut nc, size, top_left)?;

    // check top-left and bottom-right square coordinates are inside the plane:
    assert_eq![p1.translate_root(top_left), (Position(0, 0), true)];
    assert_eq![p1.translate_root((14, 14)), (Position(4, 4), true)];
    // assert_eq![p1.translate_root(top_left + size -1), (Position(4, 4), true)];

    // some other coordinates outside the plane:
    assert_eq![p1.translate_root((2, 2)), (Position(-8, -8), false)];
    assert_eq![p1.translate_root((20, 20)), (Position(10, 10), false)];

    // # cursor
    // ...

    // # strings
    // let mut p1 = Plane::new(&mut nc)?;
    let mut p1 = Plane::new_sized(&mut nc, (4, 4))?;
    p1.set_scrolling(true);

    assert_eq!["hello world".len(), p1.putstr("hello world")?];

    p1.render()?;

    Ok(())
}
