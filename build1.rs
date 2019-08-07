extern crate generator;
use std::fs::File;
use std::io::Write;


fn main() {
    let input = generator::MacroInput{
        path: "./SPIRV-Headers/include/spirv/unified1/spirv.core.grammar.json".to_owned(),
        output: "./src/raw/generated.rs".to_owned(),
    };

    generator::grammar(input);

}