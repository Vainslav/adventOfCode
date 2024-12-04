use std::fs::read_to_string;

fn main() {
    let mut vec: Vec<String> = Vec::with_capacity(140);
    for line in read_to_string("test").unwrap().lines(){
        vec.push(line.to_string());
    }

    let mut cnt: usize = 0;
    for i in 0..vec.len(){
        for j in 0..vec[i].len(){
            if vec[i].chars().nth(j).unwrap() == 'X'{
                if !vec![0, 1, 2].contains(&i){
                    if !vec![0, 1, 2].contains(&j){
                        if vec[i - 1].chars().nth(j-1).unwrap() == 'M' &&
                        vec[i - 2].chars().nth(j-2).unwrap() == 'A' &&
                        vec[i - 3].chars().nth(j-3).unwrap() == 'S'{
                            cnt += 1;
                        }
                    }
                    if vec[i - 1].chars().nth(j).unwrap() == 'M' &&
                    vec[i - 2].chars().nth(j).unwrap() == 'A' &&
                    vec[i - 3].chars().nth(j).unwrap() == 'S'{
                        cnt += 1;
                    }
                    if !vec![vec[i].len() - 1, vec[i].len() - 2, vec[i].len() - 3].contains(&j){
                        if vec[i - 1].chars().nth(j+1).unwrap() == 'M' &&
                        vec[i - 2].chars().nth(j+2).unwrap() == 'A' &&
                        vec[i - 3].chars().nth(j+3).unwrap() == 'S'{
                            cnt += 1;
                        }
                    }
                }
                if !vec![0, 1, 2].contains(&j){
                    if vec[i].chars().nth(j-1).unwrap() == 'M' &&
                        vec[i].chars().nth(j-2).unwrap() == 'A' &&
                        vec[i].chars().nth(j-3).unwrap() == 'S'{
                            cnt += 1;
                    }
                }
                if !vec![vec[i].len() - 1, vec[i].len() - 2, vec[i].len() - 3].contains(&j){
                    if vec[i].chars().nth(j+1).unwrap() == 'M' &&
                    vec[i].chars().nth(j+2).unwrap() == 'A' &&
                    vec[i].chars().nth(j+3).unwrap() == 'S'{
                        cnt += 1;
                    }
                }
                if !vec![vec.len() - 1, vec.len() - 2, vec.len() - 3].contains(&i){
                    if !vec![0, 1, 2].contains(&j){
                        if vec[i + 1].chars().nth(j-1).unwrap() == 'M' &&
                        vec[i + 2].chars().nth(j-2).unwrap() == 'A' &&
                        vec[i + 3].chars().nth(j-3).unwrap() == 'S'{
                            cnt += 1;
                        }
                    }
                    if vec[i + 1].chars().nth(j).unwrap() == 'M' &&
                    vec[i + 2].chars().nth(j).unwrap() == 'A' &&
                    vec[i + 3].chars().nth(j).unwrap() == 'S'{
                        cnt += 1;
                    }
                    if !vec![vec[i].len(), vec[i].len()-1, vec[i].len()-2].contains(&j){
                        if vec[i + 1].chars().nth(j+1).unwrap() == 'M' &&
                        vec[i + 2].chars().nth(j+2).unwrap() == 'A' &&
                        vec[i + 3].chars().nth(j+3).unwrap() == 'S'{
                            cnt += 1;
                        }
                    }
                }
            }
        }
    }
    print!("{}", cnt);
}
