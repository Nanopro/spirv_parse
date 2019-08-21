use crate::raw::*;


pub struct EntryPoint{
    pub name: String,
    pub execution_model: ExecutionModel,
    pub(crate) interface: Vec<u32>
}



#[derive(Debug)]
pub struct InterfaceVariable{
    pub  id: u32,
    pub  name: String,
    pub  location: u32,
    pub  storage_class: StorageClass,
    pub offset: u32,
    pub  ty: Type,
}


impl PartialEq for InterfaceVariable{
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location && self.ty == other.ty
    }
}






#[derive(Debug, PartialEq)]
pub enum SimpleType{
    Boolean,
    Integer,
    UInteger,
    Float,
    Numerical,
    Scalar,
}
#[derive(Debug, PartialEq)]
pub enum ComplexType{
    Vector {
        ty: SimpleType,
        len: u32
    },
    Matrix{
        ty: SimpleType,
        cols: u32,
        rows: u32,
    },
    Array{
        ty: Box<Type>,
        len: u32
    },
    Structure {
        name: String,
        members: Vec<(String, Type)>,
    }
}
#[derive(Debug, PartialEq)]
pub enum Type{
    Simple(SimpleType),
    Complex(ComplexType)
}
#[derive(Debug)]
pub enum ConstValue{
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
impl Type{
    pub fn to_format(&self) -> ash::vk::Format{
        match self{
            Type::Simple(ty) => {
                match ty{
                    SimpleType::Boolean => Format::R8_UINT,
                    SimpleType::UInteger => Format::R32_UINT,
                    SimpleType::Float => Format::R32_SFLOAT,
                    SimpleType::Numerical => unimplemented!(),
                    SimpleType::Scalar => unimplemented!(),
                    _ => Format::UNDEFINED
                }
            },
            Type::Complex(ty)=>{
                match ty{
                    ComplexType::Vector {ty, len, ..} => {
                        match ty {
                            SimpleType::Float => {
                                match len {
                                    1 => Format::R32_SFLOAT,
                                    2 => Format::R32G32_SFLOAT,
                                    3 => Format::R32G32B32_SFLOAT,
                                    4 => Format::R32G32B32A32_SFLOAT,
                                    _ => panic!("Vector's len > 5")
                                }
                            },
                            _ => {
                                unimplemented!()
                            }
                        }
                    },
                    ComplexType::Array {ty, len, ..} => {
                        unimplemented!()
                    },
                    ComplexType::Matrix{ .. } => {
                        unimplemented!()
                    },
                    ComplexType::Structure {name, members} => {
                        Format::UNDEFINED
                    }
                }
            }
        }
    }
}



