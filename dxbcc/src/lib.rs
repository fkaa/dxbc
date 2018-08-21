extern crate dxbc;
extern crate byteorder;

use byteorder::{ByteOrder, LittleEndian};

const DXBC_MAGIC: u32 = 0x44584243;
const RDEF_MAGIC: u32 = 0x52444546;

pub struct Builder<'a> {
    rdef: Option<dxbc::dr::RdefChunk<'a>>,
    isgn: Option<dxbc::dr::IOsgnChunk<'a>>,
    osgn: Option<dxbc::dr::IOsgnChunk<'a>>,
    code: Vec<u32>,
}

pub struct DxbcModule {
    pub dwords: Vec<u32>,
}

impl DxbcModule {
    pub fn new() -> Self {
        DxbcModule {

        }
    }

    pub(crate) fn write_u32(&mut self, val: u32) {
        self.dwords.write_u32::<LittleEndian>(val).unwrap();
    }

}

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Builder {
            rdef: None,
            isgn: None,
            osgn: None,
            code: Vec::new(),
        }
    }

    pub fn set_rdef(&mut self, rdef: dxbc::dr::RdefChunk<'a>) {
        self.rdef = Some(rdef);
    }

    pub fn set_isgn(&mut self, isgn: dxbc::dr::IOsgnChunk<'a>) {
        self.isgn = Some(isgn);
    }

    pub fn set_osgn(&mut self, osgn: dxbc::dr::IOsgnChunk<'a>) {
        self.osgn = Some(osgn);
    }

    pub fn set_profile(&mut self) {

    }

    pub fn module(&self) -> Result<DxbcModule, ()> {
        let module = DxbcModule::new();


        dwords.write_u32(DXBC_MAGIC);
        dwords.write_u32(0);

        if let Some(rdef) = self.rdef {
            // TODO: rdef chunk

            dwords.write_u32(RDEF_MAGIC);
            dwords.write_u32(0);
            // dwords.write_u32(rdef_chunk.len());

            dwords.write_u32_slice(rdef_chunk);
        }

        dwords.write_u32(RDEF_MAGIC);
        dwords.write_u32(rdef_chunk.len());
    }
}


