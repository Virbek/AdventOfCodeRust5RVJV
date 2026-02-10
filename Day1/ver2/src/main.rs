
fn main() {
    let contenu = include_str!("../ressources/code.txt");

    let mut result: u16 = 0;
    let mut chest: i8 = 50;

    
    let mut combinaison = String::new();

    for c in contenu.chars(){
        if c == '\n' { 
            let direction  = combinaison.split_off(1);
            let direction: u16 = direction
                                    .parse()
                                    .unwrap();
            for _ in 0.. direction{
                if combinaison == "R" {
                    chest += 1;
                    if chest > 99 {
                        chest = 0 ;
                    }
                }else {
                    chest -= 1;
                    if chest < 0 {
                        chest = 99;
                    }
                }
                if chest == 0 {result += 1; }
            } 
            //if chest == 0 {result += 1; }   
            combinaison.clear();
        }else {
            combinaison.push(c);
        }
    }

    println!("le password est : {}", result);
 
}