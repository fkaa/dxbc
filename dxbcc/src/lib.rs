extern crate dxbc;
extern crate byteorder;

use byteorder::{ByteOrder, LittleEndian};

const DXBC_MAGIC: u32 = 0x44584243;
const RDEF_MAGIC: u32 = 0x52444546;
const ISGN_MAGIC: u32 = 0x4953574e;
const OSGN_MAGIC: u32 = 0x4f53474e;

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
            dwords: Vec::new()
        }
    }

    pub fn position(&self) -> usize {
        self.dwords.len()
    }

    pub fn write_u32(&mut self, val: u32) {
        self.dwords.push(val);
    }

    pub fn set_u32_slice(&mut self, offset: usize, val: &[u32]) {
        self.dwords[offset..].copy_from_slice(val);
    }

    pub fn set_u32(&mut self, offset: usize, val: u32) {
        self.dwords[offset] = val;
    }

    pub fn write_str(&mut self, text: &str) -> u32 {
        let mut len = 0;

        for chunk in text.as_bytes().chunks(4) {
            let data = match chunk {
                &[a, b, c, d] => ((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8) | d as u32,
                &[a, b, c] => ((a as u32) << 24) | ((b as u32) << 16) | ((c  as u32) << 8),
                &[a, b] => ((a as u32) << 24) | ((b as u32) << 16),
                &[a] => (a as u32) << 24,
                _ => unreachable!()
            };

            self.write_u32(data);

            len += 4
        }

        // if string is aligned we append padded null-terminator
        if text.len() & 3 == 0 {
            self.write_u32(0);

            len += 4;
        }

        len
    }

    pub fn write_iosgn(&mut self, chunk: &dxbc::dr::IOsgnChunk, magic: u32) {

    }

    pub fn write_isgn(&mut self, chunk: &dxbc::dr::IOsgnChunk) {
        self.write_iosgn(chunk, ISGN_MAGIC);
    }

    pub fn write_osgn(&mut self, chunk: &dxbc::dr::IOsgnChunk) {
        self.write_iosgn(chunk, OSGN_MAGIC);
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.dwords.as_ptr() as *const u8,
                self.dwords.len() * std::mem::size_of::<u32>(),
            )
        }
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
        let mut module = DxbcModule::new();

        module.write_u32(DXBC_MAGIC);
        let checksum_pos = module.position();
        module.write_u32(0);
        module.write_u32(0);
        module.write_u32(0);
        module.write_u32(0);
        let size_pos = module.position();

        let mut chunk_count = 0;
        

        for _ in 0..chunk_count {
            module.write_u32(0);
        }

        if let Some(ref rdef) = self.rdef {
            module.write_u32(RDEF_MAGIC);
            let rdef_size_pos = module.position();
            module.write_u32(0);
            let chunk_start = module.position();
            module.write_u32(rdef.constant_buffers.len() as u32);
            let constant_buffers_pos = module.position();
            module.write_u32(0);
            module.write_u32(rdef.resource_bindings.len() as u32);
            let resource_bindings_pos = module.position();
            module.write_u32(0);

            let version_tok = (((rdef.shader_ty as u32) << 16) & 0xffff0000) |
                              (((rdef.major as u32) << 4) & 0x000000f0) |
                              (rdef.minor as u32 & 0x0000000f);
            module.write_u32(version_tok);
            module.write_u32(rdef.flags);
            let author_pos = module.position();
            module.write_u32(0);

            let constant_buffers_loc = (module.position() - chunk_start) as u32;
            module.set_u32(constant_buffers_pos, constant_buffers_loc);
            for constant_buffer in &rdef.constant_buffers {
                // TODO: 
            }

            let resource_bindings_loc = (module.position() - chunk_start) as u32;
            module.set_u32(resource_bindings_pos, resource_bindings_loc);
            for resource_binding in &rdef.resource_bindings {
                // TODO: 
            }

            let author_loc = (module.position() - chunk_start) as u32;
            module.set_u32(author_pos, author_loc);
            module.write_str(rdef.author);
        }

        if let Some(ref osgn) = self.osgn {
            module.write_osgn(osgn);
        }

        // finally, patch in size and checksum
        let len = module.dwords.len() as u32 - 5;
        module.set_u32(size_pos, len);
        let checksum = dxbc::checksum(module.as_bytes());
        module.set_u32(checksum_pos,     checksum[0]);
        module.set_u32(checksum_pos + 1, checksum[1]);
        module.set_u32(checksum_pos + 2, checksum[2]);
        module.set_u32(checksum_pos + 3, checksum[3]);

        Err(())
    }
}


