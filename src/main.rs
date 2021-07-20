use std::{env, io};
use list_dirs::printer;
fn main() -> io::Result<()> {
    #[cfg(windows)]
    let _enabled = ansi_term::enable_ansi_support();
    let mut args: Vec<String> = env::args().collect();
    match args.len() {
        1 => args.push(String::from(".")),
        2 => (),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "too many or too less arguments",
            ))
        }
    }
    printer(&args[1])?;
    Ok(())
}
