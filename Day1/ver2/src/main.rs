fn main() {
    let inputs = include_str!("../ressources/inputs");

    let mut position:i16 = 50;
    let mut compteur:u16 = 0;

    for ligne in inputs.lines() {
        let ligne = ligne.trim();
        let(sens,nb) = ligne.split_at(1);
        let distance:i16 = nb.parse().unwrap();

        match sens{
            "L" => position = (position + distance).rem_euclid(100),
            "R" => position = (position - distance).rem_euclid(100),
            _ => println!("Inconnu")
        }

        if position == 0{
            compteur += 1;
        }
    }
    println!("Le mot de passe est {}", compteur);
}