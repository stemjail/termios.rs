use std::old_io as io;

fn make() {
    io::Command::new("make").spawn().unwrap();
}

fn main() {
    make()
}
