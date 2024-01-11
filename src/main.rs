use binrw::{
    binrw,   // #[binrw] attribute
    BinRead, // trait for reading
};
use std::env::args;
use std::fs::File;

#[binrw]
#[derive(Debug)]
#[br(little, magic = b"VBSP")]
pub struct BSPHeader {
    pub version: u32,
    #[br(count = 64)]
    pub lumps: Vec<Lump>,
    pub map_revision: u32,
}

#[binrw]
#[derive(Debug)]
#[br(little)]
pub struct Lump {
    pub fileofs: u32,
    pub filelen: u32,
    pub version: u32,
    pub four_cc: u32,
}

fn main() {
    let dir = args().nth(1).expect("No path given");
    let mut file = File::open(dir).unwrap();
    let header = BSPHeader::read(&mut file).unwrap();
    println!("{:#?}", header);
}
