use std::{fs::File, io::Write};

fn main() {
    let buf = std::fs::read_to_string("file").unwrap();
    let mut vec: Vec<_> = buf.split('\n').map(|f|f.trim()).collect();
    println!("{}", vec.len());
    vec.dedup();
    println!("{}", vec.len());
    let mut f = File::create("new").unwrap();
    for item in vec {
        f.write_all(format!("{}\n", item.trim()).as_bytes()).unwrap();
    }
}
