// noturses::examples::events

use notcurses::*;
use std::time::{Duration, Instant};

fn main() -> Result<()> {
    let mut nc = Notcurses::new()?;
    nc.mice_enable(MiceEvents::All)?;

    let mut plane = Plane::new(&mut nc)?;
    plane.set_scrolling(true);

    // blocking

    putstrln!(
        plane,
        "Waiting for a blocking input event. Do anything to continue:"
    )?;

    let event = nc.get_event()?;
    putstrln![plane, ">> {event:?}"]?;

    // non-blocking

    putstrln!(
        plane,
        "\nStarting non-blocking event loop. Press `F01` to exit:\n"
    )?;

    let mut counting_time = Instant::now();
    loop {
        let event = nc.poll_event()?;

        if event.is_received() {
            putstrln![plane, "\n{event:?}"]?;

            if event.is_key(Key::F01) {
                putstr![plane, "\nBye!"]?;
                sleep![0, 500];
                for _ in 0..3 {
                    putstr![plane, " ."]?;
                    sleep![0, 250];
                }
                sleep![1];
                break;
            }
        }

        // do other things in-between
        if counting_time.elapsed().as_millis() > 100 {
            putstr![plane, "."]?;
            counting_time = Instant::now()
        }
    }

    Ok(())
}
