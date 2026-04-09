//Côme

pub fn solve_part1(input: &str) -> u16 {

    let mut position:i16 = 50;
    let mut compteur:u16 = 0;

    for ligne in input.lines() {
        let bytes = ligne.as_bytes();
        let nb = std::str::from_utf8(&bytes[1..]).unwrap();
        let distance:i16 = nb.parse::<i16>().unwrap();

        match bytes[0]{
            b'L' => position = (position - distance).rem_euclid(100),
            b'R' => position = (position + distance).rem_euclid(100),
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
        let bytes = ligne.as_bytes();
        let nb = std::str::from_utf8(&bytes[1..]).unwrap();
        let distance:i16 = nb.parse().unwrap();

        //98 => R4 => 2   4 clics
        match bytes[0]{
            b'L' => for _ in 0..distance{
                position -= 1;
                if position < 0{
                    position = 99;
                }
                if position == 0{
                    compteur += 1;
                }
            }
            b'R' => for _ in 0..distance{
                position += 1;
                if position > 99{
                    position = 0;
                }
                if position == 0{
                    compteur += 1;
                }
            }
            _ => println!("Inconnu")
        }
    }
    compteur
}