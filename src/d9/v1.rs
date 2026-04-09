use std::fs;

pub fn solve_part1(input: &str) -> u64{
    let mut stock : Vec<(u64,u64)> = Vec::new();

    for c in input.lines(){

        let c = c.trim();

        let mut parts = c.split(',');
        let a: u64 = parts.next().unwrap().trim().parse().unwrap();
        let b: u64 = parts.next().unwrap().trim().parse().unwrap();

        stock.push((a, b));

    }
    let mut max: u64 = 0;
    for (x, y) in &stock{
        for (a,b) in &stock {
            if x == a && y == b {continue;}
            let diff1 = if x > a { x - a } else { a - x };
            let diff2 = if y > b { y - b } else { b - y };
            let sum = (diff1+ 1) * (diff2+ 1);
            if sum > max {max = sum;}
        }
    }

    max
}

pub fn solve_part2(input: &str) -> u64{
    let mut stock : Vec<(u64,u64)> = Vec::new();

    for c in input.lines(){

        let c = c.trim();

        let mut parts = c.split(',');
        let a: u64 = parts.next().unwrap().trim().parse().unwrap();
        let b: u64 = parts.next().unwrap().trim().parse().unwrap();

        stock.push((a, b));

    }
    let mut max: u64 = 0;
    for (x, y) in &stock{
        for (a,b) in &stock {
            if x == a && y == b {continue;}
            let diff1 = if x > a { x - a } else { a - x };
            let diff2 = if y > b { y - b } else { b - y };
            let sum = (diff1+ 1) * (diff2+ 1);
            if sum > max {max = sum;}
        }
    }

    max
}

