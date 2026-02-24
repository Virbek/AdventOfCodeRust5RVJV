fn first_half(contenu: &'static str){
    let mut stock: Vec<Vec<char>> = Vec::new();
    let mut pos:usize = 0;
    stock.push(Vec::new());

    for c in contenu.chars(){
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

    println!("le resultat est : {}", sum);

}

fn main() {
    let contenu = include_str!("../ressources/input.txt");
    first_half(contenu);
}
