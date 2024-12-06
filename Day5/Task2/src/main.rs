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
        let mut correct_line: Vec<&str> = vec![""; line_split.len()];
        flag = false;
        for i in 0..line_split.len(){
            let mut incorrect_count = 0;
            for j in 0..line_split.len(){
                if i==j{
                    continue;
                }
                let tuple: (i16, i16) = (line_split[i].parse().unwrap(), line_split[j].parse().unwrap());
                if !vec_tuples.contains(&tuple){
                    if i < j{
                        flag = true;
                    }
                    incorrect_count += 1;
                }
            };
            correct_line[incorrect_count] = line_split[i];
        }
        if flag{
            let middle: i32 = correct_line[correct_line.len()/2].parse().unwrap();
            print!("{}\n", middle);
            cnt += middle as usize;
        }
    }
    print!("{}", cnt)
}
