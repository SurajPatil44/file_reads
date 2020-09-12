use ansi_term::Colour::{Cyan, Red};
use std::{fmt, fs, io};

#[allow(non_camel_case_types)]
#[derive(Debug)]
/// enum for size, will convert sizes to to B,KB and MB automatically
/// B is isize to display -1 for files which can't be read
enum Size_t {
    B(isize),
    KB(f32),
    MB(f32),
}

impl Default for Size_t {
    fn default() -> Self {
        Size_t::B(0)
    }
}

impl fmt::Display for Size_t {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Size_t::B(n) => {
                let out = format!("{} B", n);
                f.pad(&out)
            }
            Size_t::KB(n) => {
                let out = format!("{:.2} KB", n);
                f.pad(&out)
                //write!(f, "{}", String::from(out))
            }
            Size_t::MB(n) => {
                let out = format!("{:.2} MB", n);
                f.pad(&out)
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
/// file type either Directory or simple file
/// NoType for file which are not accessible
enum f_type {
    Dir,
    File,
    NoType,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
/// struct which keeps all the file information
//TODO : permission
struct File_info {
    name: String,
    size: Size_t,
    typ: f_type,
    //permission :
}

#[inline]
fn get_file_type(is_dir: bool) -> f_type {
    match is_dir {
        true => f_type::Dir,
        false => f_type::File,
    }
}

#[inline]
fn handle_error(kind: io::ErrorKind) -> String {
    match kind {
        io::ErrorKind::PermissionDenied => format!("{}", Red.paint("Access is denied")),
        io::ErrorKind::Other => format!("{}", Red.paint("File in use")),
        _ => format!("{}", Red.paint("some unknown error")),
    }
}

#[inline]
fn get_file_size(sz: usize) -> Size_t {
    match sz {
        d if d < 1024 => Size_t::B(d as isize),
        d if d > 1024 && d < (1024 * 1024 - 1) => {
            let f = d as f32 / 1024.0;
            Size_t::KB(f)
        }
        d if d > 1024 * 1024 => {
            let f = d as f32 / (1024.0 * 1024.0);
            Size_t::MB(f)
        }
        _ => Size_t::B(-1),
    }
}

fn get_file_name(ent: &str, typ: &f_type) -> String {
    match typ {
        f_type::Dir => format!("{}", Cyan.paint(ent)),
        f_type::File => format!("{}", ent),
        _ => todo!(),
    }
}

fn run(path: &str) -> Result<Vec<File_info>, io::Error> {
    let file_iterator = fs::read_dir(path);
    match file_iterator {
        Err(_e) => return Err(io::Error::new(io::ErrorKind::Other, "Directory not found")),
        Ok(_) => (),
    }
    let entries: Vec<File_info> = file_iterator?
        .map(|res| res.map(|e| e.path()))
        .map(|ent| {
            let ent = ent.unwrap();
            let md = fs::metadata(&ent);
            match md {
                Ok(md) => {
                    let ft = get_file_type(md.is_dir());
                    let sz = get_file_size(md.len() as usize);
                    let name = get_file_name(ent.file_name().unwrap().to_str().unwrap(), &ft);
                    File_info {
                        name,
                        size: sz,
                        typ: ft,
                    }
                }
                Err(e) => {
                    let error = handle_error(e.kind());
                    File_info {
                        name: String::from(error),
                        size: Size_t::B(-1),
                        typ: f_type::NoType,
                    }
                }
            }
        })
        .collect();
    Ok(entries)
}

pub fn printer(path: &str) -> Result<(), std::io::Error> {
    let entries = run(path)?;
    println!("total {}", entries.len());
    for f in entries {
        match f.typ {
            f_type::Dir => println!("({:?}) {:<8} {:<50}", f.typ, f.size, f.name),
            f_type::File => println!("{:<5} {:<8} {:<50}", ' ', f.size, f.name),
            f_type::NoType => println!("{}", f.name),
        }
    }
    /*
    match entries {
        Ok(ent) => {
            for f in ent {
                println!("{:<50} {:<8} {:?}", f.name, f.size, f.typ);
            }
        }
        Err(_) => unreachable!(),
    }
    */
    Ok(())
}
