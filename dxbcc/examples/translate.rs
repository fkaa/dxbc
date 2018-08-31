extern crate dxbcc;
extern crate dxbc;
extern crate rspirv;
extern crate pretty_hex;

use dxbc::dr::rdef::*;
use dxbc::dr::isgn::*;

use pretty_hex::PrettyHex;

fn main() {
    let rdef = RdefChunk {
        constant_buffers: Vec::new(),
        resource_bindings: Vec::new(),
        shader_ty: 1,
        minor: 0,
        major: 5,
        flags: 0,
        author: &"amazing dxbc assembler",
    };

    let isgn = IOsgnChunk {
        elements: vec![],
    };

    let osgn = IOsgnChunk {
        elements: vec![InputOutputElement {
            name: &"SV_Position",
            semantic_index: 0,
            semantic_type: 0,
            component_type: 0,
            register: 0,
            component_mask: 0,
            rw_mask: 0,
        }],
    };

    let mut builder = dxbcc::Builder::new();
    builder.set_rdef(rdef);
    builder.set_isgn(isgn);
    builder.set_osgn(osgn);

    let module = builder.module().unwrap();


    let bytes = unsafe { std::slice::from_raw_parts(module.dwords.as_ptr() as _, module.dwords.len() * 4) };

    println!("{:?}", bytes.hex_dump())
}
