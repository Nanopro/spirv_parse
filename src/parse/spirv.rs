use crate::parse::types::*;
use crate::raw::Capability::ShaderSMBuiltinsNV;
use serde_json::map::Entry;
use std::any::Any;
use std::ops::Deref;

use super::{parse_spirv, ParseError, FoundDecoration};
use crate::raw::*;
use serde::Serialize;
use serde_json::to_string;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct Spirv {
    pub version: (u8, u8),
    pub bound: u32,
    pub instructions: Vec<Instruction>,
    /// IdRef, Member, Decoration
    pub decorations: Vec<(u32, Option<u32>, Decoration)>,
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
                        // Скипаем билт-ины
                        if self.decorations.iter().find(|(deco_id, _, deco)| {
                            if *deco_id != id_result.0{
                                false
                            } else{
                                match deco{
                                    Decoration::BuiltIn(_) => true,
                                    _ => false
                                }
                            }
                        }).is_some(){
                            continue;
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
                    let name = self
                        .name_from_id(id_result.0)
                        .unwrap_or("UnnamedPushConstant".to_owned());
                    let offset = self
                        .decorations
                        .iter()
                        .find_map(|(id, member, decoration)| {
                            if *id == id_result.0 {
                                match decoration {
                                    Decoration::Offset(lit) => Some(lit.0),
                                    _ => None,
                                }
                            } else {
                                None
                            }
                        })
                        .unwrap_or(0);
                    return Some(PushConstantBlock {
                        name,
                        ty,
                        offset,
                        id: id_result.0,
                    });
                }
                _ => (),
            }
        }

        None
    }
    pub fn descriptor_sets(&self) -> Vec<DescriptorSet> {
        let mut bindings = vec![];
        for instruction in &self.instructions {
            match instruction {
                Instruction::Variable(id_res_type, id_res, class, opt)
                    if class == &StorageClass::Uniform
                        || class == &StorageClass::UniformConstant || class == &StorageClass::StorageBuffer =>
                {
                    let data_type = self.type_from_id(id_res_type.0).expect("Must have type");

                    let binding = self
                        .decorations
                        .iter()
                        .find_map(|(id, _, decoration)| {
                            if *id == id_res.0 {
                                match decoration {
                                    Decoration::Binding(lit) => Some(lit.0),
                                    _ => None,
                                }
                            } else {
                                None
                            }
                        })
                        .expect("Must have binding");
                    let set = self
                        .decorations
                        .iter()
                        .find_map(|(id, _, decoration)| {
                            if *id == id_res.0 {
                                match decoration {
                                    Decoration::DescriptorSet(lit) => Some(lit.0),
                                    _ => None,
                                }
                            } else {
                                None
                            }
                        })
                        .expect("Must have Set");
                    let input_attachment_index =
                        self.decorations.iter().find_map(|(i, _, deco)| {
                            if *i == id_res.0 {
                                match deco {
                                    Decoration::InputAttachmentIndex(index) => Some(index.0),
                                    _ => None,
                                }
                            } else {
                                None
                            }
                        });



                    let ty = data_type.descriptor_type(input_attachment_index);
                    let count = data_type.descriptor_count();
                    let name = self.name_from_id(id_res.0);

                    bindings.push(DescriptorBindning {
                        binding,
                        set,
                        ty,
                        name,
                        data_type,
                        count,
                    })
                }
                _ => (),
            }
        }

        let mut sets = HashMap::new();

        bindings.into_iter().for_each(|binding| {
            sets.entry(binding.set)
                .or_insert_with(|| DescriptorSet {
                    set: binding.set,
                    bindings: vec![],
                })
                .bindings
                .push((binding.binding, binding));
        });
        sets.into_iter()
            .map(|(_, mut v)| {
                v.bindings.sort_by_key(|v| v.0);
                v
            })
            .collect()
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
                    let ty = Box::new(self.type_from_id(element.0).unwrap());

                    let len = match self.const_value(len.0) {
                        Some(ConstValue::UInteger(len)) => ArrayLength::Number(len),
                        Some(ConstValue::SpecConst {spec_id, default, ..}) => ArrayLength::Constant{ spec_id, default},
                        _ => panic!("Array length must be unsigned integer"),
                    };



                    return Some(Type::Complex(ComplexType::Array {
                        ty,
                        len
                    }));
                }
                Instruction::TypePointer(id_res, class, point_type) if id_res.0 == id => {
                    return Some(Type::Complex(ComplexType::Pointer {
                        ty: Box::new(self.type_from_id(point_type.0)?),
                        storage: class.clone(),
                    }));
                }
                Instruction::TypeRuntimeArray(id_res, point_type) if id_res.0 == id => {
                    return Some(
                        Type::Complex(
                            ComplexType::Array{
                                ty: Box::new(self.type_from_id(point_type.0).unwrap_or_else(|| panic!("Unknown type for runtime's array point type: {}", point_type.0))),
                                len: ArrayLength::Dynamic,
                            }
                        )
                    )
                }
                Instruction::TypeStruct(id_res, members) if id_res.0 == id => {
                    return Some(Type::Complex(ComplexType::Structure {
                        name: self
                            .name_from_id(id)
                            .unwrap_or_else(|| "UnnamedStructure".to_owned()),
                        block: self
                            .decorations
                            .iter()
                            .find_map(|(i, _, decoration)| match (i, decoration) {
                                (&i, Decoration::Block) if i == id => Some(BlockType::Block),
                                (&i, Decoration::BufferBlock) if i == id => {
                                    Some(BlockType::BufferBlock)
                                }
                                _ => None,
                            })
                            .unwrap_or(BlockType::Block),
                           //.expect(&format!("Block decoration missing for struct {}", id_res.0)),
                        members: members
                            .iter()
                            .enumerate()
                            .map(|(n, member)| {
                                (
                                    self.member_name(id_res.0, n as u32)
                                        .unwrap_or_else(|| "__unnamed".to_owned()),
                                    self.type_from_id(member.0).unwrap_or_else(|| { panic!("Not found type for member: {}", member.0)}),
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
                        _ => unimplemented!(),
                    }
                }
                Instruction::TypeSampledImage(id_res, image) if id_res.0 == id => {
                    return Some(Type::Complex(ComplexType::SampledImage {
                        image: Box::new(self.type_from_id(image.0).unwrap()),
                    }));
                }
                Instruction::TypeSampler(id_res) if id_res.0 == id => {
                    return Some(Type::Simple(SimpleType::Sampler))
                }
                Instruction::TypeImage(
                    id_res,
                    sampled_type,
                    dim,
                    depth,
                    arrayed,
                    ms,
                    sampled,
                    format,
                    access,
                ) if id_res.0 == id => {
                    let depth = match depth.0 {
                        0 => Some(false),
                        1 => Some(true),
                        2 => None,
                        _ => unreachable!(),
                    };
                    let arrayed = arrayed.0 == 1;
                    let multisampled = ms.0 == 1;
                    let sampled = match sampled.0 {
                        0 => None,
                        1 => Some(true),
                        2 => Some(false),
                        _ => unreachable!(),
                    };

                    return Some(Type::Complex(ComplexType::Image {
                        dim: dim.clone(),
                        depth,
                        arrayed,
                        multisampled,
                        sampled,
                        format: format.clone(),
                    }));
                }
                Instruction::TypeAccelerationStructureKHR(id_res) if id_res.0 == id => {
                    return Some(Type::Simple(SimpleType::AccelerationStructureKHR))
                }
                Instruction::TypeAccelerationStructureNV(id_res) if id_res.0 == id => {
                    return Some(Type::Simple(SimpleType::AccelerationStructureNV))
                }
                Instruction::TypeRuntimeArray(id_res, id_type) if id_res.0 == id => {
                    let ty = Box::new(self.type_from_id(id_type.0).unwrap());
                    let len = ArrayLength::Dynamic;
                    return Some(Type::Complex(ComplexType::Array {
                        ty,
                        len
                    }));
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
                },
                Instruction::SpecConstant(id_res_type, id_res, default_value) if id_res.0 == id => {
                    let ty = self.type_from_id(id_res_type.0).unwrap();
                    let spec_id = self.decorations.iter().find_map(|(id, _,dec)| {
                        if *id != id_res.0{
                            None
                        }else{
                            match dec{
                                Decoration::SpecId(i) => Some(i.0),
                                _ => None
                            }
                        }
                    }).expect("SpecId expected for constant array length");
                    return Some(
                        ConstValue::SpecConst{ spec_id, ty, default: default_value.0[0] }
                    )
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

impl std::fmt::Display for Spirv{
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result{
        writeln!(f, "; Magic: {}", MAGIC)?;
        writeln!(f, "; Version: {:?}", self.version)?;
        writeln!(f, "; Bound: {}", self.bound)?;
        writeln!(f, "; Instructions")?;
        for i in &self.instructions{
            writeln!(f, "{:12}{}","", i)?;
        }
        Ok(())
    }
}
