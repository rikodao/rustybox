mod bin;
pub fn controller(command: &str) {
    match command {
        "ls" => bin::ls::exe(Some(".".to_string())),
        _ => println!("Not Defined"),
    }
}
