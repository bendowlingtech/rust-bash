mod commands;



fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    match args.get(1).map(String::as_str) {
        "list" => commands::
    }
}
