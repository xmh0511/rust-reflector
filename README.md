# rust-reflector

````rust
use reflect_macro::MyTrait;

use reflect_type_utilities::{MyTrait,ObjectType,HashMap};

#[derive(MyTrait,Debug)]
struct Data6{
    num:i32,
    f:f64
}


#[derive(MyTrait,Debug)]
struct Person{
    num:i32,
    age:i32,
    name:String
}

fn main() {
    let mut map:HashMap<String,ObjectType>  = HashMap::new();
    map.insert("num".to_string(), ObjectType::INT(1024));
    map.insert("f".to_string(), ObjectType::Float(12.5));
 
    let s = Data6::show(&map);
    println!("{:?}",s);
 
 
 
    let mut map2:HashMap<String,ObjectType>  = HashMap::new();
    map2.insert("num".to_string(), ObjectType::INT(1024));
    map2.insert("age".to_string(), ObjectType::INT(20));
    map2.insert("name".to_string(), ObjectType::String("abc".to_string()));
 
    let s1 = Person::show(&map2);
    println!("{:?}",s1);
}
````
