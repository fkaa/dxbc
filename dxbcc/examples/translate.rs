extern crate dxbcc;
extern crate dxbc;
extern crate rspirv;
extern crate pretty_hex;

use dxbc::dr::rdef::*;
use dxbc::dr::isgn::*;

use dxbcc::*;

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
        rd11: Some([0u32; 7]),
    };

    let isgn = IOsgnChunk {
        elements: vec![InputOutputElement {
            name: &"COLOR",
            semantic_index: 0,
            semantic_type: 0,
            component_type: 3,
            register: 0,
            component_mask: 15,
            rw_mask: 15,
        }],
    };

    let osgn = IOsgnChunk {
        elements: vec![InputOutputElement {
            name: &"SV_Position",
            semantic_index: 0,
            semantic_type: 1,
            component_type: 3,
            register: 0,
            component_mask: 15,
            rw_mask: 0,
        }],
    };

    let mut shex = ShexChunk::new();

    shex.add_instruction(Instruction::DclGlobalFlags {
        flags: GlobalFlags::REFACTORING_ALLOWED,
    });
    shex.add_instruction(Instruction::DclInput {
        register: Operand::input(
            0,
            Modifier::None,
            NumComponent::D4(ComponentMode::Mask(X | Y | Z | W))
        ),
    });
    shex.add_instruction(Instruction::DclOutputSiv {
        register: Operand::output(
            0,
            Modifier::None,
            NumComponent::D4(ComponentMode::Mask(X | Y | Z | W))
        ),
        semantic: Semantic::Position,
    });
    /*shex.add_instruction(Instruction::DclTemps {
        count: 1,
    });*/
    shex.add_instruction(Instruction::Add {
        dest: Operand::output(
            0,
            Modifier::None,
            NumComponent::D4(ComponentMode::Mask(X | Y | Z | W)),
        ),
        a: Operand::input(
            0,
            Modifier::None,
            NumComponent::D4(ComponentMode::Swizzle(X, Y, Z, W)),
        ),
        b: Operand::input(
            0,
            Modifier::AbsNeg,
            NumComponent::D4(ComponentMode::Swizzle(X, X, Y, Y)),
        ),
        saturated: false,
    });
    shex.add_instruction(Instruction::Ret);


    let mut builder = dxbcc::Builder::new();
    builder.set_rdef(rdef);
    builder.set_isgn(isgn);
    builder.set_osgn(osgn);
    builder.set_shex(shex);

    let module = builder.module().unwrap();

    let bytes = unsafe { std::slice::from_raw_parts(module.dwords.as_ptr() as _, module.dwords.len() * 4) };

    use std::io::Write;
    use std::fs::File;
    File::create("..\\dxbcd\\assembled.dxbc").unwrap().write_all(bytes).unwrap();

    println!("{:?}", bytes.hex_dump());

    let spirv = include_bytes!("shader.spirv");

    // println!("{:?}", spirv.as_ref().hex_dump());
}
