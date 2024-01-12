use binrw::BinReaderExt; // trait for reading
use std::env::args;
use std::fs::File;
use vbsp::header;

fn main() {
    let dir = args().nth(1).expect("No path given");
    let mut file = File::open(dir).unwrap();
    let header = file.read_be::<header::BSPHeader>().unwrap();
    println!("{:#?}", header);
    println!("{:#?}", header.lumps[35])
}
