use std::env;
use std::io::Write;

fn print_usage() {
    eprintln!("usage: DEVICE [1|2|3|4]");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {}
        _ => {
            print_usage();
            eprintln!("error: unexpected argument(s)");
            std::process::exit(1);
        }
    }

    let mut port = serialport::new(&args[1], 9600)
        .open()
        .expect("Failed to open port.");
    match &args[2][..] {
        "1" => port.write("1!".as_bytes()).expect("Failure"),
        "2" => port.write("2!".as_bytes()).expect("Failure"),
        "3" => port.write("3!".as_bytes()).expect("Failure"),
        "4" => port.write("4!".as_bytes()).expect("Failure"),
        _ => {
            eprintln!("error: invalid action");
            std::process::exit(1);
        }
    };
}
