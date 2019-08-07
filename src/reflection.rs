use crate::raw::{Instruction, Decoration};
use crate::parse::{Spirv, parse_spirv};


pub type Result<T> = ::std::result::Result<T, ReflectionError>;

pub enum ReflectionError{

}

#[derive(Debug)]
pub struct Reflection{
    inner: Spirv,
}


impl Reflection {
    pub fn new(spirv: Spirv) -> Result<Self>{
        Ok(
            Self{
                inner: spirv
            }
        )
    }
    pub fn get_decoration(&self, id: u32, decoration: Decoration) -> Option<u32>{
        for instr in &self.inner.instructions{
            match instr{

                _ => ()
            }
        }
        return None;
    }
}


#[cfg(test)]
mod tests{
    use crate::raw::{Instruction, Decoration};
    use crate::parse::{Spirv, parse_spirv};
    use super::*;

    #[test]
    fn test_get_dec(){
        let bytes = include_bytes!("../tests/pos_norm_col.spirv");
        let words = unsafe{
            std::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len()/4)
        };

        let res = parse_spirv(words).unwrap();

    }
}