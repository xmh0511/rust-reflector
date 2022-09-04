use std::{str::FromStr};

use proc_macro::TokenStream;

static mut value:i32 = 0;

#[proc_macro]
pub fn macro_test(v:TokenStream)-> TokenStream{
    unsafe{value+=1;}
    TokenStream::from_str("\"abc\"").unwrap()
}


#[proc_macro_derive(MyTrait)]
pub fn reflect_type(v:TokenStream)-> TokenStream {
    let s = v.to_string();
    let brace_start = s.find("{").unwrap();
    let brace_end = s.rfind("}").unwrap();
    let name_start = s.find("struct").unwrap();
    let type_name = &s[name_start+6 ..brace_start];
    let fields = &s[brace_start+1..brace_end];
    let r = format!("\"{}\"",fields);
    let fields_str = fields.split(",");
    let mut types:Vec<String> = Vec::new();
    let mut identifiers:Vec<String> = Vec::new();
    for e in fields_str{
        let pair = e.split_once(":").unwrap();
        identifiers.push(pair.0.to_string());
        types.push(pair.1.to_string());
    }
    let types_join = format!("({})",types.join(",")) ;
    //let identifiers = format!("\"{}\"",identifiers.join(","));
    let len = types.len();
    let mut init_field:Vec<String> = Vec::new();
    for index in 0.. len{
       let d = format!("(&map[\"{}\"]).into()",identifiers[index].trim());
       let s = format!("{}:{}",identifiers[index].trim(),d);
       init_field.push(s);
    }

    let p = format!("{}",init_field.join(","));

    // let r = Self{{ {} }};
    // r

    let r1 = format!(r#"
        impl MyTrait for {} {{
            fn show(map: &HashMap<String,ObjectType>)->Self{{
                let r  = Self{{ {} }};
                r
            }}
        }}
    "#,type_name,p);
    TokenStream::from_str(&r1).unwrap()
}