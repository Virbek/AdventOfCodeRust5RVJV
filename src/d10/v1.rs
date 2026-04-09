use regex::Regex;

pub fn solve_part1(input: &str) -> u32{
    let re_crochet = Regex::new(r"\[([^\]]*)\]").unwrap();
    let re_parent = Regex::new(r"\(([^)]*)\)").unwrap();

    let mut tot: u32 = 0;

    for line in input.lines(){
        let caps = re_crochet.captures(line).unwrap();
        let target_str = &caps[1];
        let mut target: u32 = 0;

        for (i, c) in target_str.chars().enumerate(){
            if c == '#'{
                target |= 1<< i; 
            }
        }

        let mut buttons: Vec<u32> = Vec::new();

        for caps in re_parent.captures_iter(line){
            let button_str = &caps[1];
            let mut button: u32 = 0;
            for num_str in button_str.split(','){
                let index: usize = num_str.trim().parse().unwrap();
                button |= 1 <<index;
            }
            buttons.push(button);
        }

        let n = buttons.len();
        let mut min_presses = u32::MAX;
        for mask in 0..(1u32 << n){
            let mut state: u32 = 0;
            for i in 0..n{
                if mask & (1 << i ) != 0{
                    state ^= buttons[i]
                }
            }
            if state == target {
                let presses = mask.count_ones();
                if presses < min_presses{
                    min_presses = presses;
                }
            }
        }

        tot += min_presses;
        
    }

    tot


}