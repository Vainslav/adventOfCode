use std::{fs::read_to_string, path::Path};

fn main() {
    let path = Path::new("input");
    let mut vec1: Vec<i32> = Vec::with_capacity(1000);
    let mut vec2: Vec<i32> = Vec::with_capacity(1000);
    for line in read_to_string(&path).unwrap().lines(){
        let line_vec:Vec<&str> = line.split("   ").collect();
        vec1.push(line_vec[0].parse().unwrap());
        vec2.push(line_vec[1].parse().unwrap());
    }
    vec1.sort();
    vec2.sort();

    let mut result = 0;
    for i in 0..1000{
        result += (vec1[i]-vec2[i]).abs();
    }
    print!("{}", result);
}
