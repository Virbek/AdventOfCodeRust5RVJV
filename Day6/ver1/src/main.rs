fn first_half(contenu : &'static str){
    let mut stock: Vec<Vec<String>> = Vec::new();
    let mut pos : usize = 0;
    let mut num: String = String::new();
    stock.push(Vec::new());
    for c in contenu.chars(){
        //println!("le char : {c}");
        if stock.len() < (pos+1){
            //println!("je rentre");
            stock.push(Vec::new());
            //println!("{:?}", stock)
        }
        if c == ' '{
            if !num.is_empty(){

                stock[pos].push(num.clone());
                num.clear();
                pos += 1;
            }
        }else if c == '\n'{
            // println!("saut de ligne");
            stock[pos].push(num.clone());
            num.clear();
            pos = 0;
        }else{
            num.push(c);
        }
    }


    let mut res:u64 = 0;
    let mut add = false;
    for i in stock{
        let mut sum:u64 = 0;
        for j in i.iter().rev(){
            //print!("{}", j);
            if !j.is_empty() {
                if j == "+"{
                    //println!(" plus ");
                    add = true;
                }else if j == "*"{
                    //println!(" multiplier ");
                    sum = 1;
                    add = false;
                }else if add{
                    //println!(" addition ");
                    let var: u64 = j.parse().unwrap();
                    sum += var;
                }else{
                    //println!(" je multiplie ");
                    let var: u64 = j.parse().unwrap();
                    sum *= var;
                }
            }
        }
        res += sum;
    }

    println!("le resultat : {}", res);
}

fn second_half(contenu : &'static str){
    let mut stock: Vec<Vec<String>> = Vec::new();
    let mut pos : usize = 0;
    let mut num: String = String::new();
    stock.push(Vec::new());
    for c in contenu.chars(){
        //println!("le char : {c}");
        if stock.len() < (pos+1){
            //println!("je rentre");
            stock.push(Vec::new());
            //println!("{:?}", stock)
        }
        if c == ' '{
            if !num.is_empty(){

                stock[pos].push(num.clone());
                num.clear();
                pos += 1;
            }
        }else if c == '\n'{
            // println!("saut de ligne");
            stock[pos].push(num.clone());
            num.clear();
            pos = 0;
        }else{
            num.push(c);
        }
    }
    let mut res:u64 = 0;
    let mut add = false;
    for i in stock{
        let mut sum:u64 = 0;
        for j in i.iter().rev(){
            //print!("{}", j);
            if !j.is_empty() {
                if j == "+"{
                    //println!(" plus ");
                    add = true;
                }else if j == "*"{
                    //println!(" multiplier ");
                    sum = 1;
                    add = false;
                }else if add{
                    //println!(" addition ");
                    let var: u64 = j.parse().unwrap();
                    sum += var;
                }else{
                    //println!(" je multiplie ");
                    let var: u64 = j.parse().unwrap();
                    sum *= var;
                }
            }
        }
        res += sum;
    }

    println!("le resultat : {}", res);
}

fn main() {
    let contenu = include_str!("../ressources/input.txt");
    first_half(contenu);
    // second_half(contenu);
}
