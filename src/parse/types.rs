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

#[cfg(feature = "vk-format")]
impl EntryPoint {
    pub fn shader_flags(&self) -> ash::vk::ShaderStageFlags{
        use ash::vk::ShaderStageFlags as SSF;
        use ExecutionModel::*;
        match &self.execution_model{
            Vertex => SSF::VERTEX,
            TessellationControl => SSF::TESSELLATION_CONTROL,
            TessellationEvaluation => SSF::TESSELLATION_EVALUATION,
            Geometry => SSF::GEOMETRY,
            Fragment => SSF::FRAGMENT,
            GLCompute => SSF::COMPUTE,
            Kernel => SSF::COMPUTE,
            TaskNV => SSF::TASK_NV,
            MeshNV => SSF::MESH_NV,
            RayGenerationNV => SSF::RAYGEN_NV,
            IntersectionNV => SSF::INTERSECTION_NV,
            AnyHitNV => SSF::ANY_HIT_NV,
            ClosestHitNV => SSF::CLOSEST_HIT_NV,
            MissNV => SSF::MISS_NV,
            CallableNV => SSF::CALLABLE_NV,
        }
    }
}

impl SimpleType{
    pub fn size(&self) -> Option<u64>{
        match self{
            SimpleType::Sampler => None,
            SimpleType::Integer | SimpleType::UInteger | SimpleType::Float | SimpleType::Numerical | SimpleType::Scalar=> Some(4),
            _ => unimplemented!() //TODO!
        }
    }
}
impl ComplexType{
    pub fn size(&self) -> Option<u64>{
        match self{
            ComplexType::Vector { len, ty } => { Some(*len as u64 * ty.size()?)},
            ComplexType::Matrix { ty, cols, rows } => { Some(*cols as u64 * *rows as u64 * ty.size()?)},
            ComplexType::Array { ty, len } => {Some(*len as u64 * ty.size()? )},
            ComplexType::Structure { members, .. } => {  //TODO! not true for complex types with some aligment
                let sizes = members.iter().map(|(_, ty)| ty.size()).collect::<Option<Vec<_>>>();

                Some(sizes?.into_iter().sum())

            },
            ComplexType::SampledImage { .. } => { None },
            ComplexType::Image { .. } => { None },
        }
    }
}
impl Type{
    pub fn size(&self) -> Option<u64>{
        match self{
            Type::Simple(ty) => {ty.size()},
            Type::Complex(ty) => { ty.size()},
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
#[cfg(feature = "vk-format")]
impl DescriptorType {
    pub fn to_vulkan(&self) -> ash::vk::DescriptorType {
        use ash::vk::DescriptorType as DT;

        match self {
            DescriptorType::Undefined => DT::from_raw(-1), // TODO? HOW to fix it
            DescriptorType::Sampler => DT::SAMPLER,
            DescriptorType::CombinedImageSampler => DT::COMBINED_IMAGE_SAMPLER,
            DescriptorType::SampledImage => DT::SAMPLED_IMAGE,
            DescriptorType::StorageImage => DT::STORAGE_IMAGE,
            DescriptorType::UniformTexelBuffer => DT::UNIFORM_TEXEL_BUFFER,
            DescriptorType::StorageTexelBuffer => DT::STORAGE_TEXEL_BUFFER,
            DescriptorType::UniformBuffer => DT::UNIFORM_BUFFER,
            DescriptorType::StorageBuffer => DT::STORAGE_BUFFER,
            DescriptorType::UniformBufferDynamic => DT::UNIFORM_BUFFER_DYNAMIC, //TODO?
            DescriptorType::StorageBufferDynamic => DT::STORAGE_BUFFER_DYNAMIC, //TODO?
            DescriptorType::InputAttachment(_) => DT::INPUT_ATTACHMENT,
            DescriptorType::AccelerationStructureNV => DT::ACCELERATION_STRUCTURE_NV, //TODO?

        }
    }
}