use ferris_says::say;
use std::io::{stdout, BufWriter};


fn main() {
    let stdout = stdout();
    let message = String::from("Jimminy Crickets, Chimbabwa is a Chimblabla!");
    let width = message.chars().count();

    let writer = BufWriter::new(stdout.lock());

    say(&message, width/2, writer).unwrap();
}
