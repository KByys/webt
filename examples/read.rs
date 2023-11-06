use std::{
    fs::File,
    io::{BufRead, BufReader},
};
fn main() {
    let f = File::open("content_type.txt").unwrap();
    let mut reader = BufReader::new(f);
    // let mut buf = Vec::new();
    let mut lines = Vec::new();
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).unwrap() == 0 {
            break;
        }
        let mut texts: Vec<_> = line.split(" => ").map(|f| f.trim().to_string()).collect();
        texts[1].pop();
        lines.push(texts.pop().unwrap());
    }
    println!("{}", lines.len());
    lines.dedup();
    println!("{}", lines.len());

    // std::fs::write("file", buf).unwrap();

}
