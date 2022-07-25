// notcurses::examples::info

use notcurses::*;

fn main() -> Result<()> {
    let mut nc = Notcurses::new_cli()?;
    let mut cli = nc.cli_plane()?;

    let caps = nc.capabilities();
    let styles = nc.supported_styles();

    putstrln!(cli, "\n{caps:#?}")?;
    putstrln!(
        cli,
        "\nSupported styles: {}.",
        styles.to_string().replace(" ", ", ")
    )?;

    Ok(())
}
