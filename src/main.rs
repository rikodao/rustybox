use std::{
    env::{self, Args},
    path::Path,
};
mod bin;

fn main() {
    let args = &mut env::args();
    let command = get_command(args);
    match command.as_str() {
        "ls" => {
            bin::ls::Ls::new_from_cli(args).handle();
        }
        _ => {
            eprintln!("実行コマンドが定義されていません。")
        }
    }
}

fn get_command(args: &mut Args) -> String {
    let arg: String = (*args.next().unwrap()).to_string();
    let path = Path::new(&arg);
    let file_name = path.file_name().unwrap().to_string_lossy().to_string();

    if file_name == "main" {
        match args.next() {
            Some(arg) => arg,
            None => {
                eprintln!("コマンドを入力してください。");
                std::process::exit(1);
            }
        }
    } else {
        file_name
    }
}
