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
    let mut stock: Vec<String> = Vec::new();
    let mut pos : usize = 0;
    let mut pos_in : usize = 0;
    let contenu = contenu.replace(" ", "0");
    for c in contenu.chars(){
        if stock.len() < pos +1 {
            stock.push(String::new());
        }
        if c == '\n'{
            pos = 0;
            pos_in = 0;
        }else{ 
            stock[pos].push(c);
            pos += 1;
            
            
        }
    }
    let mut res:u64 = 0;
    // println!("{:?}", stock);
    let mut var: u64 = 0;
    let new_var:u64 = 0;
    let mut sign: String = String::new();
    for i in stock{
        if i != "00000"{
            // println!("le i : {}", i);
            if i.contains('*') || i.contains('+'){
                let (var_string, sign_string) = i.split_at(i.len()-1);
                let mut var_string:String = var_string.to_string();
                if var_string.contains('0'){
                    var_string = var_string.replace('0', "");
                }
                var = var_string.parse().unwrap();
                sign = sign_string.to_string();
                // print!("{}-{}",sign, var);
            }else if i.contains('0'){
                let new_var = i.replace('0', "");
                let new_var:u64 = new_var.parse().unwrap();
                // print!("{}-{}-{}-",sign, var, new_var);
                if(sign.contains('+')){
                    var += new_var;
                }
                if sign.contains('*'){
                    var *= new_var;
                }
                
            }
        }else{
            res += var;
            // println!("le resultat :{res}");
        }
    }
    res += var;
    println!("le resultat : {}", res);
}

fn main() {
    let contenu = include_str!("../ressources/input.txt");
    first_half(contenu);
    second_half(contenu);
}
