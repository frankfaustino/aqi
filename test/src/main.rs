use std::env::args;
use std::process::exit;

fn main() {
    let arg_count = args().len();

    println!("{}", args().nth(0).unwrap());

    if arg_count != 3 {
        println!("Exactly 2 arguments required, got {}", arg_count);
        exit(exitcode::USAGE);
    }

    let who = args().nth(1).expect("missing 'who' argument");
    let lang = args().nth(2).expect("missing 'lang' argument");

    match lang.as_str() {
        "en" => println!("Hello, {}", who),
        "fr" => println!("Bonjour, {}", who),
        "de" => println!("Hallo, {}", who),
        _ => println!("unknown language '{}'", lang)
    }    
}
