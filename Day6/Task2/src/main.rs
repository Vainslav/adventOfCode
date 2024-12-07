use std::fs::read_to_string;

fn main() {
    let mut vec: Vec<Vec<char>> = Vec::with_capacity(10);

    for line in read_to_string("test").unwrap().lines(){
        vec.push(line.chars().collect());
    }

    let mut coord: (usize, usize) = (0,0);
    for i in 0..vec.len(){
        for j in 0..vec[i].len(){
            if vec!['<','>','v','^'].contains(&vec[i][j]){
                coord = (i, j);
            }
        }
    }

    let start = coord;
    let arrow = vec[coord.0][coord.1];
    let mut cnt = vec.len()*vec[0].len();

    for i in 0..vec.len(){
        for j in 0..vec[i].len(){
            if (i, j) == start{
                cnt -= 1;
                continue
            }
            vec[start.0][start.1] = arrow;
            coord = start;
            let back = vec[i][j];
            vec[i][j] = '#';
            let mut cycle = 0;
            while cycle < 15000{
                cycle += 1;
                if '<' == vec[coord.0][coord.1]{
                    if coord.1 == 0{
                        cnt -= 1;
                        break;
                    }
                    if vec[coord.0][coord.1 - 1] == '#'{
                        vec[coord.0][coord.1] = '^';
                        continue
                    }
                    coord.1 -= 1;
                    vec[coord.0][coord.1] = '<';
                }
                if '^' == vec[coord.0][coord.1]{
                    if coord.0 == 0{
                        cnt -= 1;
                        break;
                    }
                    if vec[coord.0 - 1][coord.1] == '#'{
                        vec[coord.0][coord.1] = '>';
                        continue
                    }
                    coord.0 -= 1;
                    vec[coord.0][coord.1] = '^';
                }
                if '>' == vec[coord.0][coord.1]{
                    if coord.1 == vec[0].len() - 1{
                        cnt -= 1;
                        break;
                    }
                    if vec[coord.0][coord.1 + 1] == '#'{
                        vec[coord.0][coord.1] = 'v';
                        continue
                    }
                    coord.1 += 1;
                    vec[coord.0][coord.1] = '>';
                }
                if 'v' == vec[coord.0][coord.1]{
                    if coord.0 == vec.len() - 1{
                        cnt -= 1;
                        break;
                    }
                    if vec[coord.0 + 1][coord.1] == '#'{
                        vec[coord.0][coord.1] = '<';
                        continue
                    }
                    coord.0 += 1;
                    vec[coord.0][coord.1] = 'v';
                }
            }
            vec[i][j] = back;
        }
    }

    print!("{}", cnt);
}
