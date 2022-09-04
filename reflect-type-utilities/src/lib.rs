pub use std::collections::HashMap;
pub trait MyTrait {
    fn show(v:&HashMap<String,ObjectType>)->Self;
}

pub enum ObjectType{
    String(String),
    INT(i32),
    Float(f64)
}


impl Into<i32> for &ObjectType{
    fn into(self) -> i32 {
        match self{
            ObjectType::String(s) => todo!(),
            ObjectType::INT(s) => {
                //println!("evaluation");
                *s
            },
            ObjectType::Float(s) => todo!(),
        }
    }
}

impl Into<f64> for &ObjectType{
    fn into(self) -> f64 {
        match self{
            ObjectType::String(s) => todo!(),
            ObjectType::INT(s) => todo!(),
            ObjectType::Float(s) => *s,
        }
    }
}

impl Into<String> for &ObjectType{
    fn into(self) -> String {
        match self{
            ObjectType::String(s) => s.clone(),
            ObjectType::INT(s) => todo!(),
            ObjectType::Float(s) => todo!()
        }
    }
}
