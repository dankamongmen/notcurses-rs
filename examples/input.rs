// noturses::examples::input

use notcurses::*;
use std::time::Instant;

fn main() -> Result<()> {
    let mut nc = Notcurses::new()?;
    nc.mice_enable(MiceEvents::All)?;

    let mut plane = Plane::new(&mut nc)?;
    plane.set_scrolling(true);

    // blocking

    putstrln!(+render plane,
        "Waiting for a blocking input event. Do anything to continue:"
    )?;

    let event = nc.get_event()?;
    putstrln![+render plane, "{event:?}"]?;

    // non-blocking

    putstrln!(+render plane,
        "\n{0}\nStarting non-blocking event loop. Press `F01` to exit:\n{}\n",
        "-".repeat(50)
    )?;

    let mut counting_time = Instant::now();
    loop {
        let event = nc.poll_event()?;

        if event.received() {
            putstrln![+render plane, "\n{event:?}"]?;

            if event.is_key(Key::F01) {
                putstr![+render plane, "\nBye!"]?;
                sleep![0, 500];
                for _ in 0..3 {
                    putstr![+render plane, " ."]?;
                    sleep![0, 50];
                }
                sleep![0, 250];
                break;
            }
        }

        // do other things in-between
        if counting_time.elapsed().as_millis() > 100 {
            putstr![+render plane, "."]?;
            counting_time = Instant::now()
        }
    }

    Ok(())
}
