pub fn solve_part1(input: &str) -> u64{
    let mut sum:u64 = 0;
    let grille: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let largeur = grille.iter().map(|l| l.len()).max().unwrap();
    let mut dans_bloc = false;
    let mut debut_bloc = 0;

    // Détection des blocs
    for col in 0..largeur {
        let mut vide = true;

        for row in 0..grille.len() {
            if col < grille[row].len() && grille[row][col] != ' '{
                vide = false;
                break;
            }
        }

        if !vide && !dans_bloc{
            debut_bloc = col;
            dans_bloc = true;
        }

        // Opérations
        if vide && dans_bloc {
            let fin_bloc = col - 1;
            let mut op= ' ';
            let last_row = grille.len() - 1;

            // Opérateur
            for col in debut_bloc..=fin_bloc {
                if col < grille[last_row].len() && grille[last_row][col] != ' ' {
                    op = grille[last_row][col];
                }
            }

            // 0 pour +, 1 pour *
            let mut res_bloc = if op == '+' { 0 } else { 1 };

            for row in 0..grille.len()-1 {
                let mut s = String::new();
                for col in debut_bloc..=fin_bloc {
                    if col < grille[row].len() && grille[row][col] != ' '{
                        s.push(grille[row][col]);
                    }
                }
                let nb:u64 = s.parse().unwrap();

                if op == '+'{
                    res_bloc += nb;
                }

                if op == '*'{
                    res_bloc *= nb;;
                }
            }
            dans_bloc = false;
            sum += res_bloc;
        }
    }

    // Dernier bloc
    if dans_bloc{
        let fin_bloc = largeur - 1;
        let mut op= ' ';
        let last_row = grille.len() - 1;

        // Opérateur
        for col in debut_bloc..=fin_bloc {
            if col < grille[last_row].len() && grille[last_row][col] != ' ' {
                op = grille[last_row][col];
            }
        }

        // 0 pour +, 1 pour *
        let mut res_bloc = if op == '+' { 0 } else { 1 };

        for row in 0..grille.len()-1 {
            let mut s = String::new();
            for col in debut_bloc..=fin_bloc {
                if col < grille[row].len() && grille[row][col] != ' '{
                    s.push(grille[row][col]);
                }
            }
            let nb:u64 = s.parse().unwrap();

            if op == '+'{
                res_bloc += nb;
            }

            if op == '*'{
                res_bloc *= nb;;
            }
        }
        dans_bloc = false;
        sum += res_bloc;
    }
    sum
}

pub fn solve_part2(input: &str) -> u64{
    let mut sum:u64 = 0;
    let grille: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let largeur = grille.iter().map(|l| l.len()).max().unwrap();
    let mut dans_bloc = false;
    let mut debut_bloc = 0;

    // Détection des blocs
    for col in 0..largeur {
        let mut vide = true;

        for row in 0..grille.len() {
            if col < grille[row].len() && grille[row][col] != ' '{
                vide = false;
                break;
            }
        }

        if !vide && !dans_bloc{
            debut_bloc = col;
            dans_bloc = true;
        }

        // Opérations
        if vide && dans_bloc {
            let fin_bloc = col - 1;
            let mut op= ' ';
            let last_row = grille.len() - 1;

            // Opérateur
            for col in debut_bloc..=fin_bloc {
                if col < grille[last_row].len() && grille[last_row][col] != ' ' {
                    op = grille[last_row][col];
                }
            }

            // 0 pour +, 1 pour *
            let mut res_bloc = if op == '+' { 0 } else { 1 };

            for row in 0..grille.len()-1 {
                let mut s = String::new();
                for col in debut_bloc..=fin_bloc {
                    if col < grille[row].len() && grille[row][col] != ' '{
                        s.push(grille[row][col]);
                    }
                }
                let nb:u64 = s.parse().unwrap();

                if op == '+'{
                    res_bloc += nb;
                }

                if op == '*'{
                    res_bloc *= nb;;
                }
            }
            dans_bloc = false;
            sum += res_bloc;
        }
    }

    // Dernier bloc
    if dans_bloc{
        let fin_bloc = largeur - 1;
        let mut op= ' ';
        let last_row = grille.len() - 1;

        // Opérateur
        for col in debut_bloc..=fin_bloc {
            if col < grille[last_row].len() && grille[last_row][col] != ' ' {
                op = grille[last_row][col];
            }
        }

        // 0 pour +, 1 pour *
        let mut res_bloc = if op == '+' { 0 } else { 1 };

        for row in 0..grille.len()-1 {
            let mut s = String::new();
            for col in debut_bloc..=fin_bloc {
                if col < grille[row].len() && grille[row][col] != ' '{
                    s.push(grille[row][col]);
                }
            }
            let nb:u64 = s.parse().unwrap();

            if op == '+'{
                res_bloc += nb;
            }

            if op == '*'{
                res_bloc *= nb;;
            }
        }
        dans_bloc = false;
        sum += res_bloc;
    }
    sum
}
