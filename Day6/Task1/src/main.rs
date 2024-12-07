use std::fs::read_to_string;

fn main() {
    let mut vec: Vec<Vec<char>> = Vec::with_capacity(10);

    for line in read_to_string("input").unwrap().lines(){
        vec.push(line.chars().collect());
    }

    let cnt = 0;
    let mut coord: (usize, usize) = (0,0);
    for i in 0..vec.len(){
        for j in 0..vec[i].len(){
            if vec!['<','>','v','^'].contains(&vec[i][j]){
                coord = (i, j);
            }
        }
    }

    while coord.0 < vec[0].len() && coord.1 < vec.len(){
        if '<' == vec[coord.0][coord.1]{
            if coord.1 == 0{
                vec[coord.0][coord.1] = 'X';
                break;
            }
            if vec[coord.0][coord.1 - 1] == '#'{
                vec[coord.0][coord.1] = '^';
                continue
            }
            vec[coord.0][coord.1] = 'X';
            coord.1 -= 1;
            vec[coord.0][coord.1] = '<';
        }
        if '^' == vec[coord.0][coord.1]{
            if coord.0 == 0{
                vec[coord.0][coord.1] = 'X';
                break;
            }
            if vec[coord.0 - 1][coord.1] == '#'{
                vec[coord.0][coord.1] = '>';
                continue
            }
            vec[coord.0][coord.1] = 'X';
            coord.0 -= 1;
            vec[coord.0][coord.1] = '^';
        }
        if '>' == vec[coord.0][coord.1]{
            if coord.1 == vec[0].len() - 1{
                vec[coord.0][coord.1] = 'X';
                break;
            }
            if vec[coord.0][coord.1 + 1] == '#'{
                vec[coord.0][coord.1] = 'v';
                continue
            }
            vec[coord.0][coord.1] = 'X';
            coord.1 += 1;
            vec[coord.0][coord.1] = '>';
        }
        if 'v' == vec[coord.0][coord.1]{
            if coord.0 == vec.len() - 1{
                vec[coord.0][coord.1] = 'X';
                break;
            }
            if vec[coord.0 + 1][coord.1] == '#'{
                vec[coord.0][coord.1] = '<';
                continue
            }
            vec[coord.0][coord.1] = 'X';
            coord.0 += 1;
            vec[coord.0][coord.1] = 'v';
        }
    }

    let mut cnt = 0;
    for i in 0..vec.len(){
        for j in 0..vec[i].len(){
            if vec[i][j] == 'X'{
                cnt += 1;
            }
        }
    }
    print!("{}", cnt);
}
