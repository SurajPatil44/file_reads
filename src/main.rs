use list_dirs::*;
use std::{env, io};

fn main() -> io::Result<()> {
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
    let path = &args[1];
    let entries = run(path)?;
    let header = format!("{:<50} {:<8} {}", "Filename", "Size", "Type");
    println!("{}", header);
    println!("=================================================================================================================");
    println!();
    for f in entries {
        println!("{:<50} {:<8} {:?}", f.name, f.size, f.typ);
        println!("------------------------------------------------------------------------------------------------------------------");
    }

    Ok(())
}
