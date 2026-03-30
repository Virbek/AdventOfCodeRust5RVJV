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
        let mut tachyon_eph: Vec<(usize,usize)> = Vec::new();
        // println!("je regarde la listavant : {}",tachyon_eph.len());
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
        tachyon = tachyon_eph;
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
    let stock: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let size = stock.len();

    let start_col = stock[0]
        .iter()
        .position(|&c| c == 'S')
        .expect("No 'S' found in first row");

    //je cree un tableau de 0 pour pouvoir compter en parrallele 
    let mut tab_num: Vec<Vec<i64>> = Vec::new();
    
    let mut index = 0; 
    for i in &stock{
        tab_num.push(Vec::new());
        for j in i {
            tab_num[index].push(0);
        }
        index += 1;
    }

    let mut ind: usize = 0;
    tab_num[ind][start_col] = 1;
    let mut sum: i64 = 0;

    while ind != size-1{

        // println!("je regarde la listavant : {}",tachyon_eph.len());
        for j in  0..stock[ind].len(){
            if ind+1 < stock.len(){
                if stock[ind+1][j] == '^'{
                    if j > 0 {
                        tab_num[ind + 1][j - 1] += tab_num[ind][j];
                    }
                    if j + 1 < stock[ind + 1].len() {
                        tab_num[ind + 1][j + 1] += tab_num[ind][j];
                    }
                } else {
                    if stock[ind + 1][j] != '|' {
                        tab_num[ind + 1][j] += tab_num[ind][j];

                    }
                }
            }
            
        }
        // println!("je regarde la list apres: {}",tachyon.len());
        ind +=1;
        // for i in &stock  {
        //     for j in i{
        //         print!("{}",j)
        //     }
        //     println!("");
        // }
    }
    for i in &tab_num[ind]{
        sum += i;
    }

    sum

}