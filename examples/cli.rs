// noturses::examples::cli

use notcurses::*;

fn main() -> NotcursesResult<()> {
    let mut nc = Notcurses::new_cli()?;
    let mut cli = nc.cli_plane()?;

    putstrln![cli, "{cli:?}"]?;

    cli.set_fg(0xDE935F);
    putstr![cli, "·←cursor:{} ", cli.cursor()]?;
    putstrln![cli, "·← cursor {}", cli.cursor()]?;
    cli.unset_fg();

    Ok(())
}
