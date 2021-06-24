//! Shows how you can manipulate [`Plane`]s.

use notcurses::*;

fn main() -> NResult<()> {
    let mut nc = Notcurses::new()?;

    let mut p1 = Plane::build().rows(20).cols(40).new_pile(&mut nc)?;
    p1.set_base(
        "l",
        Style::BOLD | Style::ITALIC | Style::STRUCK,
        Channels::new(0x88aa00, 0x222288),
    )?;
    p1.show()?;
    sleep![0, 500];

    let mut p2 = Plane::build()
        .rows(10)
        .cols(10)
        .y(2)
        .x(3)
        .into_pile(&mut p1)?;
    p2.set_base("^", Style::UNDERLINE, Channels::new(0x332244, Rgb::GREY))?;
    p1.show()?;
    sleep![0, 500];

    for _ in 0..10 {
        p2.move_rel(1, 3)?;
        p1.show()?;
        sleep![0, 50];
    }
    sleep![2];

    Ok(())
}
