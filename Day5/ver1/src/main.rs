fn first_half(contenu : &'static str){

    let mut index: Vec<(u64, u64)> = Vec::new();
    let mut check = false;
    let mut save_first: String = String::new();
    let mut save_last: String = String::new();
    let mut pos = 0;
    let mut count = 0;


    for (i, c) in contenu.chars().enumerate(){
        if c == '-' {
            check = true;
        }else if c == '\n' { 
            if count == 1 {
                //println!("j'essaie d'arreter la boucle");
                break;
            }else{
                //println!("first : {}, last : {}", save_first, save_last);
                let x: u64 = save_first.parse().unwrap();
                let y: u64 = save_last.parse().unwrap();

                //println!("x : {}, y : {}", x, y);
                index.push((x,y));
                count = 1;
                check = false;
                save_first.clear();
                save_last.clear();
            }
        }else{
            count = 0;
            if check {
                save_last.push(c);
            }else{
                save_first.push(c);
            }          
        }
        pos = i;
    }

    let reste: String = contenu.split_at(pos).1.to_string();
    //println!("{contenu}, {reste}");
    let mut contenu: Vec<u64> = Vec::new(); 
    let mut save = String::new();
    
    for c in reste.chars() {
        if c == '\n' {
            if !save.is_empty(){
                //println!("save : {}", save);
                let x: u64 = save.parse().unwrap();
                contenu.push(x);
                save.clear();
            }   
        }else{
            save.push(c)
        }
    }
    let mut pos_rem: Vec<usize> = Vec::new();
    let mut count: usize = 0;
    let mut res: u32 = 0;
    for (x,y) in &index{
        for i in &contenu {
            count += 1;
            if i > x && i < y{
                //println!("l'ingredient : {}, entre : {} et {}", i , x, y);
                res += 1;
                pos_rem.push(count);
                
            }
        }
        //println!("la taille : {}", count);
        count = 0;
        for i in pos_rem.iter().rev() {
            //println!("position : {}, contenu size : {}", i , contenu.len());
            contenu.remove(*i - 1);
        }
        pos_rem.clear();
    }

    println!("le resultat : {}", res);

}

fn main() {
    let contenu = include_str!("../ressources/input.txt");
    first_half(&contenu);
}


