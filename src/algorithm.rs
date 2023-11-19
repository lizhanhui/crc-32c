#[derive(Debug, Clone)]
pub struct Algorithm {
    pub width: u8,
    pub poly: u32,
    pub init: u32,
    pub refin: bool,
    pub refout: bool,
    pub xorout: u32,
    pub check: u32,
    pub residue: u32,
}

pub const CRC_32_ISCSI: Algorithm = Algorithm {
    width: 32,
    poly: 0x1edc6f41,
    init: 0xffffffff,
    refin: true,
    refout: true,
    xorout: 0xffffffff,
    check: 0xe3069283,
    residue: 0xb798b438,
};
