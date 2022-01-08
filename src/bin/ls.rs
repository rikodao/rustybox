extern crate clap;

use std::{
    env::{self, Args},
    fs::{self, DirEntry},
    path::PathBuf,
};

use clap::{App, AppSettings, Arg};
#[cfg_attr(test, allow(dead_code))]
fn main() {
    Ls::new_from_cli(&mut env::args()).handle();
}

pub struct LsOption {
    pub(crate) all: bool,
}
impl LsOption {
    pub(crate) fn new() -> Self {
        Self { all: false }
    }
    pub(crate) fn all(mut self, all: bool) -> Self {
        self.all = all;
        self
    }
    pub(crate) fn build(self) -> LsOption {
        let Self { all } = self;
        LsOption { all }
    }
}
pub struct Ls {
    path: PathBuf,
    option: LsOption,
}

impl Ls {
    // コンストラクタ
    pub fn new(path_option: Option<PathBuf>, option: LsOption) -> Self {
        // unwrap_orだと eager load(先行評価)、unwrap_or_elseだと lazy load(遅延評価)になるらしい
        let path: PathBuf = path_option.unwrap_or_else(|| PathBuf::from("."));
        Ls { path, option }
    }

    pub(crate) fn new_from_cli(args: &mut Args) -> Self {
        let matches = App::new("ls")
            .about("rustybox")
            .version("0.1.0")
            .author("rikodao <rikodao@gmail.com>")
            .setting(AppSettings::VersionlessSubcommands)
            .arg(Arg::with_name("PATH").help("パス名"))
            .arg(
                Arg::with_name("all")
                    .short("a")
                    .long("all")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("directory")
                    .short("d")
                    .long("directory")
                    .takes_value(false),
            )
            .get_matches_from(args);

        let mut builder = LsOption::new();

        if matches.is_present("all") {
            builder = builder.all(true);
        }

        let option = builder.build();
        let path = matches
            .value_of("PATH")
            .expect("ファイル名を指定してください。");
        Self::new(Some(PathBuf::from(path)), option)
    }

    // メソッド
    pub(crate) fn handle(self) {
        let dir_name = self.path;
        // let options = self.option;

        let files = fs::read_dir(dir_name);
        match files {
            Ok(files) => {
                let valid_files = files.filter_map(Result::ok);
                // if !options.all {
                //     valid_files = valid_files.filter_map(|f| is_hidden(f));
                // }
                valid_files.for_each(print_files);
            }
            Err(err) => {
                eprintln!("{}", err)
            }
        }
    }
}

fn print_files(file: DirEntry) {
    let name = file.file_name().into_string().unwrap();
    println!("{}", name);
}
// fn is_hidden(file: DirEntry) -> Option<bool> {
//     file.file_name().to_str().filter_map(|s| s.starts_with("."))
// }
