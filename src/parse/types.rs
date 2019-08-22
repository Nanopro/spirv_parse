use crate::raw::*;


pub struct EntryPoint {
    pub name: String,
    pub execution_model: ExecutionModel,
    pub(crate) interface: Vec<u32>,
}


#[derive(Debug)]
pub struct InterfaceVariable {
    pub  id: u32,
    pub  name: String,
    pub  location: u32,
    pub  storage_class: StorageClass,
    pub offset: u32,
    pub  ty: Type,
}


impl PartialEq for InterfaceVariable {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location && self.ty == other.ty
    }
}


#[derive(Debug, PartialEq)]
pub enum SimpleType {
    Boolean,
    Integer,
    UInteger,
    Float,
    Numerical,
    Scalar,
    Sampler
}

#[derive(Debug, PartialEq)]
pub enum ComplexType {
    Vector {
        ty: SimpleType,
        len: u32,
    },
    Matrix {
        ty: SimpleType,
        cols: u32,
        rows: u32,
    },
    Array {
        ty: Box<Type>,
        len: u32,
    },
    Structure {
        name: String,
        block: BlockType,
        members: Vec<(String, Type)>,
    },
    SampledImage {
        image: Box<Type>
    },
    Image {
        dim: Dim,
        depth: Option<bool>,
        arrayed: bool,
        multisampled: bool,
        sampled: Option<bool>,
        format: ImageFormat,

    },
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BlockType{
    Block,
    BufferBlock,
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Simple(SimpleType),
    Complex(ComplexType),
}

#[derive(Debug)]
pub enum ConstValue {
    Boolean(bool),
    Integer(i64),
    UInteger(u32),
    Float(f32),
    Numerical(u64),
    Scalar(u64),
}

#[cfg(feature = "vk-format")]
extern crate ash;

#[cfg(feature = "vk-format")]
use ash::vk::Format;
use std::collections::HashMap;

#[cfg(feature = "vk-format")]
impl Type {
    pub fn to_format(&self) -> ash::vk::Format {
        match self {
            Type::Simple(ty) => {
                match ty {
                    SimpleType::Boolean => Format::R8_UINT,
                    SimpleType::UInteger => Format::R32_UINT,
                    SimpleType::Float => Format::R32_SFLOAT,
                    SimpleType::Numerical => unimplemented!(),
                    SimpleType::Scalar => unimplemented!(),
                    _ => Format::UNDEFINED
                }
            }
            Type::Complex(ty) => {
                match ty {
                    ComplexType::Vector { ty, len, .. } => {
                        match ty {
                            SimpleType::Float => {
                                match len {
                                    1 => Format::R32_SFLOAT,
                                    2 => Format::R32G32_SFLOAT,
                                    3 => Format::R32G32B32_SFLOAT,
                                    4 => Format::R32G32B32A32_SFLOAT,
                                    _ => panic!("Vector's len > 5")
                                }
                            }
                            _ => {
                                unimplemented!()
                            }
                        }
                    }
                    ComplexType::Array { ty, len, .. } => {
                        unimplemented!()
                    }
                    ComplexType::Matrix { .. } => {
                        unimplemented!()
                    }
                    ComplexType::Structure { name, members,.. } => {
                        Format::UNDEFINED
                    }
                    _ => unimplemented!(),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct PushConstantBlock {
    pub name: String,
    pub ty: Type,
    pub offset: u32,
    pub id: u32,
}

impl PartialEq for PushConstantBlock {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.ty == other.ty && self.offset == other.offset
    }
}

#[derive(Debug)]
pub struct DescriptorSet {
    pub set: u32,
    pub bindings: Vec<(u32, DescriptorBindning)>,
}

#[derive(Debug)]
pub struct DescriptorBindning {
    pub binding: u32,
    pub set: u32,
    pub data_type: Type,
    pub ty: DescriptorType,
    pub count: u32,

}
#[derive(Debug)]
pub enum DescriptorType {
    Undefined,
    Sampler,
    CombinedImageSampler,
    SampledImage,
    StorageImage,
    UniformTexelBuffer,
    StorageTexelBuffer,
    UniformBuffer,
    StorageBuffer,
    UniformBufferDynamic,
    StorageBufferDynamic,
    InputAttachment(u32),
    AccelerationStructureNV,
}
