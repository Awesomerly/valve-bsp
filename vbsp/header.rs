use binrw::binrw; // #[binrw] attribute

#[binrw]
#[derive(Debug)]
#[br(little, magic = b"VBSP")]
pub struct BSPHeader {
    pub version: i32,
    #[br(count = 64)]
    pub lumps: Vec<Lump>,
    pub map_revision: i32,
}

#[binrw]
#[derive(Debug)]
#[br(little)]
pub struct Lump {
    pub fileofs: i32,
    pub filelen: i32,
    pub version: i32,
    pub four_cc: [u8; 4],
}
