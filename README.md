# rust-reflector

````rust
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
