use std::env::args;
use std::process::exit;

fn main() {
    let args = args();
    if args.len() != 1 {
        eprintln!("Usage: laines game.nes");
        exit(1);
    }

}
