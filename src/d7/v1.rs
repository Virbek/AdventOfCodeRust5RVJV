pub fn solve_part1(input: &str) -> i64{
    let mut stock: Vec<Vec<char>> = Vec::new();
    let mut pos:usize = 0;
    stock.push(Vec::new());

    for c in input.chars(){
        if c == '\n'{
            pos += 1;
            stock.push(Vec::new());
        }else{
            stock[pos].push(c)
        }
    }
    let size = pos;
    pos = 0;
    let mut found: bool = false;
    while !found{
        if stock[0][pos] == 'S'{
            found = true;
            pos -= 1;
        }
        pos +=1;
    }

    let mut ind: usize = 0;
    let mut tachyon: Vec<(usize,usize)> = Vec::new();
    tachyon.push((ind,pos));
    let mut sum = 0;

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
    let mut stock: Vec<Vec<char>> = Vec::new();
    let mut pos:usize = 0;
    stock.push(Vec::new());

    //je recupere le fichier input dans une matrice pour l'utiliser plus tard
    for c in input.chars(){
        if c == '\n'{
            pos += 1;
            stock.push(Vec::new());
        }else{
            stock[pos].push(c)
        }
    }
    let size = pos;
    pos = 0;
    let mut found: bool = false;

    //je cherche la position de S pour commencer au bonne endroit
    while !found{
        if stock[0][pos] == 'S'{
            found = true;
            pos -= 1;
        }
        pos +=1;
    }

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
    tab_num[ind][pos] = 1;
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