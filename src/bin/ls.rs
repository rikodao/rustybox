use std::{env, fs};

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    exe(Some(command.to_string()))
}

pub fn exe(dir_name: Option<String>) {
    match dir_name {
        None => ls(".".to_string()),
        Some(dir_name) => ls(dir_name),
    }
}

fn ls(dir_name: String) {
    let directory = fs::read_dir(dir_name);
    match directory {
        Ok(files) => {
            files.for_each(|file| {
                match file {
                    Ok(f) => {
                        let name = f.file_name().into_string().unwrap();
                        println!("{}", name);
                        // matchの無限ループになる
                        // match name {
                        //     Ok(name)=>{println!("{}",name);},
                        //     Err(err)=>{eprintln!("{}",err.into_string(););}
                        // }
                    }
                    Err(err) => {
                        eprintln!("{}", err)
                    }
                }
            });
        }
        Err(err) => {
            eprintln!("{}", err)
        }
    }
}
