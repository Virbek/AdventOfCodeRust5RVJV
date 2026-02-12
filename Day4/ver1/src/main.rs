
fn first_half(contenu : &'static str){
    let size = 136;
    let mut matrix = vec![vec![0; size]; size];
    let mut lenght = 0;
    let mut height = 0;
    for c in contenu.chars(){
        if c == '\n'{
            height += 1;
            lenght = 0;
        }else{
            if c =='.'{
                matrix[lenght][height] = 0;
            }else{
                matrix[lenght][height] = 1;
            }
            lenght += 1;
        }
    }    

    let mut sum = 0;

    for y in 0..size {
        for x in 0..size {
            if matrix[x][y] == 1{
                let mut count = 0;
                if x == 0{
                    if y == 0{
                        if matrix[x+1][y] == 1{
                            count += 1;
                        }
                        if matrix[x][y+1] == 1{
                            count += 1;
                        }
                        if matrix[x+1][y+1] == 1{
                            count += 1;
                        }
                    }else if y == size-1{
                        if matrix[x][y-1] == 1{
                            count += 1;
                        }
                        if matrix[x+1][y-1] == 1{
                            count += 1;
                        }
                        if matrix[x+1][y] == 1{
                            count += 1;
                        }
                    }else{
                        if matrix[x+1][y] == 1{
                            count += 1;
                        }
                        if matrix[x][y+1] == 1{
                            count += 1;
                        }
                        if matrix[x+1][y+1] == 1{
                            count += 1;
                        }
                        if matrix[x][y-1] == 1{
                            count += 1;
                        }
                        if matrix[x+1][y-1] == 1{
                            count += 1;
                        }
                    }
                }else if x == size-1{
                    if y == 0{
                        if matrix[x-1][y] == 1{
                            count += 1;
                        }
                        if matrix[x][y+1] == 1{
                            count += 1;
                        }
                        if matrix[x-1][y+1] == 1{
                            count += 1;
                        }
                    }else if y == size-1{
                        if matrix[x][y-1] == 1{
                            count += 1;
                        }
                        if matrix[x-1][y-1] == 1{
                            count += 1;
                        }
                        if matrix[x-1][y] == 1{
                            count += 1;
                        }
                    }else{
                        if matrix[x-1][y] == 1{
                            count += 1;
                        }
                        if matrix[x][y+1] == 1{
                            count += 1;
                        }
                        if matrix[x-1][y+1] == 1{
                            count += 1;
                        }
                        if matrix[x][y-1] == 1{
                            count += 1;
                        }
                        if matrix[x-1][y-1] == 1{
                            count += 1;
                        }
                    }
                }else{
                    if y == 0{
                        if matrix[x-1][y] == 1{
                            count += 1;
                        }
                        if matrix[x][y+1] == 1{
                            count += 1;
                        }
                        if matrix[x-1][y+1] == 1{
                            count += 1;
                        }
                        if matrix[x+1][y+1] == 1{
                            count += 1;
                        }
                        if matrix[x+1][y] == 1{
                            count += 1;
                        }
                    }else if y == size-1{
                        if matrix[x-1][y] == 1{
                            count += 1;
                        }
                        if matrix[x][y-1] == 1{
                            count += 1;
                        }
                        if matrix[x-1][y-1] == 1{
                            count += 1;
                        }
                        if matrix[x+1][y-1] == 1{
                            count += 1;
                        }
                        if matrix[x+1][y] == 1{
                            count += 1;
                        }
                    }else{
                        if matrix[x-1][y-1] == 1{
                            count += 1;
                        }
                        if matrix[x][y-1] == 1{
                            count += 1;
                        }
                        if matrix[x+1][y-1] == 1{
                            count += 1;
                        }
                        if matrix[x-1][y] == 1{
                            count += 1;
                        }
                        if matrix[x+1][y] == 1{
                            count += 1;
                        }
                        if matrix[x-1][y+1] == 1{
                            count += 1;
                        }
                        if matrix[x][y+1] == 1{
                            count += 1;
                        }
                        if matrix[x+1][y+1] == 1{
                            count += 1;
                        }
                    }
                }
                if count < 4 {
                    sum += 1;
                    //println!("x : {}, y : {}", x,y);
                }
            }
        }      
    }
    print!("le resultat : {}", sum);
}

fn main() {
    let mut contenu = include_str!("../ressources/input.txt");
    first_half(contenu);

}
