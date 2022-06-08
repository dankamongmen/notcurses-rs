use notcurses::*;

fn main() -> Result<()> {
    let mut nc = Notcurses::new_cli()?;
    let mut cli = nc.cli_plane()?;
    cli.putstr("\nhello world!\n")?;
    cli.render()?;
    Ok(())
}
