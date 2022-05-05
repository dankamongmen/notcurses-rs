// noturses::examples::events

use notcurses::*;
use std::time::Instant;

fn main() -> Result<()> {
    let mut nc = Notcurses::new()?;
    nc.mice_enable(MiceEvents::All)?;

    let mut plane = Plane::new(&mut nc)?;
    plane.set_scrolling(true);

    // blocking

    printstrln!(
        plane,
        "Waiting for a blocking input event. Do anything to continue:"
    )?;

    let event = nc.get_event()?;
    printstrln![plane, ">> {event:?}"]?;

    // non-blocking

    printstrln!(
        plane,
        "\nStarting non-blocking event loop. Press `F01` to exit:\n"
    )?;

    let mut counting_time = Instant::now();
    loop {
        let event = nc.poll_event()?;

        if event.is_received() {
            printstrln![plane, "\n{event:?}"]?;

            if event.is_key(Key::F01) {
                printstr![plane, "\nBye!"]?;
                sleep![0, 500];
                for _ in 0..3 {
                    printstr![plane, " ."]?;
                    sleep![0, 250];
                }
                sleep![1];
                break;
            }
        }

        // do other things in-between
        if counting_time.elapsed().as_millis() > 100 {
            printstr![plane, "."]?;
            counting_time = Instant::now()
        }
    }

    Ok(())
}
