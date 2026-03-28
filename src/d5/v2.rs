// fn contains(var : u64, tab : &Vec<u64>) -> bool{
//     for i in tab{
//         if var == *i {
//             return true;
//         }
//     }
//     false
// }



pub fn solve_part1(input: &str) -> u32{

    let mut lines = input.lines();

    let index: Vec<(u64, u64)> = lines.by_ref()
        .take_while(|l| !l.trim().is_empty())
        .filter_map(|line| line.split_once('-'))
        .map(|(a, b)| (a.trim().parse().unwrap(), b.trim().parse().unwrap()))
        .collect();

    let mut contenu: Vec<u64> = lines
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim().parse().unwrap())
        .collect();

    let mut pos_rem: Vec<usize> = Vec::new();
    let mut res: u32 = 0;

    for (x,y) in &index{
        for (i, val) in contenu.iter().enumerate() {
            if val > x && val < y{
                res += 1;
                pos_rem.push(i);
            }
        }

        contenu = contenu
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !pos_rem.contains(i))
            .map(|(_, v)| v)
            .collect();
        pos_rem.clear();
    }

    res

}

pub fn solve_part2(input: &str) -> u64{
    let mut lines = input.lines();

    let mut index: Vec<(u64, u64)> = lines
        .by_ref()
        .take_while(|l| !l.trim().is_empty())
        .filter_map(|line| line.split_once('-'))
        .map(|(a, b)| (a.trim().parse().unwrap(), b.trim().parse().unwrap()))
        .collect();

    index.sort();

    let mut all_ingredient: Vec<(u64,u64)> = Vec::new();
    let mut res: u64 = 0;

    for (x,y) in index{
        let mut size: usize = 0;
        //println!("j'affiche x et y : {} - {}",x , y);
        //println!("je check un vecteur vide : {}", all_ingredient.len());
        if all_ingredient.len() != 0{
            size = all_ingredient.len() - 1;
        }
        if size == 0 && all_ingredient.len() == 0{
            all_ingredient.push((x,y));
        }else{
            if x <= all_ingredient[size].1 && y > all_ingredient[size].1{
                //print!("je rentre dans 1 ");
                all_ingredient[size].1 = y;
            }else if x > all_ingredient[size].1{
                //print!("je rentre dans 2 ");
                all_ingredient.push((x,y));
            }
        }
        //println!("verif : {} - {}", all_ingredient[size].0, all_ingredient[size].1 );

    }

    for (x,y) in &all_ingredient  {
        //println!("le x et le y : {} - {}", x,y);
        res+= (y - x) +1;
        //println!("le resultat : {}", res);
    }
    res
}