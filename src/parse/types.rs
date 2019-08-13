use crate::raw::*;


pub struct EntryPoint{
    pub name: String,
    pub execution_model: ExecutionModel,
    pub(crate) interface: Vec<u32>
}




pub enum InterfaceVariable{
    Simple{
        id: u32,
        name: String,
        location: u32,
        storage_class: StorageClass,
        offset: u32,

    },
    Struct{
        members: Vec<u32>,

    }
}

#[allow(non_camel_case_types)]
pub enum Format{
    Undefined,
    R32_UINT,
    R32_SINT,
    R32_SFLOAT,
    R32G32_UINT,
    R32G32_SINT,
    R32G32_SFLOAT,
    R32G32B32_UINT,
    R32G32B32_SINT,
    R32G32B32_SFLOAT,
    R32G32B32A32_UINT,
    R32G32B32A32_SINT,
    R32G32B32A32_SFLOAT,
}


