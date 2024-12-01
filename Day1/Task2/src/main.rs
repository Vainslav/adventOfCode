use std::{fs::read_to_string, path::Path};
use std::collections::HashMap;

fn main() {
    let path = Path::new("input");
    let mut vec: Vec<i32> = Vec::with_capacity(1000);
    let mut map: HashMap<i32, i32> = HashMap::new();
    for line in read_to_string(&path).unwrap().lines(){
        let line_vec:Vec<&str> = line.split("   ").collect();
        vec.push(line_vec[0].parse().unwrap());
        if map.contains_key(&line_vec[1].parse().unwrap()){
            let value = map.get(&line_vec[1].parse().unwrap()).unwrap();
            map.insert(line_vec[1].parse().unwrap(),value + 1);
        }else{
            map.insert(line_vec[1].parse().unwrap(), 1);
        }
    }

    let mut result: i32 = 0;
    for i in 0..1000{
        let value = vec[i];
        if map.contains_key(&value){
            result += value*map.get(&value).unwrap();
        }
    }
    print!("{}", result);
}
