#![recursion_limit = "1024"]
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate proc_macro;

use proc_macro2::Span;
use serde_json::{from_reader, Value};
use std::collections::HashSet;
use std::fs::File;
use std::io::{Read, Write};
use syn::parse::{Parse, ParseStream, Result};
use syn::{Attribute, Ident, LitBool, LitInt, LitStr};

pub struct MacroInput {
    pub path: String,
    pub output: String,
}

fn op_kinds(operand_kinds: &Vec<Value>) -> proc_macro2::TokenStream {
    let mut structs = vec![];

    for op_kind in operand_kinds {
        let category = op_kind.get("category").unwrap().as_str().unwrap();
        let struct_str = op_kind.get("kind").unwrap().as_str().unwrap();
        let struct_name = Ident::new(struct_str, Span::call_site());
        let parameters_enum_name = Ident::new(&format!("{}Parameters", struct_name), Span::call_site());
        let struct_name_enums = Ident::new(&format!("{}Bits", struct_name), Span::call_site());

        if category == "BitEnum" {
            let mut values = vec![];
            let mut parse_parameters = vec![];
            let mut from_name = vec![];
            let mut parameter_enums = vec![];

            let mut max_value = 0x0;

            for enumerant in op_kind.get("enumerants").unwrap().as_array().unwrap() {
                let name = enumerant.get("enumerant").unwrap().as_str().unwrap();
                let ident = match syn::parse_str::<Ident>(name) {
                    Ok(ident) => ident,
                    Err(_) => {
                        let name_prepend = format!("_{}", name);
                        Ident::new(&name_prepend, Span::call_site())
                    }
                };

                let value =
                    syn::parse_str::<LitInt>(enumerant.get("value").unwrap().as_str().unwrap())
                        .unwrap()
                        .value() as u32;

                let parameters = match enumerant.get("parameters"){
                    Some(pars) => {
                        pars.as_array().unwrap()
                            .iter()
                            .map(|parameter|{
                                let kind: Ident = parameter.get("kind").and_then(|p| p.as_str()).map(|p| syn::parse_str::<Ident>(p).unwrap()).unwrap();
                                kind
                            })
                            .collect::<Vec<_>>()
                    },
                    None => {
                        vec![]
                    }
                };
                let parameters2 = parameters.clone();
                parse_parameters.push(
                    quote!{
                        if bits.contains(#struct_name_enums::#ident){
                            parameters.push(
                                #parameters_enum_name::#ident(
                                    #({
                                        let (v, d) = #parameters2::from_raw(data);
                                        data = d;
                                        v
                                    }, )*
                                )
                            )
                        }
                    }
                );

                parameter_enums.push(
                    quote!{
                        #ident( #( #parameters, )* ),
                    }
                );

                if value > max_value {
                    max_value = value;
                }

                values.push(quote!(
                    const #ident = #value;
                ));
                from_name.push(quote!(
                    #name => Some(#struct_name_enums::#ident)
                ));
            }

            structs.push(quote!(

                #[derive(Debug, Clone, Serialize, Deserialize, Derivative)]
                #[derivative(PartialEq, Default(bound=""))]
                pub struct #struct_name{
                    pub value: #struct_name_enums,
                    #[derivative(PartialEq="ignore")]
                    pub parameters: Vec<#parameters_enum_name>
                }

               #[derive(Debug, Clone, Serialize, Deserialize)]
                pub enum #parameters_enum_name{
                    #( #parameter_enums )*
                }

                bitflags!{
                    #[derive(Default, Serialize, Deserialize)]
                    pub struct #struct_name_enums: u32{
                        #( #values )*
                    }
                }

                impl #struct_name{
                     pub fn new(value: u32) -> Self{
                        Self{
                            value: #struct_name_enums::from_bits(value).unwrap(),
                            parameters: vec![],
                        }
                     }
                }

                impl #struct_name {
                    #[inline]
                    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
                        //assert!(data.len() > 0);
                        let bits = #struct_name_enums::from_bits(data[0]).unwrap();
                        data = &data[1..];
                        let mut parameters = vec![];
                        #( #parse_parameters )*
                        (
                            Self{
                                value: bits,
                                parameters,
                            },
                            data
                        )
                    }
                }
            ));
        }
        if category == "ValueEnum" {
            let mut values = vec![];

            let mut from_value = vec![];
            let mut occupied = HashSet::new();
            for enumerant in op_kind.get("enumerants").unwrap().as_array().unwrap() {
                let name = enumerant.get("enumerant").unwrap().as_str().unwrap();
                let ident = match syn::parse_str::<Ident>(name) {
                    Ok(ident) => ident,
                    Err(_) => {
                        let name_prepend = format!("_{}", name);
                        Ident::new(&name_prepend, Span::call_site())
                    }
                };
                let value = enumerant.get("value").unwrap().as_u64().unwrap() as u32;
                let mut fields_constr = vec![];
                let mut fields = vec![];
                match enumerant.get("parameters") {
                    Some(v) => {
                        let parameters = v.as_array().unwrap();
                        for par in parameters {
                            let kind = Ident::new(
                                par.get("kind").unwrap().as_str().unwrap(),
                                Span::call_site(),
                            );
                            fields.push(quote!(
                                #kind
                            ));

                            fields_constr.push(quote!(
                                {
                                    let (v, d) = #kind::from_raw(data);
                                    data = d;
                                    v
                                }
                            ));
                        }
                    }
                    None => {}
                }

                if occupied.insert(value) {
                    if fields.len() > 0 {
                        values.push(quote!(
                            #ident(
                                #( #fields, )*
                            )
                        ));

                        from_value.push(quote!(
                            #value => {
                                let s = #struct_name::#ident(
                                    #( #fields_constr, )*
                                );
                                (s, data)
                            }
                        ))
                    } else {
                        values.push(quote!(
                                #ident
                        ));

                        from_value.push(quote!(
                            #value => {
                                (#struct_name::#ident, data)
                            }
                        ))
                    }
                }
            }

            structs.push(quote!(
                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
                #[repr(u32)]
                pub enum #struct_name{
                    #( #values, )*
                }

                impl #struct_name{
                    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]){
                        let value = data[0];
                        data = &data[1..];
                        match value{
                            #( #from_value, )*
                            _ => panic!("Unknown value for Enum: {}", #struct_str),
                        }
                    }
                }
            ));
        }
        if category == "Id" {
            let c = match struct_str{
                "IdResult" => "%",
                "IdResultType" => "@",
                "IdRef" => "&",
                _ => ""
            };


            structs.push(quote!(
                 #[derive(PartialEq, Clone, Serialize, Deserialize)]
                pub struct #struct_name(pub u32);
                impl #struct_name{
                    pub fn from_raw(data: &[u32]) -> (Self, &[u32]){
                        //assert!(data.len() > 0);
                        (Self(data[0]), &data[1..])
                    }
                }
                impl std::fmt::Debug for #struct_name{
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
                        use std::fmt::{self, Alignment};
                        let width = f.width().unwrap_or(0);
                        if let Some(s) = f.align() {
                            match s {
                                Alignment::Left    =>  write!(f, "{:<width$?}", format_args!("{}{}", #c, self.0), width=width),
                                Alignment::Right   =>  write!(f, "{:>width$?}", format_args!("{}{}", #c, self.0), width=width),
                                Alignment::Center  =>  write!(f, "{:^width$?}", format_args!("{}{}", #c, self.0), width=width),
                            }
                        } else {
                             write!(f, "{:width$?}", format_args!("{}{}", #c, self.0), width=width)
                        }

                    }
                }
            ));
        }
        if category == "Literal" {
            let t = match op_kind.get("kind").unwrap().as_str().unwrap() {
                "LiteralInteger" => quote!(pub u32),
                "LiteralString" => quote!(pub String),
                "LiteralContextDependentNumber" => quote!(pub Vec<u32>),
                "LiteralExtInstInteger" => quote!(pub u32),
                "LiteralSpecConstantOpInteger" => quote!(pub Vec<u32>),
                _ => panic!("Unknow Literal kind, please update spirv grammar json"),
            };

            let constr = match op_kind.get("kind").unwrap().as_str().unwrap() {
                "LiteralInteger" => {
                    quote!(
                        pub fn from_raw(data: &[u32]) -> (Self, &[u32]) {
                            //assert!(data.len() > 0);
                            (Self(data[0]), &data[1..])
                        }
                    )
                }
                "LiteralString" => quote!(
                    pub fn from_raw(data: &[u32]) -> (Self, &[u32]) {
                        let res = parse_string(data);
                        (Self(res.0), res.1)
                    }
                ),
                "LiteralContextDependentNumber" => quote!(
                    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
                        let mut v = vec![];
                        while (data.len() > 0) {
                            v.push(data[0]);
                            data = &data[1..];
                        }
                        (Self(v), data)
                    }
                ),
                "LiteralExtInstInteger" => {
                    quote!(
                        pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
                            //assert!(data.len() > 0);
                            (Self(data[0]), &data[1..])
                        }
                    )
                }
                "LiteralSpecConstantOpInteger" => quote!(
                    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
                        let mut v = vec![];
                        while (data.len() > 0) {
                            v.push(data[0]);
                            data = &data[1..];
                        }
                        (Self(v), data)
                    }
                ),
                _ => panic!("Unknow Literal kind, please update spirv grammar json"),
            };

            structs.push(quote!(
                #[derive(PartialEq, Clone, Serialize, Deserialize)]
                pub struct #struct_name (#t);
                impl #struct_name {
                    #constr
                }
                 impl std::fmt::Debug for #struct_name{
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        use std::fmt::{self, Alignment};
                        let width = f.width().unwrap_or(0);
                        if let Some(s) = f.align() {
                            match s {
                                Alignment::Left    =>  write!(f, "{:<1$?}", self.0, width),
                                Alignment::Right   =>  write!(f, "{:>1$?}", self.0, width),
                                Alignment::Center  =>  write!(f, "{:^1$?}", self.0, width),
                            }
                        } else {
                             write!(f, "{:width$?}", self.0, width=width)
                        }

                    }
                 }
            ));
        }
        if category == "Composite" {
            let bases = op_kind
                .get("bases")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .map(|v| Ident::new(v.as_str().unwrap(), Span::call_site()))
                .collect::<Vec<_>>();

            let fields_constr = bases
                .iter()
                .map(|ident| {
                    quote!(
                        {
                            let (v, d) = #ident::from_raw(data);
                            data = d;
                            v
                        }
                    )
                })
                .collect::<Vec<_>>();

            structs.push(quote!(
                #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
                pub struct #struct_name( #( pub #bases, )* );
                impl #struct_name{
                    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]){
                        (
                            Self(
                                #( #fields_constr, )*
                            ),
                            data
                        )
                    }
                }
            ));
        }
    }

    quote!(
         #( #structs )*
    )
}

fn instructions(instrs: &[Value]) -> proc_macro2::TokenStream {
    let mut constr = vec![];
    let mut members = vec![];
    let mut formaters = vec![];

    for instr in instrs {
        let op_str = &instr.get("opname").unwrap().as_str().unwrap()[2..];
        let op_name = Ident::new(op_str, Span::call_site());
        let op_code = instr.get("opcode").unwrap().as_u64().unwrap() as u16;
        let mut fields = vec![];
        let mut fields_constr = vec![];
        let mut fields_formatters = vec![];
        match instr.get("operands") {
            Some(operands) => {
                for (num, operand) in operands.as_array().unwrap().iter().enumerate() {
                    let kind = operand.get("kind").unwrap().as_str().unwrap();
                    let name = match operand.get("name") {
                        Some(v) => v.as_str().unwrap(),
                        None => match kind {
                            "IdResult" => "id",
                            "IdRef" => "id_ref",
                            _ => "unknown",
                        },
                    };
                    let quantifier = operand.get("quantifier");
                    let ident = Ident::new(kind, Span::call_site());
                    match quantifier {
                        Some(v) => match v.as_str() {
                            Some("?") => {
                                fields.push(quote!(
                                   Option<#ident>
                                ));
                                fields_constr.push(quote!(
                                    {
                                        if data.len() > 0{
                                            let (v, d) = #ident::from_raw(data);
                                            data = d;
                                            Some(v)
                                        } else{
                                            None
                                        }

                                    }
                                ));
                            }
                            Some("*") => {
                                fields.push(quote!(
                                   Vec<#ident>
                                ));
                                fields_constr.push(quote!(
                                    {
                                        let mut v = vec![];
                                        while data.len() > 0{
                                            let (s, d) = #ident::from_raw(data);
                                            data = d;
                                            v.push(s);
                                        }

                                        v
                                    }
                                ));
                            }
                            _ => unreachable!(),
                        },
                        None => {
                            fields.push(quote!(
                               #ident
                            ));
                            fields_constr.push(quote!(
                                {
                                    let (v, d) = #ident::from_raw(data);
                                    data = d;
                                    v
                                }
                            ));
                        }
                    }
                    match kind{
                        "IdResult" => {
                            fields_formatters.push(
                                (-1, num)
                            )
                        },
                        _ => {
                            fields_formatters.push(
                                (num as isize, num)
                            )
                        }
                    }

                }
                
                fields_formatters.sort_by_key(|(prior, _)| *prior);
                let has_result = fields_formatters[0].0 == -1;
                let result_key = fields_formatters[0].1;
                let fmt_string = if has_result {
                    let rest = fields_formatters.iter().skip(1).map(|(_, num)| format!("{{{}:?}}", num)).collect::<Vec<_>>().join(" ");
                    format!("{{{0}:?}} = Op{1} {2}", result_key, op_name, rest)
                }else{
                    let rest = fields_formatters.iter().map(|(_, num)| format!("{{{}:?}}", num)).collect::<Vec<_>>().join(" ");
                    format!("Op{0} {1}",op_name, rest)
                };
                let fields = (0..fields.len()).map(|i| {
                    let ident = Ident::new(&format!("x_{}", i), Span::call_site());
                    quote!( #ident )
                    }).collect::<Vec<_>>();
                let fields_tupple = fields.clone();
                formaters.push(
                    quote!{
                        Instruction::#op_name( #( #fields_tupple, )* ) => {write!(f, #fmt_string, #( #fields, )* )?;}
                    }
                );
            }
            None => {
                let fmt_string = format!("Op{}", op_name);
                formaters.push(
                    quote!(
                        Instruction::#op_name => {write!(f, #fmt_string)?;}
                    )
                );
            },
        }

        if fields.len() > 0 {
            members.push(quote!(
                #op_name (#( #fields, )*)
            ));

            constr.push(quote!(
                #op_code =>{
                    //println!("{}, {:?}", #op_str, data);
                    let instr = Instruction::#op_name(
                        #( #fields_constr, )*
                    );
                    assert_eq!(data.len(), 0);
                    instr
                }
            ));
        } else {
            members.push(quote!(
                #op_name
            ));
            constr.push(quote!(
                #op_code => Instruction::#op_name
            ));
        }
    }

    quote! {
        fn parse_string(data: &[u32]) -> (String, &[u32]) {
            let bytes = data.iter()
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

            (s, &data[r ..])
        }

        #[derive(Debug, Clone, PartialEq, Serialize)]
        pub enum Instruction{
             #( #members, )*
            None(u16),
        }
        impl Instruction{
            pub fn from_raw(op_code: u16, mut data: &[u32]) -> Self{
                match op_code {
                     #( #constr, )*
                    _ => Instruction::None(op_code)
                }
            }
        }
        impl std::fmt::Display for Instruction{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
                match self{
                    #( #formaters, )*
                    Instruction::None(op_code) => {write!(f, "OpNone opcode({})", op_code)?;},
                }
                Ok(())
            }
        }
    }
}

//#[proc_macro]
pub fn grammar(input: MacroInput) {
    let file = File::open(input.path).unwrap();
    let data: Value = from_reader(file).unwrap();
    let instr = &data.get("instructions").unwrap().as_array().unwrap();
    let operand_kinds = data.get("operand_kinds").unwrap().as_array().unwrap();

    let magic: LitInt =
        syn::parse_str(data.get("magic_number").unwrap().as_str().unwrap()).unwrap();
    let major = data.get("major_version").unwrap().as_u64().unwrap() as u8;
    let minor = data.get("minor_version").unwrap().as_u64().unwrap() as u8;

    // let res = vec![];
    let enums = op_kinds(operand_kinds);
    let instr = instructions(instr);

    let res = quote!(
        #![allow(non_upper_case_globals)]
        #![allow(non_camel_case_types)]
        #![allow(non_snake_case)]
        #![allow(unused)]
        use serde::{Deserialize, Serialize};
        pub const MAGIC: u64 = #magic;
        pub const VERSION: (u8, u8) = (#major, #minor);
        #enums
        #instr
    );
    let mut file = File::create(input.output).unwrap();
    write!(&mut file, "{}", res).expect("WTF");
}
