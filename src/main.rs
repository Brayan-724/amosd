use std::error::Error;
use amosd_ui::*;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = MainWindow::new()?;

    ui.run()?;
    Ok(())
}
