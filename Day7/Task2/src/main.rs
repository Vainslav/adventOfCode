use core::num;
use std::fs::read_to_string;

fn number_to_base_3_vec(mut number: u32, capacity: usize) -> Vec<u8>{
    let mut vec: Vec<u8> = vec![0; capacity];
    let mut index = vec.len() - 1;
    loop {
        let m = number % 3 as u32;
        number = number / 3 as u32;


        vec[index] = std::char::from_digit(m as u32, 3).unwrap().to_string().parse::<u8>().unwrap();
        if number == 0 {
            break;
        }
        index -= 1;
    }
    return vec;
}

fn main() {
    let mut vec:Vec<Vec<u64>> = Vec::with_capacity(850);
    for line in read_to_string("input").unwrap().lines(){
        let mut vec_to_push: Vec<u64> = vec![];
        let split = line.split(':').collect::<Vec<&str>>();
        let first = split[0].parse::<u64>().unwrap();
        vec_to_push.push(first);
        for digit in split[1].split(' '){
            let number = digit.parse::<u64>();
            if number.is_err(){
                continue;
            }
            vec_to_push.push(digit.parse::<u64>().unwrap());
        }
        vec.push(vec_to_push);
    }

    let mut cnt = 0;
    for line in vec.iter(){
        for i in 0..3_u32.pow((line.len() - 2).try_into().unwrap()){
            let operations = number_to_base_3_vec(i, line.len() - 2);
            let mut sum = line[1];
            for j in 2..line.len(){
                if operations[j-2] == 0{
                    sum += line[j];
                }else if operations[j-2] == 1{
                    sum *= line[j];
                }else{
                    sum = (sum.to_string() + &line[j].to_string()).parse().unwrap();
                }
            }
            if sum == line[0]{
                cnt += line[0];
                break;
            }
        }
    }
    print!("{}\n", cnt);
}
