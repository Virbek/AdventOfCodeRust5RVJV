fn first_half(contenu : &'static str){
    let mut higher : u32 = 0 ;
    let mut second : u32 = 0;
    let mut transition= (0,0);
    let mut sum : u32 = 0;
    for c in contenu.chars() {
        if c == '\n'{
            if second == 0 {
                second = higher;
                higher = transition.0;
            }
            //println!("plus haut : {}, deuxieme plus haut : {}", higher,second);
            higher *=10;
            higher += second;
            sum += higher;
            higher = 0;
            second = 0;
        }else{
            let n: u32 = c.to_string().parse().unwrap();
            if n > higher {
                transition = (higher , second);
                higher = n;
                second = 0;
            }else if n > second{
                second = n;
            }
        }
    }

    println!("le resultat : {}", sum);

}

fn second_half(contenu : &'static str){
    let mut sum:u64 = 0;
    let mut jolt: Vec<u64> = Vec::new();
    let mut res: Vec<u64> = Vec::new() ;
    for c in contenu.chars() {
        if c == '\n'{
            let size = jolt.len();
            if size > 12 {
                res = jolt.split_off(size - 12);
                for j in  jolt.iter().rev(){
                    if j >= res.first().unwrap() {
                        let mut count = 0;
                        let mut check = 10;
                        for (i, &r) in res.iter().enumerate(){
                            if check < r{
                                break;
                            }
                            check = r;
                            count = i ;
                        }
                        res.remove(count);
                        res.insert(0, *j);
                    }
                }
                let mut ind:u32 = 11;
                let ten : u64 = 10;
                let mut test : u64 = 0;
                for r in &res{
                    //println!("le r : {}", r);
                    if ind > 0{
                        test += r * ten.pow(ind) ;
                        ind -= 1;
                    }else {
                        test += r;
                    } 
                }
                //println!("le test : {}", test);
                sum += test;
            }
            jolt.clear();
            res.clear();

        }else{
            jolt.push(c.to_string().parse().unwrap());
        }
    }
    println!("le resultat : {}", sum);
}
fn main() {
    let contenu = include_str!("../ressources/input.txt");
    first_half(contenu);
    second_half(contenu);
}
