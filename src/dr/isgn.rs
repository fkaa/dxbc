use binary::*;

#[repr(C)]
#[derive(Debug)]
pub struct InputOutputElement<'a> {
    pub name: &'a str,
    pub semantic_index: u32,
    pub semantic_type: u32,
    pub component_type: u32,
    pub register: u32,
    pub component_mask: u8,
    pub rw_mask: u8,
}

impl<'a> InputOutputElement<'a> {
    pub fn parse(decoder: &mut decoder::Decoder<'a>) -> Result<Self, State> {
        let name_offset = decoder.read_u32();
        let semantic_index = decoder.read_u32();
        let semantic_type = decoder.read_u32();
        let component_type = decoder.read_u32();
        let register = decoder.read_u32();
        let component_mask = decoder.read_u8();
        let rw_mask = decoder.read_u8();
        decoder.skip(2);

        let name = decoder.seek(name_offset as usize).str().map_err(|e| State::DecoderError(e))?;

        Ok(Self {
            name,
            semantic_index,
            semantic_type,
            component_type,
            register,
            component_mask,
            rw_mask,
        })
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct IOsgnChunk<'a> {
    pub elements: Vec<InputOutputElement<'a>>,
}

impl<'a> IOsgnChunk<'a> {
    pub fn parse<'b>(decoder: &'b mut decoder::Decoder) -> Result<IOsgnChunk<'b>, State> {
        let element_count = decoder.read_u32();
        let _unknown = decoder.read_u32();

        let mut elements = Vec::new();
        for _ in 0..element_count {
            elements.push(InputOutputElement::parse(decoder)?);
        }

        Ok(IOsgnChunk {
            elements,
        })
    }
}
