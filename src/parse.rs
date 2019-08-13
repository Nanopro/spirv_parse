use crate::raw::*;
use serde::Serialize;
use serde_json::to_string;
use std::collections::HashMap;

mod types;
mod util;


use self::types::*;
use serde_json::map::Entry;


#[derive(Debug, Clone, Serialize)]
pub struct Spirv {
    pub version: (u8, u8),
    pub bound: u32,
    pub instructions: Vec<Instruction>,
    pub decorations: Vec<(u32, Option<u32>, Decoration)>,
}


pub fn parse_spirv(i: &[u32]) -> Result<Spirv, ParseError> {
    if i.len() < 5 {
        return Err(ParseError::MissingHeader);
    }

    if i[0] as u64 != MAGIC {
        return Err(ParseError::WrongHeader);
    }

    let version = (((i[1] & 0x00ff0000) >> 16) as u8, ((i[1] & 0x0000ff00) >> 8) as u8);
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

    let decorations = instructions.iter().filter_map(|instruction| {
        match &instruction {
            &Instruction::Decorate(id, decoration) => Some((id.0, None, decoration.clone())),
            &Instruction::MemberDecorate(id, member, decoration) => Some((id.0, Some(member.0), decoration.clone())),
            _ => None
        }
    }).collect::<Vec<_>>();

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
    let bytes = data.iter()
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

pub(crate) struct FoundDecoration {
    pub target_id: u32,
    pub params: Vec<u32>,
}

impl Spirv {
    pub fn parse(words: &[u32]) -> Result<Self, ParseError> {
        parse_spirv(words)
    }

    pub fn entry_points(&self) -> Vec<EntryPoint>{
        let mut res = vec![];
        for instruction in &self.instructions{
            match instruction{
                Instruction::EntryPoint(model, _function, name,interface) => {
                    let interface = interface.iter().map(|id_ref| id_ref.0 as u32).collect();
                    res.push(
                        EntryPoint{
                            name: name.0.clone(),
                            interface,
                            execution_model: model.clone()
                        }
                    )
                },
                _ => (),
            }
        }
        res
    }
    pub fn main_entry_point(&self) -> EntryPoint{
        let mut entries = self.entry_points();
        match entries.len(){
            0 => panic!("Shader have no entry point"),
            1 => {
                entries.pop().unwrap()
            },
            _ => {
                for entry in entries{
                    if entry.name == "main"{
                        return entry
                    }
                }
                panic!("Shader entry point's name is not \"main\".")
            }
        }
    }
    pub fn input_variables(&self, entry: &EntryPoint) -> Vec<InterfaceVariable>{
        let mut input = vec![];
        for &interface in &entry.interface{
            for instruction in &self.instructions{
                match instruction{
                    Instruction::Variable(id_result_type, id_result,storage, _) => {

                    },
                    _ => ()
                }

            }
        }


        input
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test() {
        let bytes = include_bytes!("../tests/pos_norm_col.spirv");
        let words = unsafe {
            std::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4)
        };

        let res = parse_spirv(words).unwrap();

        let s = to_string(&res).unwrap();


        let mut file = File::create("./tests/pos_norm_col.json").unwrap();
        file.write(s.as_bytes());
    }
    #[test]
    fn test_entries() {
        let bytes = include_bytes!("../tests/pos_norm_col.spirv");
        let words = unsafe {
            std::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4)
        };

        let res = parse_spirv(words).unwrap();
        let main = res.main_entry_point();
        assert_eq!(
            &main.name,
            "main"
        );
        assert_eq!(
            main.execution_model,
            ExecutionModel::Vertex
        )
    }


}
