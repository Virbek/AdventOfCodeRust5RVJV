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

pub fn solve_part2(input: &str) -> u64{
    let mut sum:u64 = 0;

    for ligne in input.lines() {
        let chiffres: Vec<u64> = ligne.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
        let mut douze_max:u64 = 0;
        let mut start = 0;
        
        for i in 0..12{
            let mut val = 0;
            let mut max_index = start;
            let zone = chiffres.len() - (12-i);

            for j in start..=zone{
                if chiffres[j] > val{
                    val = chiffres[j];
                    max_index = j;
                }
            }
            douze_max = douze_max * 10 + val;
            start = max_index+1;
        }
        sum += douze_max;
    }
    sum
}