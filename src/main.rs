use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].as_str();
    rustybox::controller(command);
}
