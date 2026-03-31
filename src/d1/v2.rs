//Côme

pub fn solve_part1(input: &str) -> u16 {

    let mut position:i16 = 50;
    let mut compteur:u16 = 0;

    for ligne in input.lines() {
        let ligne = ligne.trim();
        let(sens,nb) = ligne.split_at(1);
        let distance:i16 = nb.parse().unwrap();

        match sens{
            "L" => position = (position - distance).rem_euclid(100),
            "R" => position = (position + distance).rem_euclid(100),
            _ => println!("Inconnu")
        }

        if position == 0{
            compteur += 1;
        }
    }
    compteur
}

pub fn solve_part2(input: &str) -> u16 {

    let mut position:i16 = 50;
    let mut compteur:u16 = 0;

    for ligne in input.lines() {
        let ligne = ligne.trim();
        let(sens,nb) = ligne.split_at(1);
        let distance:i16 = nb.parse().unwrap();

        //98 => R4 => 2   4 clics
        match sens{
            "L" => for _ in 0..distance{
                position -= 1;
                position = position.rem_euclid(100);
                if position == 0{
                    compteur += 1;
                }
            }
            "R" => for _ in 0..distance{
                position += 1;
                position = position.rem_euclid(100);
                if position == 0{
                    compteur += 1;
                }
            }
            _ => println!("Inconnu")
        }
    }
    compteur
}