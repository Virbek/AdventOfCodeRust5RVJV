fn first_half(contenu : &'static str){
    let mut limite = String::new();
    let mut start = String::new();
    let mut invalide : u64 = 0;

    for c in contenu.chars(){
        if c == ',' || c == '\n' {
            let s : u32 = start.parse().unwrap();
            let l : u32 = limite.parse().unwrap();
            //println!("start : {}, limite : {}", s, l);
            for i in s..=l {
                let v = i.to_string();
                if v.len() % 2 == 0{
                    //println!("valeur : {}", v);
                    start = v;
                    limite = start.split_off(start.len()/2);
                    let r1 : u32 = start.parse().unwrap();
                    let r2 : u32 = limite.parse().unwrap();
                    if r1 == r2 {
                        start.push_str(&limite);
                        let res :u64 = start.parse().unwrap();
                        //println!("resultat : {}", res);
                        invalide += res;
                    }
                } 
            }
            start.clear();
            limite.clear();
        }else{
            if c == '-'{
                start = limite.to_string();
                limite.clear();
            }else {
                limite.push(c);
            }
        }
        //println!("Mais que vaut C : {}", c);
    }

    println!("le resultat : {}", invalide);
}

fn second_half(contenu : &'static str){
    let mut limite = String::new();
    let mut start = String::new();
    let mut invalide : u64 = 0;

    for c in contenu.chars(){
        if c == ',' || c == '\n' {
            let s : u32 = start.parse().unwrap();
            let l : u32 = limite.parse().unwrap();
            //println!("start : {}, limite : {}", s, l);
            for i in s..=l {
                let mut find = false;
                let v = i.to_string();
                if v.len() % 2 == 0 && !find{
                    //println!("valeur par 2 : {}", v);
                    start = v.clone();
                    limite = start.split_off(start.len()/2);
                    let r1 : u32 = start.parse().unwrap();
                    let r2 : u32 = limite.parse().unwrap();
                    if r1 == r2 {
                        find = true;
                        start.push_str(&limite);
                        let res :u64 = start.parse().unwrap();
                        invalide += res;
                    }
                } 
                if v.len() % 3 == 0 && !find{
                    //println!("valeur par 3 : {}", v);
                    //println!("la division : {}", v.len()/3);

                    start = v.clone();
                    limite = start.split_off(start.len()/3);
                    let third = limite.split_off(limite.len()/2);
                    //println!("start : {}, limite : {}, third : {}", start, limite, third);
                    let r1 : u32 = start.parse().unwrap();
                    let r2 : u32 = limite.parse().unwrap();
                    let r3 : u32 = third.parse().unwrap();
                    if r1 == r2 && r2 == r3{
                        find = true;
                        start.push_str(&limite);
                        start.push_str(&third);
                        let res :u64 = start.parse().unwrap();
                        invalide += res;
                    }
                } 
                if v.len() % 5 == 0 && !find{
                    //println!("valeur par 5 : {}", v);
                    //println!("la division 5 : {}", v.len()/5);

                    start = v.clone();
                    limite = start.split_off(start.len()/5);
                    let mut third = limite.split_off(limite.len()/4);
                    let mut fourth = third.split_off(third.len()/3);
                    let five = fourth.split_off(fourth.len()/2);
                    //println!("start : {}, limite : {}, third : {}", start, limite, third);
                    let r1 : u32 = start.parse().unwrap();
                    let r2 : u32 = limite.parse().unwrap();
                    let r3 : u32 = third.parse().unwrap();
                    let r4 : u32 = fourth.parse().unwrap();
                    let r5 : u32 = five.parse().unwrap();
                    //println!("r1 : {}, r2 : {}, r3 :{}, r4 : {}, r5 : {}", r1,r2,r3,r4,r5);
                    if r1 == r2 && r2 == r3 && r3 == r4 && r4 == r5{
                        start.push_str(&limite);
                        start.push_str(&third);
                        start.push_str(&fourth);
                        start.push_str(&five);
                        let res :u64 = start.parse().unwrap();
                        invalide += res;
                    }
                }
                if v.len() % 7 == 0 && !find{
                    //println!("valeur par 7 : {}", v);
                    //println!("la division 7 : {}", v.len()/7);
                    start = v.clone();
                    limite = start.split_off(start.len()/7);
                    let mut third = limite.split_off(limite.len()/6);
                    let mut fourth = third.split_off(third.len()/5);
                    let mut five = fourth.split_off(fourth.len()/4);
                    let mut six = five.split_off(five.len()/3);
                    let seven = six.split_off(six.len()/2);
                    //println!("start : {}, limite : {}, third : {}", start, limite, third);
                    let r1 : u32 = start.parse().unwrap();
                    let r2 : u32 = limite.parse().unwrap();
                    let r3 : u32 = third.parse().unwrap();
                    let r4 : u32 = fourth.parse().unwrap();
                    let r5 : u32 = five.parse().unwrap();
                    let r6 : u32 = six.parse().unwrap();
                    let r7 : u32 = seven.parse().unwrap();
                     
                    if r1 == r2 && r2 == r3 && r3 == r4 && r4 == r5 && r5 == r6 && r6 == r7{
                        start.push_str(&limite);
                        start.push_str(&third);
                        start.push_str(&fourth);
                        start.push_str(&five);
                        start.push_str(&six);
                        start.push_str(&seven);
                        let res :u64 = start.parse().unwrap();
                        invalide += res;
                    }
                } 

            }
            start.clear();
            limite.clear();
        }else{
            if c == '-'{
                start = limite.to_string();
                limite.clear();
            }else {
                limite.push(c);
            }
        }
        //println!("Mais que vaut C : {}", c);
    }

    println!("le resultat : {}", invalide);
}

fn main() {
    let contenu = include_str!("../ressources/code.txt");
    first_half(&contenu);
    second_half(&contenu);

}
