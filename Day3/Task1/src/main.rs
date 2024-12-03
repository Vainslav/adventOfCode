use std::path::Path;
use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let path: &Path = Path::new("input");
    let str = read_to_string(&path).unwrap();
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut cnt = 0;
    for caps in regex.captures_iter(&str){
        let first = &caps[1].parse::<i32>().unwrap();
        let second = &caps[2].parse::<i>().unwrap();
        cnt += first*second;
    }
    print!("{}", cnt);
}
