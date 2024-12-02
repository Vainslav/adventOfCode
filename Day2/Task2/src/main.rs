use std::path::Path;
use std::fs::read_to_string;


fn is_safe(vec: &Vec<i32>) -> bool{
    if vec[1] == vec[0]{
        return false;
    }
    let mut last: i32 = vec[0];
    let is_asc:bool = vec[1] > vec[0];
    for j in 1..vec.len(){
        let mut difference = vec[j] - last;
        if difference == 0 {
            return false;
        }
        if !is_asc{
            difference *= -1;
        }
        if difference < 0 || difference > 3{
            return false;
        }
        last = vec[j];
    }
    return true;
}

fn main() {
    let path: &Path = Path::new("input");
    let mut vec: Vec<Vec<i32>> = Vec::with_capacity(1000);
    for line in read_to_string(&path).unwrap().lines(){
        vec.push(line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect());
    }

    let mut cnt:usize = 0;
    for i in 0..1000{
        if is_safe(&vec[i]){
            cnt += 1;
        }else{
            for j in 0..vec[i].len(){
                let mut clone = vec[i].clone();
                clone.remove(j);
                if is_safe(&clone){
                    cnt += 1;
                    break;
                }
            }
        }
    }
    print!("{}\n", cnt);
}
