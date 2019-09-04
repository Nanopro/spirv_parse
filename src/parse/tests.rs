use super::*;
use crate::raw::StorageClass::PushConstant;
use std::fs::File;
use std::io::Write;

#[test]
fn test() {
    let bytes = include_bytes!("../../test_shaders/compiled/gaussblur.spirv");
    let words =
        unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4) };

    let res = parse_spirv(words).unwrap();

    let s = to_string(&res).unwrap();

    let mut file = File::create("./test_shaders/compiled/gaussblur.json").unwrap();
    file.write(s.as_bytes());
}

#[test]
fn test_entries() {
    let bytes = include_bytes!("../../tests/pos_norm_col.spirv");
    let words =
        unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4) };

    let res = parse_spirv(words).unwrap();
    let main = res.main_entry_point();
    assert_eq!(&main.name, "main");
    assert_eq!(main.execution_model, ExecutionModel::Vertex)
}

#[test]
fn test_push_constant() {
    let bytes = include_bytes!("../../tests/pos_norm_col.spirv");
    let words =
        unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4) };

    let res = parse_spirv(words).unwrap();
    let main = res.main_entry_point();
    let block = res.push_constant_blocks();

    let x = PushConstantBlock {
        name: "".to_string(),
        ty: Type::Complex(ComplexType::Structure {
            name: "Model".to_owned(),
            block: BlockType::Block,
            members: vec![(
                "model".to_owned(),
                Type::Complex(ComplexType::Matrix {
                    ty: SimpleType::Float,
                    cols: 4,
                    rows: 4,
                }),
            )],
        }),
        offset: 0,
        id: 0,
    };
    assert!(block.is_some());
    let block = block.unwrap();
    assert_eq!(block, x);
    assert_eq!(block.ty.size(), Some(4 * 4 * 4));
}

#[test]
fn test_interfaces() {
    let bytes = include_bytes!("../../tests/pos_norm_col.spirv");
    let words =
        unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4) };

    let vert = parse_spirv(words).unwrap();

    let bytes = include_bytes!("../../tests/shaded.spirv");
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

#[test]
fn test_descriptors() {
    let bytes = include_bytes!("../../test_shaders/compiled/gaussblur.spirv");
    let words =
        unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4) };

    let res = parse_spirv(words).unwrap();

    let sets = res.descriptor_sets();
    println!("{:#?}", sets);

    assert_eq!(1, 2)
}