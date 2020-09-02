use ansi_term::Colour::Red;
use std::{fmt, fs, io};

#[allow(non_camel_case_types)]
#[derive(Debug)]
/// enum for size, will convert sizes to to B,KB and MB automatically
/// B is isize to display -1 for files which can't be read
pub enum Size_t {
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
pub enum f_type {
    Dir,
    File,
    NoType,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
/// struct which keeps all the file information
//TODO : permission
pub struct File_info {
    pub name: String,
    pub size: Size_t,
    pub typ: f_type,
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

pub fn run(path: &str) -> Result<Vec<File_info>, io::Error> {
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
                    File_info {
                        name: String::from(ent.file_name().unwrap().to_str().unwrap()),
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
