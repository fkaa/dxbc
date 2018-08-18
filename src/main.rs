extern crate byteorder;
#[macro_use]
extern crate bitflags;
extern crate winapi;
extern crate term;

mod dr;
mod binary;

use dr::*;
use binary::*;

use std::mem;

struct DisasmConsumer {
    out: Box<term::StdoutTerminal>,
}

impl DisasmConsumer {
    fn new() -> Self {
        Self {
            out: term::stdout().unwrap()
        }
    }

    fn write_instruction<'a>(&mut self, instruction: &str) {
        self.out.fg(term::color::GREEN).unwrap();
        write!(self.out, "{}", instruction).unwrap();
        self.out.reset().unwrap();
    }

    fn write_mask(&mut self, mask: ComponentMask) {
        if mask.contains(ComponentMask::COMPONENT_MASK_R) {
            write!(self.out, "x").unwrap();
        }
        if mask.contains(ComponentMask::COMPONENT_MASK_G) {
            write!(self.out, "y").unwrap();
        }
        if mask.contains(ComponentMask::COMPONENT_MASK_B) {
            write!(self.out, "z").unwrap();
        }
        if mask.contains(ComponentMask::COMPONENT_MASK_A) {
            write!(self.out, "w").unwrap();
        }
    }

    fn write_operands<'a>(&mut self, operands: &[OperandToken0<'a>]) {
        let len = operands.len();

        for (idx, operand) in operands.iter().enumerate() {
            self.write_operand(operand);
            if idx + 1 != len {
                write!(self.out, ", ").unwrap();
            }
        }

        writeln!(self.out, "").unwrap();
    }

    fn write_operand<'a>(&mut self, operand: &OperandToken0<'a>) {
        let ty = operand.get_operand_type();
        let immediate = operand.get_immediate();
        match ty {
            OperandType::Immediate32 | OperandType::Immediate64 => {
                if let OperandType::Immediate32 = ty {
                    write!(self.out, "l(").unwrap();
                } else {
                    write!(self.out, "d(").unwrap();
                }

                match immediate {
                    Immediate::U32(vals) => {
                        let values = vals.iter().map(|&v| f32::from_bits(v).to_string()).collect::<Vec<_>>();
                        write!(self.out, "{}", values.join(",")).unwrap();
                    },
                    Immediate::U64(vals) => {
                        let values = vals.iter().map(|&v| f64::from_bits(v).to_string()).collect::<Vec<_>>();
                        write!(self.out, "{}", values.join(",")).unwrap();
                    }
                    _ => {}
                }
                write!(self.out, ")").unwrap();
            }
            _ => {

            }
        }

        if let Some(operand) = operand.get_extended_operand() {
            match operand.get_operand_modifier() {
                OperandModifier::None => {},
                OperandModifier::Neg => {
                    write!(self.out, "-").unwrap();
                },
                OperandModifier::Abs => {
                    write!(self.out, "|").unwrap();
                },
                OperandModifier::AbsNeg => {
                    write!(self.out, "-|").unwrap();
                },
            }
        }

        let prefix = match ty {
            OperandType::Temp => "r",
            OperandType::Input => "v",
            OperandType::Output => "o",
            OperandType::ConstantBuffer => "cb",
            _ => unimplemented!()
        };

        write!(self.out, "{}", prefix).unwrap();

        match immediate {
            Immediate::U32(vals) => {
                match vals.len() {
                    1 => {
                        write!(self.out, "{}.", vals[0]).unwrap();
                    },
                    2 => {
                        write!(self.out, "{}[{}].", vals[0], vals[1]).unwrap();
                    },
                    _ => {}
                }
            },
            _ => {}
        }

        fn write_swizzle_component(disasm: &mut DisasmConsumer, val: ComponentName) {
            match val {
                ComponentName::X => write!(disasm.out, "x").unwrap(),
                ComponentName::Y => write!(disasm.out, "y").unwrap(),
                ComponentName::Z => write!(disasm.out, "z").unwrap(),
                ComponentName::W => write!(disasm.out, "w").unwrap(),
            }
        }

        match operand.get_component_select_mode() {
            ComponentSelectMode::Mask => {
                let mask = operand.get_component_mask();

                if mask.contains(ComponentMask::COMPONENT_MASK_R) {
                    write!(self.out, "x").unwrap();
                }
                if mask.contains(ComponentMask::COMPONENT_MASK_G) {
                    write!(self.out, "y").unwrap();
                }
                if mask.contains(ComponentMask::COMPONENT_MASK_B) {
                    write!(self.out, "z").unwrap();
                }
                if mask.contains(ComponentMask::COMPONENT_MASK_A) {
                    write!(self.out, "w").unwrap();
                }
            }
            ComponentSelectMode::Swizzle => {
                let swizzle = operand.get_component_swizzle();

                write_swizzle_component(self, swizzle.0);
                write_swizzle_component(self, swizzle.1);
                write_swizzle_component(self, swizzle.2);
                write_swizzle_component(self, swizzle.3);
            }
            ComponentSelectMode::Select1 => {

            }
        }

        if let Some(operand) = operand.get_extended_operand() {
            match operand.get_operand_modifier() {
                OperandModifier::None => {},
                OperandModifier::Neg => {},
                OperandModifier::Abs | OperandModifier::AbsNeg => {
                    write!(self.out, "|").unwrap();
                },
            }
        }
    }
}

impl Consumer for DisasmConsumer {
    fn initialize(&mut self) -> Action {
        self.out.fg(term::color::WHITE).unwrap();

        Action::Continue
    }

    fn finalize(&mut self) -> Action {
        self.out.reset().unwrap();

        Action::Continue
    }

    fn consume_header(&mut self, header: &dr::DxbcHeader) -> Action {
        Action::Continue
    }

    fn consume_rdef(&mut self, rdef: &dr::RdefChunk) -> Action {

        self.out.fg(term::color::BRIGHT_BLACK).unwrap();
        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "// Generated by {}", rdef.author).unwrap();
        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "// Buffer Definitions:").unwrap();

        writeln!(self.out, "//").unwrap();
        for cb in &rdef.constant_buffers {
            writeln!(self.out, "// cbuffer {}", cb.name).unwrap();
            writeln!(self.out, "// {{").unwrap();
            writeln!(self.out, "// }}").unwrap();
        }
        writeln!(self.out, "//").unwrap();

        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "// Resource Bindings:").unwrap();
        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "// Name                                 Type  Format         Dim      HLSL Bind  Count").unwrap();
        writeln!(self.out, "// ------------------------------ ---------- ------- ----------- -------------- ------").unwrap();

        for bind in &rdef.resource_bindings {
            // writeln!(self.out, "// {:30} {:10}", bind.name, return_type, bind.).unwrap();
        }
        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "//").unwrap();

        self.out.reset().unwrap();

        Action::Continue
    }

    fn consume_isgn(&mut self, isgn: &dr::IOsgnChunk) -> Action {
        self.out.fg(term::color::BRIGHT_BLACK).unwrap();

        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "// Input signature:").unwrap();
        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "// Name                 Index   Mask Register SysValue  Format   Used").unwrap();
        writeln!(self.out, "// -------------------- ----- ------ -------- -------- ------- ------").unwrap();

        for elem in &isgn.elements {
            writeln!(
                self.out,
                "// {:20} {:5} {:6} {:8} {:8} {:7} {:6}",
                elem.name,
                elem.semantic_index,
                elem.component_type,
                elem.register,
                "NONE",
                elem.component_type,
                elem.rw_mask,
            ).unwrap();
        }
        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "//").unwrap();

        self.out.reset().unwrap();

        Action::Continue
    }

    fn consume_osgn(&mut self, osgn: &dr::IOsgnChunk) -> Action {

        Action::Continue
    }

    fn consume_shex(&mut self, osgn: &dr::ShexHeader) -> Action {

        Action::Continue
    }

    fn consume_instruction(&mut self, instruction: dr::Instruction) -> Action
    {
        use dr::Instruction::*;

        match instruction {
            DclGlobalFlags(flags) => {
                self.write_instruction("dcl_globalFlags");

                if flags.is_refactoring_allowed() {
                    write!(self.out, "{}", " refactoringAllowed").unwrap();
                }
                writeln!(self.out, "").unwrap();
            }
            DclInput(input) => {
                self.write_instruction("dcl_input");
                write!(self.out, " v{}.", input.get_input_register()).unwrap();
                self.write_mask(input.operand.get_component_mask());
                writeln!(self.out, "").unwrap();
            }
            DclOutput(output) => {
                self.write_instruction("dcl_output");
                write!(self.out, " o{}.", output.get_output_register()).unwrap();
                self.write_mask(output.operand.get_component_mask());
                writeln!(self.out, "").unwrap();
            }
            DclConstantBuffer(cb) => {
                self.write_instruction("dcl_constantbuffer");

                writeln!(self.out, " CB{}[{}], {:?}", cb.get_binding(), cb.get_size(), cb.get_access_pattern()).unwrap();
            }
            DclTemps(temps) => {
                self.write_instruction("dcl_temps");

                writeln!(self.out, " {}", temps.register_count).unwrap();
            }
            DclOutputSiv(siv) => {
                self.write_instruction("dcl_output_siv");
                write!(self.out, " o{}.", siv.get_output_register()).unwrap();
                self.write_mask(siv.operand.get_component_mask());
                writeln!(self.out, ", {:?}", siv.get_system_name()).unwrap();
            },
            Mul(mul) => {
                self.write_instruction("mul ");
                self.write_operands(&[mul.dst, mul.a, mul.b]);
            },
            Mad(mad) => {
                self.write_instruction("mad ");
                self.write_operands(&[mad.dst, mad.a, mad.b, mad.c]);
            }
            Mov(mov) => {
                self.write_instruction("mov ");
                self.write_operands(&[mov.dst, mov.src]);
                println!("  {:#?}", mov.dst);
                println!("  {:#?}", mov.src);
            }
            _ => {
                println!("  {:?}", instruction);
            }
        }

        self.out.reset().unwrap();

        Action::Continue
    }
}

fn main() {
    let shader_bytes = include_bytes!("..\\shader.dxbc");

    let mut consumer = DisasmConsumer::new();
    let mut parser = Parser::new(shader_bytes, &mut consumer);

    parser.parse().unwrap();
}
