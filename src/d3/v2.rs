pub fn solve_part1(input: &str) -> u32{
    let mut sum:u32 = 0;

    for ligne in input.lines() {
        let chiffres: Vec<u32> = ligne.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut paire_max:u32 = 0;

        // 987898
        for i in 0..chiffres.len(){
            // i < j donc on part de i+1
            for j in i+1..chiffres.len(){
                let val = chiffres[i]*10 + chiffres[j];
                if val > paire_max{
                    paire_max = val;
                }
            }
        }
        sum += paire_max;
    }
    sum
}

pub fn solve_part2(input: &str) -> u32{
    let mut sum:u32 = 0;

    for ligne in input.lines() {
        let chiffres: Vec<u32> = ligne.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut douze_max:u32 = 0;

        // 987 898 898 898
        for i in 0..chiffres.len(){
            // i < j donc on part de i+1
            for j in i+1..chiffres.len(){
                let val = chiffres[i]*10 + chiffres[j];
                if val > douze_max{
                    douze_max = val;
                }
            }
        }
        sum += douze_max;
    }
    sum
}
