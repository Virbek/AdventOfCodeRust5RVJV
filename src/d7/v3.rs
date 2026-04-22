pub fn solve_part1(input: &str) -> i64{
    let mut stock: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let size = stock.len();

    let start_col = stock[0]
        .iter()
        .position(|&c| c == 'S')
        .expect("No 'S' found in first row");

    let mut tachyon: Vec<(usize, usize)> = Vec::with_capacity(64);
    let mut tachyon_eph: Vec<(usize, usize)> = Vec::with_capacity(64);
    tachyon.push((0, start_col));

    let mut sum = 0;
    let mut ind: usize = 0;


    while ind != size-1{
        tachyon_eph.clear();
        // println!("je regarde la list avant : {}",tachyon_eph.len());
        for (i, j) in &tachyon{
            if stock[i+1][*j] == '^'{
                if stock[i+1][j-1] != '|'{
                    stock[i+1][j-1] = '|';
                    tachyon_eph.push((i+1,j-1));
                }
                if stock[i+1][j+1] != '|'{
                    stock[i+1][j+1] = '|';
                    tachyon_eph.push((i+1, j+1));
                }
                sum += 1;
            }else{
                if stock[i+1][*j] != '|'{
                    stock[i+1][*j] = '|';
                    tachyon_eph.push((i+1, *j));
                }
            }
        }
        tachyon.clear();
        std::mem::swap(&mut tachyon, &mut tachyon_eph);
        // println!("je regarde la list apres: {}",tachyon.len());
        ind +=1;
        // for i in &stock  {
        //     for j in i{
        //         print!("{}",j)
        //     }
        //     println!("");
        // }
    }

    sum

}

pub fn solve_part2(input: &str) -> i64{
    let stock: Vec<&[u8]> = input
        .lines()
        .map(|line| line.as_bytes())
        .collect();

    let size = stock.len();
    let width = stock[0].len();

    let start_col = stock[0]
        .iter()
        .position(|&c| c == b'S')
        .expect("No 'S' found in first row");

    let mut current: Vec<i64> = vec![0; width];
    let mut next: Vec<i64> = vec![0; width];

    current[start_col] = 1;

    for ind in 0..size - 1{
        let row_next = stock[ind + 1];
        let w_next = row_next.len(); 
        
        for v in next.iter_mut(){
            *v = 0;
        }

        for j in  0..stock[ind].len(){
            let val = current[j];
            if val == 0{
                continue;
            }
            match row_next[j]{
                b'^' => {
                    if j > 0 {
                        next[j - 1] += val;
                    }
                    if j+1 < w_next {
                        next[j + 1] += val;
                    }
                }
                b'|' => {}
                _ =>{
                    next[j] += val;
                }
            }
            
        }
        std::mem::swap(&mut current, &mut next);
    }
    current.iter().sum()

}