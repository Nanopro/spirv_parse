use crate::raw::*;
use serde::Serialize;
use serde_json::to_string;
use std::collections::HashMap;

pub mod types;
mod spirv;


pub use spirv::Spirv;

use self::types::*;
use crate::raw::Capability::ShaderSMBuiltinsNV;
use serde_json::map::Entry;
use std::any::Any;
use std::ops::Deref;



fn parse_instruction(i: &[u32]) -> Result<(Instruction, &[u32]), ParseError> {
    assert!(i.len() >= 1);

    // Opcode: The 16 high-order bits are the WordCount of the
    //          instruction. The 16 low-order bits are the opcode enumerant.
    let word_count = (i[0] >> 16) as usize;
    assert!(word_count >= 1);
    let opcode = (i[0] & 0xffff) as u16;

    if i.len() < word_count {
        return Err(ParseError::IncompleteInstruction);
    }

    let opcode = decode_instruction(opcode, &i[1..word_count])?;
    Ok((opcode, &i[word_count..]))
}

fn decode_instruction(opcode: u16, operands: &[u32]) -> Result<Instruction, ParseError> {
    Ok(Instruction::from_raw(opcode, operands))
}

fn parse_string(data: &[u32]) -> (String, &[u32]) {
    let bytes = data
        .iter()
        .flat_map(|&n| {
            let b1 = (n & 0xff) as u8;
            let b2 = ((n >> 8) & 0xff) as u8;
            let b3 = ((n >> 16) & 0xff) as u8;
            let b4 = ((n >> 24) & 0xff) as u8;
            vec![b1, b2, b3, b4].into_iter()
        })
        .take_while(|&b| b != 0)
        .collect::<Vec<u8>>();

    let r = 1 + bytes.len() / 4;
    let s = String::from_utf8(bytes).expect("Shader content is not UTF-8");

    (s, &data[r..])
}


pub fn parse_spirv(i: &[u32]) -> Result<Spirv, ParseError> {
    if i.len() < 5 {
        return Err(ParseError::MissingHeader);
    }

    if i[0] as u64 != MAGIC {
        return Err(ParseError::WrongHeader);
    }

    let version = (
        ((i[1] & 0x00ff0000) >> 16) as u8,
        ((i[1] & 0x0000ff00) >> 8) as u8,
    );
    if version.0 != VERSION.0 || version.1 > VERSION.1 {
        return Err(ParseError::BadVersion);
    }

    let instructions = {
        let mut ret = Vec::new();
        let mut i = &i[5..];
        while i.len() >= 1 {
            let (instruction, rest) = parse_instruction(i)?;
            ret.push(instruction);
            i = rest;
        }
        ret
    };

    let decorations = instructions
        .iter()
        .filter_map(|instruction| match &instruction {
            &Instruction::Decorate(id, decoration) => Some((id.0, None, decoration.clone())),
            &Instruction::MemberDecorate(id, member, decoration) => {
                Some((id.0, Some(member.0), decoration.clone()))
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    Ok(Spirv {
        version,
        bound: i[3],
        instructions,
        decorations,
    })
}

#[derive(Debug, Clone)]
pub enum ParseError {
    MissingHeader,
    BadVersion,
    WrongHeader,
    IncompleteInstruction,
    UnknownConstant(&'static str, u32),
}



pub(crate) struct FoundDecoration {
    pub target_id: u32,
    pub params: Vec<u32>,
}



#[cfg(test)]
mod tests;
