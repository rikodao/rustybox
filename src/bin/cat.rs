extern crate clap;

use std::{
    env::{self, Args},
    fs::File,
    io::Read,
    path::PathBuf,
};

use clap::{App, AppSettings, Arg};
#[cfg_attr(test, allow(dead_code))]
fn main() {
    Cat::new_from_cli(&mut env::args()).handle();
}

pub struct CatOption {
    pub(crate) all: bool,
}
impl CatOption {
    pub(crate) fn new() -> Self {
        Self { all: false }
    }
    pub(crate) fn all(mut self, all: bool) -> Self {
        self.all = all;
        self
    }
    pub(crate) fn build(self) -> CatOption {
        let Self { all } = self;
        CatOption { all }
    }
}
pub struct Cat {
    path: PathBuf,
    option: CatOption,
}

impl Cat {
    // コンストラクタ
    pub fn new(path: PathBuf, option: CatOption) -> Self {
        Cat { path, option }
    }

    pub(crate) fn new_from_cli(args: &mut Args) -> Self {
        let matches = App::new("cat")
            .about("rustybox")
            .version("0.1.0")
            .author("rikodao <rikodao@gmail.com>")
            .setting(AppSettings::VersionlessSubcommands)
            .arg(Arg::with_name("PATH").help("ファイル名").required(true))
            .get_matches_from(args);

        let builder = CatOption::new();

        let option = builder.build();
        let path = matches
            .value_of("PATH")
            .expect("ファイルを指定してください");

        let path_buf = PathBuf::from(path);
        Self::new(path_buf, option)
    }

    // メソッド
    pub(crate) fn handle(self) {
        let mut f = File::open(self.path).expect("file not found");
        let mut content = String::new();
        f.read_to_string(&mut content);
        println!("{}", content);
    }
}
