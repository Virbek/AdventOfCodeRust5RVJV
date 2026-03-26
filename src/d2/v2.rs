pub fn solve_part1(input: &str) -> u64{
    let mut sum_invalides:u64 = 0;
    let plages:Vec<&str> = input.split(',').collect();

    for plage_str in plages {
        let plage:Vec<&str> = plage_str.trim().split('-').collect();
        let min: u64 = plage[0].trim().parse().unwrap();
        let max: u64 = plage[1].trim().parse().unwrap();

        for nb in min..=max{
            let nb_str = nb.to_string();

            if nb_str.len() % 2 == 0{
                let mid = nb_str.len()/2;
                let left = &nb_str[0..mid];
                let right = &nb_str[mid..];

                if left == right{
                    sum_invalides += nb;
                }
            }
        }
    }

    sum_invalides
}

//12 12 12 => 3 fois
//12 12 => 2 fois

pub fn solve_part2(input: &str) -> u64{
    let mut sum_invalides:u64 = 0;
    let plages:Vec<&str> = input.split(',').collect();

    for plage_str in plages {
        let plage:Vec<&str> = plage_str.trim().split('-').collect();
        let min: u64 = plage[0].trim().parse().unwrap();
        let max: u64 = plage[1].trim().parse().unwrap();

        for nb in min..=max{
            let nb_str = nb.to_string();
            for taille in 1..=nb_str.len()/2{
                if nb_str.len() % taille == 0{

                    let motif = &nb_str[0..taille];
                    let mut invalide = true;

                    // De 0 à la longueur, pas = taille (longueur/2)
                    for i in (0..nb_str.len()).step_by(taille){
                        let bloc = &nb_str[i..i+taille];

                        if bloc != motif{
                            invalide = false;
                            break;
                        }
                    }

                    if invalide{
                        sum_invalides += nb;
                        break;
                    }
                }
            }
        }
    }
    sum_invalides
}