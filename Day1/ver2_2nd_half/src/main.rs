fn main() {
    let inputs = include_str!("../ressources/inputs");

    let mut position:i16 = 50;
    let mut compteur:u16 = 0;

    for ligne in inputs.lines() {
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
    println!("Le mot de passe est {}", compteur);
}