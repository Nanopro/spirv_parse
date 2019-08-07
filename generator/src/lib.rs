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
use syn::{Ident, LitBool, LitInt, LitStr, Attribute};



pub struct MacroInput {
    pub path: String,
    pub output: String,
}



fn op_kinds(operand_kinds: &Vec<Value>) -> proc_macro2::TokenStream {
    let mut structs = vec![];

    for op_kind in operand_kinds {
        let category = op_kind.get("category").unwrap().as_str().unwrap();
        let struct_str = op_kind.get("kind").unwrap().as_str().unwrap();
        let struct_name = Ident::new(
            struct_str,
            Span::call_site(),
        );

        if category == "BitEnum" {
            let mut values = vec![];
            let mut from_name = vec![];
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
                        .unwrap().value() as u32;

                if value > max_value {
                    max_value = value;
                }

                values.push(quote!(
                    pub const #ident: Self = #struct_name(#value);
                ));
                from_name.push(quote!(
                    #name => Some(#struct_name::#ident)
                ));
            }

            structs.push(quote!(

                #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
                pub struct #struct_name(pub u32);



                impl #struct_name{
                     #( #values )*

                    pub fn from_name(name: &str) -> Option<Self>{
                        match name{
                             #( #from_name, )*
                             _ => None
                        }
                    }
                }
                impl Default for #struct_name {
                    fn default() -> #struct_name {
                        #struct_name(0)
                    }
                    }
                    impl #struct_name {
                        #[inline]
                        pub const fn empty() -> #struct_name {
                        #struct_name(0)
                        }
                        #[inline]
                        pub const fn all() -> #struct_name {
                            #struct_name(#max_value as u32)
                        }
                        #[inline]
                        pub fn from_raw(data: &[u32]) -> (Self, &[u32]) {
                            //assert!(data.len() > 0);
                            (#struct_name(data[0]), &data[1..])
                        }
                        #[inline]
                        pub const fn as_raw(self) -> u32 {
                            self.0
                        }
                        #[inline]
                        pub fn is_empty(self) -> bool {
                        self == #struct_name::empty()
                        }
                        #[inline]
                        pub fn is_all(self) -> bool {
                        self & #struct_name::all() == #struct_name::all()
                        }
                        #[inline]
                        pub fn intersects(self, other: #struct_name) -> bool {
                        self & other != #struct_name::empty()
                        }
                        #[doc = r" Returns whether `other` is a subset of `self`"]
                        #[inline]
                        pub fn contains(self, other: #struct_name) -> bool {
                        self & other == other
                        }
                    }
                    impl ::std::ops::BitOr for #struct_name {
                        type Output = #struct_name;
                        #[inline]
                        fn bitor(self, rhs: #struct_name) -> #struct_name {
                        #struct_name(self.0 | rhs.0)
                        }
                    }
                    impl ::std::ops::BitOrAssign for #struct_name {
                        #[inline]
                        fn bitor_assign(&mut self, rhs: #struct_name) {
                        *self = *self | rhs
                        }
                    }
                    impl ::std::ops::BitAnd for #struct_name {
                        type Output = #struct_name;
                        #[inline]
                        fn bitand(self, rhs: #struct_name) -> #struct_name {
                            #struct_name(self.0 & rhs.0)
                        }
                    }
                    impl ::std::ops::BitAndAssign for #struct_name {
                        #[inline]
                        fn bitand_assign(&mut self, rhs: #struct_name) {
                        *self = *self & rhs
                        }
                    }
                    impl ::std::ops::BitXor for #struct_name {
                        type Output = #struct_name;
                        #[inline]
                        fn bitxor(self, rhs: #struct_name) -> #struct_name {
                        #struct_name(self.0 ^ rhs.0)
                        }
                    }
                    impl ::std::ops::BitXorAssign for #struct_name {
                    #[inline]
                        fn bitxor_assign(&mut self, rhs: #struct_name) {
                        *self = *self ^ rhs
                        }
                    }
                    impl ::std::ops::Sub for #struct_name {
                        type Output = #struct_name;
                        #[inline]
                        fn sub(self, rhs: #struct_name) -> #struct_name {
                        self & !rhs
                        }
                    }
                    impl ::std::ops::SubAssign for #struct_name {
                        #[inline]
                        fn sub_assign(&mut self, rhs: #struct_name) {
                        *self = *self - rhs
                        }
                    }
                    impl ::std::ops::Not for #struct_name {
                        type Output = #struct_name;
                        #[inline]
                        fn not(self) -> #struct_name {
                        self ^ #struct_name::all()
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
                match enumerant.get("parameters"){
                    Some(v) => {
                        let parameters = v.as_array().unwrap();
                        for par in parameters{
                            let kind = Ident::new(par.get("kind").unwrap().as_str().unwrap(), Span::call_site());
                            fields.push(
                                quote!(
                                    #kind
                                )
                            );

                            fields_constr.push(
                                quote!(
                                    {
                                        let (v, d) = #kind::from_raw(data);
                                        data = d;
                                        v
                                    }
                                )
                            );

                        }
                    },
                    None => {

                    }
                }



                if occupied.insert(value) {
                    if fields.len() > 0{
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
                    }else{
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
            structs.push(
                quote!(
                     #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
                    pub struct #struct_name(pub u32);
                    impl #struct_name{
                        pub fn from_raw(data: &[u32]) -> (Self, &[u32]){
                            //assert!(data.len() > 0);
                            (Self(data[0]), &data[1..])
                        }
                    }
                )
            );
        }
        if category == "Literal" {
            let t = match op_kind.get("kind").unwrap().as_str().unwrap() {
                "LiteralInteger" => {
                    quote!(pub u32)
                }
                "LiteralString" => {
                    quote!(pub String)
                }
                "LiteralContextDependentNumber" => {
                    quote!(pub u32)
                }
                "LiteralExtInstInteger" => {
                    quote!(pub u32)
                }
                "LiteralSpecConstantOpInteger" => {
                    quote!(pub u32)
                }
                _ => panic!("Unknow Literal kind, please update spirv grammar json"),
            };

            let constr = match op_kind.get("kind").unwrap().as_str().unwrap() {
                "LiteralInteger" => {
                    quote!(
                        pub fn from_raw(data: &[u32]) -> (Self, &[u32]){
                            //assert!(data.len() > 0);
                            (Self(data[0]), &data[1..])
                        }
                    )
                }
                "LiteralString" => {
                    quote!(
                        pub fn from_raw(data: &[u32]) -> (Self, &[u32]){
                            let res = parse_string(data);
                            (Self(res.0), res.1)
                        }
                    )
                }
                "LiteralContextDependentNumber" => {
                    quote!(
                        pub fn from_raw(data: &[u32]) -> (Self, &[u32]){
                              //TODO! это ваще не правильно
                             //assert!(data.len() > 0);
                            (Self(data[0]), &data[1..])
                        }
                    )
                }
                "LiteralExtInstInteger" => {
                    quote!(
                        pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]){
                            //assert!(data.len() > 0);
                            (Self(data[0]), &data[1..])
                        }
                    )
                }
                "LiteralSpecConstantOpInteger" => {
                    quote!(
                        pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]){
                             //assert!(data.len() > 0);
                            (Self(data[0]), &data[1..])
                        }
                    )
                }
                _ => panic!("Unknow Literal kind, please update spirv grammar json"),
            };


            structs.push(
                quote!(
                    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
                    pub struct #struct_name (#t);
                    impl #struct_name {
                        #constr
                    }
                )
            );
        }
        if category == "Composite" {
            let bases = op_kind.get("bases").unwrap().as_array().unwrap().iter()
                .map(|v| {
                    Ident::new(v.as_str().unwrap(), Span::call_site())
                })
                .collect::<Vec<_>>();

            let fields_constr = bases.iter().map(|ident|
                quote!(
                            {
                                let (v, d) = #ident::from_raw(data);
                                data = d;
                                v
                            }
                        )
            ).collect::<Vec<_>>();

            structs.push(
                quote!(
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
                )
            );
        }
    }

    quote!(
         #( #structs )*
    )
}


fn instructions(instrs: &[Value]) -> proc_macro2::TokenStream {
    let mut constr = vec![];
    let mut members = vec![];

    for instr in instrs {
        let op_str = &instr.get("opname").unwrap().as_str().unwrap()[2..];
        let op_name = Ident::new(op_str, Span::call_site());
        let op_code = instr.get("opcode").unwrap().as_u64().unwrap() as u16;
        let mut fields = vec![];
        let mut fields_constr = vec![];
        match instr.get("operands") {
            Some(operands) => {
                for operand in operands.as_array().unwrap() {
                    let kind = operand.get("kind").unwrap().as_str().unwrap();
                    let name = match operand.get("name") {
                        Some(v) => {
                            v.as_str().unwrap()
                        }
                        None => {
                            match kind {
                                "IdResult" => "id",
                                "IdRef" => "id_ref",
                                _ => "unknown",
                            }
                        }
                    };
                    let quantifier = operand.get("quantifier");
                    let ident = Ident::new(kind, Span::call_site());
                    match quantifier {
                        Some(v) => {
                            match v.as_str() {
                                Some("?") => {
                                    fields.push(
                                        quote!(
                                           Option<#ident>
                                        )
                                    );
                                    fields_constr.push(
                                        quote!(
                                            {
                                                if data.len() > 0{
                                                    let (v, d) = #ident::from_raw(data);
                                                    data = d;
                                                    Some(v)
                                                } else{
                                                    None
                                                }

                                            }
                                        )
                                    );
                                }
                                Some("*") => {
                                    fields.push(
                                        quote!(
                                           Vec<#ident>
                                        )
                                    );
                                    fields_constr.push(
                                        quote!(
                                            {
                                                let mut v = vec![];
                                                while data.len() > 0{
                                                    let (s, d) = #ident::from_raw(data);
                                                    data = d;
                                                    v.push(s);
                                                }

                                                v
                                            }
                                        )
                                    );
                                }
                                _ => unreachable!()
                            }
                        }
                        None => {
                            fields.push(
                                quote!(
                                   #ident
                                )
                            );
                            fields_constr.push(
                                quote!(
                                    {
                                        let (v, d) = #ident::from_raw(data);
                                        data = d;
                                        v
                                    }
                                )
                            );
                        }
                    }
                }
            }
            None => (),
        }

        if fields.len() > 0 {
            members.push(
                quote!(
                    #op_name (#( #fields, )*)
                )
            );

            constr.push(
                quote!(
                    #op_code =>{
                        //println!("{}, {:?}", #op_str, data);
                        let instr = Instruction::#op_name(
                            #( #fields_constr, )*
                        );
                        assert_eq!(data.len(), 0);
                        instr
                    }
                )
            );
        } else {
            members.push(
                quote!(
                    #op_name
                )
            );
            constr.push(
                quote!(
                    #op_code => Instruction::#op_name
                )
            );
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
    }
}


//#[proc_macro]
pub fn grammar(input: MacroInput) {
    let file = File::open(input.path).unwrap();
    let data: Value = from_reader(file).unwrap();
    let instr = &data.get("instructions").unwrap().as_array().unwrap();
    let operand_kinds = data.get("operand_kinds").unwrap().as_array().unwrap();

    let magic: LitInt = syn::parse_str(data.get("magic_number").unwrap().as_str().unwrap()).unwrap();
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






















































