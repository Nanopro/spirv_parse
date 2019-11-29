mod generated;

pub use generated::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let data = vec![32];
        let x = IdScope::from_raw(&data);
        assert_eq!(x.0, IdScope(32));
        assert_eq!(x.1, &[] as &[u32]);
    }

    #[test]
    fn test_lit_int() {
        let data = vec![32];
        let x = LiteralInteger::from_raw(&data);
        assert_eq!(x.0, LiteralInteger(32));
        assert_eq!(x.1, &[] as &[u32]);
    }

    #[test]
    fn test_lit_str() {
        let string = "Hello".to_owned();
        let bytes = string.as_bytes();
        let mut data = vec![];
        data.extend(bytes);
        data.push(0);
        data.push(0);
        data.push(0);
        println!("{:?}", data);
        let data =
            unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u32, data.len() / 4) };
        println!("{:?}", data);

        let x = LiteralString::from_raw(&data);
        assert_eq!(x.0, LiteralString(string));
        assert_eq!(x.1, &[] as &[u32]);
    }

    #[test]
    fn test_composite() {
        let data = vec![32, 36];
        let x = PairIdRefIdRef::from_raw(&data);
        assert_eq!(x.0, PairIdRefIdRef(IdRef(32), IdRef(36)));
        assert_eq!(x.1, &[] as &[u32]);
    }

    #[test]
    fn test_enum() {
        let data = vec![2];
        let x = MemoryModel::from_raw(&data);
        assert_eq!(x.0, MemoryModel::OpenCL);
        assert_eq!(x.1, &[] as &[u32]);
    }

    #[test]
    fn test_enum_with_par() {
        let data = vec![6, 2];
        let x = Decoration::from_raw(&data);
        assert_eq!(x.0, Decoration::ArrayStride(LiteralInteger(2)));
        assert_eq!(x.1, &[] as &[u32]);
    }

    #[test]
    fn test_bit() {
        let data = vec![0b11];
        let x = ImageOperands::from_raw(&data);
        assert_eq!(x.0, ImageOperands(0b11));
        assert_eq!(x.1, &[] as &[u32]);
    }

    #[test]
    fn test_op_1() {
        let data = vec![1, 2];
        let x = Instruction::from_raw(1, &data);
        assert_eq!(x, Instruction::Undef(IdResultType(1), IdResult(2)));
    }
    #[test]
    fn test_op_name() {
        let data = vec![1, 2];
        let x = Instruction::from_raw(1, &data);
        assert_eq!(x, Instruction::Undef(IdResultType(1), IdResult(2)));
    }

    #[test]
    fn test_instruction_with_parametred_bit_enum() {
        let data = vec![1, 2, 3, 4, 0b1];
        let x = Instruction::from_raw(88, &data);
        assert_eq!(
            x,
            Instruction::ImageSampleExplicitLod(
                IdResultType(1),
                IdResult(2),
                IdRef(3),
                IdRef(4),
                ImageOperands(0b1)
            )
        );
    }
}
