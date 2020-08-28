use std::{fs,io,fmt,env};
use ansi_term::Colour::Red;

#[allow(non_camel_case_types)]
#[derive(Debug)]
/// enum for size, will convert sizes to to B,KB and MB automatically
/// B is isize to display -1 for files which can't be read
enum Size_t {
    B(isize),
    KB(usize),
    MB(usize),
}

impl Default for Size_t {
    fn default() -> Self { Size_t::B(0) }

}
impl fmt::Display for Size_t {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Size_t::B(n) => {
                let out = format!("{} B",n);
                write!(f,"{}",String::from(out))
            }
            Size_t::KB(n) => {
                let out = format!("{} KB",n);
                write!(f,"{}",String::from(out))
            }
            Size_t::MB(n) => {
                let out = format!("{} MB",n);
                write!(f,"{}",String::from(out))
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
    NoType
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
/// struct which keeps all the file information
//TODO : permission
struct File_info {
    name : String,
    size : Size_t,
    typ : f_type,
    //permission : 
}


fn main() -> io::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    match args.len() {
        1 => args.push(String::from(".")),
        2 => (),
        _ => return Err(io::Error::new(io::ErrorKind::Other, "too many or too less arguments"))
    }
    let entries : Vec<File_info> = fs::read_dir(&args[1])?
        .map(|res| res.map(|e| e.path()))
        .map(|ent| {
             let ent = ent.unwrap();
             let md = fs::metadata(&ent);
             match md {

                Ok(md) => {
                    let ft = match md.is_dir() {

                     true => f_type::Dir,
                         false => f_type::File
                    };
                    let sz = md.len() as usize;
                    let sz = match sz {
                         d if d < 1024 =>  Size_t::B(d as isize),
                         d if d > 1024 && d < (1024*1024 - 1) =>  { 
                             let f = (d as f32 / 1024.0) as usize;
                             Size_t::KB(f)
                        }
                        d if d > 1024*1024 => { 
                            let f = (d as f32 / (1024.0*1024.0)) as usize;
                            Size_t::MB(f)
                        }
                        _ => Size_t::B(0),
                    };
                    //ent.pop();
                    //let name = ent;
                
                    File_info { 
                        name : String::from(ent.file_name().unwrap().to_str().unwrap()),
                        //name : String::from(ent.pop()),
                        size : sz,
                        typ  : ft
                    } 
                },
                Err(e) => {
                    let error = match e.kind() {
                        io::ErrorKind::PermissionDenied  => format!("{}",Red.paint("Access is denied")),
                        io::ErrorKind::Other  => format!("{}",Red.paint("File in use")),
                        _ => format!("{}",Red.paint("some unknown error")),
                    };
                    File_info {
                        name : String::from(error),
                        size : Size_t::B(-1),
                        typ  : f_type::NoType
                    }
                }
             }
         })
        .collect();
    
    println!("=================================================================================================================");
    let header = format!("{:<50} {:<8} {}","Filename","size","type");
    println!("{}",header);
    println!("=================================================================================================================");
    println!();
    for f in entries {
        println!("{:<50} {:<8} {:?}",f.name,f.size,f.typ);
        println!("------------------------------------------------------------------------------------------------------------------");
    }
   
    Ok(())
}