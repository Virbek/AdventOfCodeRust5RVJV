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

fn main() {
    let contenu = include_str!("../ressources/code.txt");
    first_half(contenu);

}
