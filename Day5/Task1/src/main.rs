use std::fs::read_to_string;

fn main() {
    let mut vec_tuples: Vec<(i16,i16)> = Vec::with_capacity(1176);
    let mut vec: Vec<String> = Vec::with_capacity(400);
    for (i, line) in read_to_string("input").unwrap().lines().enumerate(){
        if i == 1176{
            continue;
        }
        if i < 1176{
            let str = line.split("|").collect::<Vec<&str>>();
            vec_tuples.push((str[0].parse().unwrap(), str[1].parse().unwrap()))
        }
        else{
            vec.push(line.to_string());
        }
    }

    let mut flag: bool;
    let mut cnt: usize = 0;
    for line in vec.iter(){
        let line_split: Vec<&str> = line.split(',').collect();
        flag = true;
        let middle: i16 = line_split[line_split.len()/2].parse().unwrap();
        for i in 0..line_split.len(){
            for j in i+1..line_split.len(){
                let tuple: (i16, i16) = (line_split[i].parse().unwrap(), line_split[j].parse().unwrap());
                if !vec_tuples.contains(&tuple){
                    flag = false;
                    break
                }
            }
            if !flag{
                break;
            }
        }
        if flag{
            cnt += middle as usize;
        }
    }
    print!("{}", cnt)
}
