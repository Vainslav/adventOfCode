use std::fs::read_to_string;

fn main() {
    let mut vec: Vec<String> = Vec::with_capacity(140);
    for line in read_to_string("input").unwrap().lines(){
        vec.push(line.to_string());
    }

    let mut cnt: usize = 0;
    for i in 0..vec.len(){
        for j in 0..vec[i].len(){
            if vec[i].chars().nth(j).unwrap() == 'A'{
                if (i != 0) && (j != 0) && (j != vec[i].len() - 1) && (i != vec.len() - 1){
                    if vec[i-1].chars().nth(j-1).unwrap() == 'M' && vec[i+1].chars().nth(j+1).unwrap() == 'S' && vec[i-1].chars().nth(j+1).unwrap() == 'M' && vec[i+1].chars().nth(j-1).unwrap() == 'S' ||
                    vec[i-1].chars().nth(j-1).unwrap() == 'M' && vec[i+1].chars().nth(j+1).unwrap() == 'S' && vec[i-1].chars().nth(j+1).unwrap() == 'S' && vec[i+1].chars().nth(j-1).unwrap() == 'M' ||
                    vec[i-1].chars().nth(j-1).unwrap() == 'S' && vec[i+1].chars().nth(j+1).unwrap() == 'M' && vec[i-1].chars().nth(j+1).unwrap() == 'M' && vec[i+1].chars().nth(j-1).unwrap() == 'S' ||
                    vec[i-1].chars().nth(j-1).unwrap() == 'S' && vec[i+1].chars().nth(j+1).unwrap() == 'M' && vec[i-1].chars().nth(j+1).unwrap() == 'S' && vec[i+1].chars().nth(j-1).unwrap() == 'M'{
                        cnt += 1;
                    }
                }
            }
        }
    }
    print!("{}", cnt);
}
