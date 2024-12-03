use std::path::Path;
use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let path: &Path = Path::new("input");
    let str = read_to_string(&path).unwrap();
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut cnt = 0;
    let mut do_mul = true;
    for caps in regex.captures_iter(&str){
        if &caps[0] == "do()"{
            do_mul = true;
            continue;
        }
        if &caps[0] == "don't()"{
            do_mul = false;
            continue;
        }
        if do_mul{
            cnt += &caps[1].parse::<i32>().unwrap() * &caps[2].parse::<i32>().unwrap();
        }
    }
    print!("{}", cnt);
}
