use crate::raw::*;
use serde::Serialize;
use serde_json::to_string;
use std::collections::HashMap;

mod types;
mod util;

use self::types::*;
use crate::raw::Capability::ShaderSMBuiltinsNV;
use serde_json::map::Entry;
use std::any::Any;

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

pub(crate) struct FoundDecoration {
    pub target_id: u32,
    pub params: Vec<u32>,
}

impl Spirv {
    pub fn parse(words: &[u32]) -> Result<Self, ParseError> {
        parse_spirv(words)
    }

    pub fn entry_points(&self) -> Vec<EntryPoint> {
        let mut res = vec![];
        for instruction in &self.instructions {
            match instruction {
                Instruction::EntryPoint(model, _function, name, interface) => {
                    let interface = interface.iter().map(|id_ref| id_ref.0 as u32).collect();
                    res.push(EntryPoint {
                        name: name.0.clone(),
                        interface,
                        execution_model: model.clone(),
                    })
                }
                _ => (),
            }
        }
        res
    }
    pub fn main_entry_point(&self) -> EntryPoint {
        let mut entries = self.entry_points();
        match entries.len() {
            0 => panic!("Shader have no entry point"),
            1 => entries.pop().unwrap(),
            _ => {
                for entry in entries {
                    if entry.name == "main" {
                        return entry;
                    }
                }
                panic!("Shader entry point's name is not \"main\".")
            }
        }
    }
    fn interface_varibles(
        &self,
        entry: &EntryPoint,
        class: StorageClass,
    ) -> Vec<InterfaceVariable> {
        let mut interfaces = vec![];
        for &interface in &entry.interface {
            for instruction in &self.instructions {
                match instruction {
                    Instruction::Variable(id_result_type, id_result, storage, _)
                    if *storage == class && interface == id_result.0 =>
                        {
                            let ty = self.type_from_id(id_result_type.0);
                            if let Some(Type::Complex(ComplexType::Structure { name, .. })) = &ty {
                                if name == "gl_PerVertex" {
                                    continue;
                                }
                            }
                            let name = self.name_from_id(interface).expect("__unnamed");
                            let offset = self
                                .decorations
                                .iter()
                                .find_map(|(id, member, decoration)| {
                                    if *id == interface {
                                        match decoration {
                                            Decoration::Offset(lit) => Some(lit.0),
                                            _ => None,
                                        }
                                    } else {
                                        None
                                    }
                                })
                                .unwrap_or(0);
                            let location = self
                                .decorations
                                .iter()
                                .find_map(|(id, member, decoration)| {
                                    if *id == interface {
                                        match decoration {
                                            Decoration::Location(lit) => Some(lit.0),
                                            _ => None,
                                        }
                                    } else {
                                        None
                                    }
                                })
                                .unwrap_or(0);

                            match ty {
                                Some(ty @ Type::Simple(_)) => {
                                    interfaces.push(InterfaceVariable {
                                        id: interface,
                                        name,
                                        ty,
                                        location,
                                        storage_class: class.clone(),
                                        offset,
                                    });
                                }
                                Some(ty @ Type::Complex(_)) => {
                                    interfaces.push(InterfaceVariable {
                                        id: interface,
                                        name,
                                        ty,
                                        location,
                                        storage_class: class.clone(),
                                        offset,
                                    });
                                }
                                _ => panic!("Missing type for interface variable #{}", interface),
                            }
                        }
                    _ => (),
                }
            }
        }
        interfaces
    }

    pub fn push_constant_blocks(&self) -> Option<PushConstantBlock> {

        for instruction in &self.instructions {
            match instruction {
                Instruction::Variable(id_result_type, id_result, storage, _)
                if storage == &StorageClass::PushConstant =>
                    {
                        let ty = self.type_from_id(id_result_type.0).unwrap();
                        let name = self.name_from_id(id_result.0).unwrap_or("UnnamedPushConstant".to_owned());
                        let offset = self.decorations.iter().find_map(|(id, member, decoration)|{
                            if *id == id_result.0{
                                match decoration{
                                    Decoration::Offset(lit)=> Some(lit.0),
                                    _ => None,
                                }
                            }else{
                                None
                            }

                        }).unwrap_or(0);
                        return Some(PushConstantBlock{
                            name,
                            ty,
                            offset,
                            id: id_result.0
                        })
                    }
                _ => (),
            }
        }


        None
    }

    pub fn input_variables(&self, entry: &EntryPoint) -> Vec<InterfaceVariable> {
        self.interface_varibles(entry, StorageClass::Input)
    }
    pub fn output_variables(&self, entry: &EntryPoint) -> Vec<InterfaceVariable> {
        self.interface_varibles(entry, StorageClass::Output)
    }


    pub fn type_from_id(&self, id: u32) -> Option<Type> {
        for instruction in &self.instructions {
            match instruction {
                Instruction::TypeBool(id_res) if id_res.0 == id => {
                    return Some(Type::Simple(SimpleType::Boolean));
                }
                Instruction::TypeFloat(id_res, width) if id_res.0 == id => {
                    //TODO! F32 or F64
                    return Some(Type::Simple(SimpleType::Float));
                }
                Instruction::TypeInt(id_res, width, signedness) if id_res.0 == id => {
                    if signedness.0 == 0 {
                        return Some(Type::Simple(SimpleType::UInteger));
                    } else {
                        return Some(Type::Simple(SimpleType::Integer));
                    }
                }
                Instruction::TypeVector(id_res, element, count) if id_res.0 == id => {
                    return Some(Type::Complex(ComplexType::Vector {
                        ty: match self.type_from_id(element.0).unwrap() {
                            Type::Simple(ty) => ty,
                            _ => unimplemented!(),
                        },
                        len: count.0,
                    }));
                }
                Instruction::TypeArray(id_res, element, len) if id_res.0 == id => {
                    return Some(Type::Complex(ComplexType::Array {
                        ty: Box::new(self.type_from_id(element.0).unwrap()),
                        len: match self.const_value(len.0) {
                            Some(ConstValue::UInteger(len)) => len,
                            _ => panic!("Array length must be unsigned integer"),
                        },
                    }));
                }
                Instruction::TypePointer(id_res, class, point_type) if id_res.0 == id => {
                    return self.type_from_id(point_type.0);
                }
                Instruction::TypeStruct(id_res, members) if id_res.0 == id => {
                    return Some(Type::Complex(ComplexType::Structure {
                        name: self.name_from_id(id).unwrap_or_else(|| "UnnamedStructure".to_owned()),
                        members: members
                            .iter()
                            .enumerate()
                            .map(|(n, member)| {
                                (
                                    self.member_name(id_res.0, n as u32)
                                        .unwrap_or_else(|| "__unnamed".to_owned()),
                                    self.type_from_id(member.0).unwrap(),
                                )
                            })
                            .collect(),
                    }));
                }
                Instruction::TypeMatrix(id_res, member, count) if id_res.0 == id => {
                    match self.type_from_id(member.0) {
                        Some(Type::Complex(ComplexType::Vector { ty, len })) => {
                            return Some(Type::Complex(ComplexType::Matrix {
                                ty,
                                rows: count.0,
                                cols: len,
                            }));
                        }
                        _ => unimplemented!()
                    }
                }
                _ => (),
            }
        }
        None
    }

    pub fn const_value(&self, id: u32) -> Option<ConstValue> {
        for instruction in &self.instructions {
            match instruction {
                Instruction::Constant(id_res_type, id_res, value) if id_res.0 == id => {
                    match self.type_from_id(id_res_type.0) {
                        Some(Type::Simple(SimpleType::UInteger)) => {
                            return Some(ConstValue::UInteger(value.0[0]));
                        }
                        x => panic!("Not implemented for {:?}", x),
                    }
                }
                _ => (),
            }
        }
        None
    }

    pub fn name_from_id(&self, id: u32) -> Option<String> {
        for instruction in &self.instructions {
            match instruction {
                Instruction::Name(id_ref, literal) if id_ref.0 == id => {
                    return Some(literal.0.clone());
                }
                _ => (),
            }
        }
        None
    }

    pub fn member_name(&self, id: u32, member: u32) -> Option<String> {
        for instruction in &self.instructions {
            match instruction {
                Instruction::MemberName(id_ref, lit_int, literal)
                if id_ref.0 == id && lit_int.0 == member =>
                    {
                        return Some(literal.0.clone());
                    }
                _ => (),
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use crate::raw::StorageClass::PushConstant;

    #[test]
    fn test() {
        let bytes = include_bytes!("../tests/pos_norm_col.spirv");
        let words =
            unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4) };

        let res = parse_spirv(words).unwrap();

        let s = to_string(&res).unwrap();

        let mut file = File::create("./tests/pos_norm_col.json").unwrap();
        file.write(s.as_bytes());
    }

    #[test]
    fn test_entries() {
        let bytes = include_bytes!("../tests/pos_norm_col.spirv");
        let words =
            unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4) };

        let res = parse_spirv(words).unwrap();
        let main = res.main_entry_point();
        assert_eq!(&main.name, "main");
        assert_eq!(main.execution_model, ExecutionModel::Vertex)
    }

    #[test]
    fn test_push_constant() {
        let bytes = include_bytes!("../tests/pos_norm_col.spirv");
        let words =
            unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4) };

        let res = parse_spirv(words).unwrap();
        let main = res.main_entry_point();
        let block = res.push_constant_blocks();

        let x = Some(
            PushConstantBlock{
                name: "".to_string(),
                ty: Type::Complex(ComplexType::Structure {name: "Model".to_owned(), members: vec![("model".to_owned(), Type::Complex(ComplexType::Matrix { ty: SimpleType::Float, cols: 4, rows: 4 }))] }),
                offset: 0,
                id: 0
            }
        );

        assert_eq!(block, x);
    }

    #[test]
    fn test_interfaces() {
        let bytes = include_bytes!("../tests/pos_norm_col.spirv");
        let words =
            unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4) };

        let vert = parse_spirv(words).unwrap();

        let bytes = include_bytes!("../tests/shaded.spirv");
        let words =
            unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4) };
        let frag = parse_spirv(words).unwrap();


        let outputs = vert.output_variables(&vert.main_entry_point());
        let inputs = frag.input_variables(&frag.main_entry_point());
        for input in &outputs {
            println!("{:?}", input);
        }

        for input in &inputs {
            println!("{:?}", input);
        }


        assert_eq!(inputs, outputs);
    }
}
