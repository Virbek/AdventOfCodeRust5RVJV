pub fn solve_part1(input: &str) -> u64{
    let mut sum:u64 = 0;

    for ligne in input.lines() {
        let chiffres: Vec<u64> = ligne.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
        let mut deux_max:u64 = 0;
        let mut start = 0;

        for i in 0..2{
            let mut val = 0;
            let mut max_index = start;
            let zone = chiffres.len() - (2-i);

            for j in start..=zone{
                if chiffres[j] > val{
                    val = chiffres[j];
                    max_index = j;
                }
            }
            deux_max = deux_max * 10 + val;
            start = max_index+1;
        }
        sum += deux_max;
    }
    sum
}