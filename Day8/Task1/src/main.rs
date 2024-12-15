use std::fs::read_to_string;

fn main() {
    let capacity = 50;
    let mut input: Vec<Vec<char>> = Vec::with_capacity(capacity);
    for line in read_to_string("input").unwrap().lines(){
        let vec_to_push: Vec<char> = line.chars().collect();
        input.push(vec_to_push);
    }

    let chars = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
                                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
                                'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
                                'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G',
                                'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
                                'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    let mut vec: Vec<Vec<(usize, usize)>> = vec![Vec::with_capacity(100); 62];
    for i in 0..capacity{
        for j in 0..capacity{
            if chars.contains(&input[i][j]){
                let index = chars.iter().position(|&r| r == input[i][j]).unwrap();
                vec[index].push((j, i));
            }
        }
    }

    let mut cnt = 0;
    for frequency in vec.iter(){
        for i in 0..frequency.len(){
            for j in i+1..frequency.len(){
                let y_diffrence = frequency[j].1 - frequency[i].1;
                let x_diffrence = (frequency[j].0 as i64 - frequency[i].0 as i64).abs() as usize;
                let mut first: Option<Vec<(usize,usize)>> = None;
                let mut second: Option<Vec<(usize,usize)>> = None;
                first.get_or_insert(Vec::new()).push((frequency[i].0,frequency[i].1));
                second.get_or_insert(Vec::new()).push((frequency[j].0,frequency[j].1));
                if frequency[i].0 > frequency[j].0{
                    if frequency[i].0 + x_diffrence < capacity && frequency[i].1 >= y_diffrence{
                        let mut l = frequency[i].0 + x_diffrence;
                        let mut k = frequency[i].1 - y_diffrence;
                        first.insert(Vec::new()).push((l,k));
                        while l + x_diffrence < capacity && k >= y_diffrence{
                            l += x_diffrence;
                            k -= y_diffrence;
                            first.get_or_insert(Vec::new()).push((l,k));
                        }
                    }
                    if frequency[j].0 >= x_diffrence && frequency[j].1 + y_diffrence < capacity{
                        let mut l = frequency[j].0 - x_diffrence;
                        let mut k = frequency[j].1 + y_diffrence;
                        second.get_or_insert(Vec::new()).push((l,k));
                        while l >= x_diffrence && k + y_diffrence < capacity{
                            l -= x_diffrence;
                            k += y_diffrence;
                            second.get_or_insert(Vec::new()).push((l,k));
                        }
                    }
                }else{
                    if frequency[i].0 >= x_diffrence && frequency[i].1 >= y_diffrence{
                        let mut l = frequency[i].0 - x_diffrence;
                        let mut k = frequency[i].1 - y_diffrence;
                        first.get_or_insert(Vec::new()).push((l,k));
                        while l >= x_diffrence && k >= y_diffrence{
                            l -= x_diffrence;
                            k -= y_diffrence;
                            first.get_or_insert(Vec::new()).push((l,k));
                        }
                    }
                    if frequency[j].0 + x_diffrence < capacity && frequency[j].1 + y_diffrence < capacity{
                        let mut l = frequency[j].0 + x_diffrence;
                        let mut k = frequency[j].1 + y_diffrence;
                        second.get_or_insert(Vec::new()).push((l,k));
                        while l + x_diffrence < capacity && k + y_diffrence < capacity{
                            l += x_diffrence;
                            k += y_diffrence;
                            second.get_or_insert(Vec::new()).push((l,k));
                        }
                    }
                }

                if first.is_some(){
                    let first_unwraped = first.unwrap();
                    for i in first_unwraped.iter(){
                        if input[i.1][i.0] != '#' && !chars.contains(&input[i.1][i.0]){
                            cnt += 1;
                            input[i.1][i.0] = '#';
                        }
                    }
                }
                if second.is_some(){
                    let second_unwraped = second.unwrap();
                    for i in second_unwraped.iter(){
                        if input[i.1][i.0] != '#' && !chars.contains(&input[i.1][i.0]){
                            cnt += 1;
                            input[i.1][i.0] = '#';
                        }
                    }
                }
            }
        }
    }
    for line in input.iter(){
        for char in line.iter(){
            if *char != '#' && *char != '.'{
                cnt+=1;
            }
            print!("{}", char)
        }
        print!("\n")
    }
    print!("{}\n", cnt);
}
