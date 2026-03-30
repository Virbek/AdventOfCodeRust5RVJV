use std::fs;
use std::thread::park;

pub fn calculate_dist(s: (u32, u32, u32), s2: (u32, u32, u32)) -> f64 {
    let dx = (s.0 as f64) - (s2.0 as f64);
    let dy = (s.1 as f64) - (s2.1 as f64);
    let dz = (s.2 as f64) - (s2.2 as f64);
    (dx * dx + dy * dy + dz * dz).sqrt()
}

pub fn solve_part1(input: &str) -> u32 {
    let mut stock: Vec<(u32, u32, u32)> = Vec::new();
    let mut groups: Vec<Vec<usize>> = Vec::new();
    let mut vec : Vec<u32> = Vec::new();

    for c in input.lines() {
        let mut split = c.split(',');
        stock.push((
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
        ));
    }

    let mut pairs: Vec<(f64, usize, usize)> = Vec::new();
    for i in 0..stock.len() {
        for j in (i + 1)..stock.len() {
            let dist = calculate_dist(stock[i], stock[j]);
            pairs.push((dist, i, j));
        }
    }

    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // println!("{:?}", pairs);

    let mut stop : u32 = 0;

    for (_, i, j) in pairs {
        if stop >= 1000 { // 1000
            break;
        }
        let group_i = groups.iter().position(|g| g.contains(&i));
        let group_j = groups.iter().position(|g| g.contains(&j));

        match (group_i, group_j) {
            (Some(a), Some(b)) if a == b => {
                stop += 1;
            }

            (Some(a), Some(b)) => {
                let merged = groups[b].clone();
                groups[a].extend(merged);
                groups.remove(b);
                stop += 1;
            }

            (Some(a), None) => {
                groups[a].push(j);
                stop += 1;
            }

            (None, Some(b)) => {
                groups[b].push(i);
                stop += 1;
            }

            (None, None) => {
                groups.push(vec![i, j]);
                stop += 1;
            }
        }
    }
    //
    // for (i, group) in groups.iter().enumerate() {
    //     println!("Groupe {} : {:?}", i, group);
    // }

    for group in groups.clone() {
        vec.push(group.len() as u32);
    }
    vec.sort_by(|a, b| b.cmp(a));
    vec.truncate(3);

    vec[0] * vec[1] * vec[2]
}


pub fn solve_part2(input: &str) -> u64 {
    let mut stock: Vec<(u32, u32, u32)> = Vec::new();
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for c in input.lines() {
        let mut split = c.split(',');
        stock.push((
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
        ));
    }
    for i in 0..stock.len() {
        groups.push(vec![i]);
    }
    let mut pairs: Vec<(f64, usize, usize)> = Vec::new();
    for i in 0..stock.len() {
        for j in (i + 1)..stock.len() {
            let dist = calculate_dist(stock[i], stock[j]);
            pairs.push((dist, i, j));
        }
    }
    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut last_i = 0;
    let mut last_j = 0;

    for (_, i, j) in &pairs {
        if groups.len() == 1 { break; }
        let group_i = groups.iter().position(|g| g.contains(i));
        let group_j = groups.iter().position(|g| g.contains(j));
        match (group_i, group_j) {
            (Some(a), Some(b)) if a != b => {
                let merged = groups[b].clone();
                groups[a].extend(merged);
                groups.remove(b);
                last_i = *i;
                last_j = *j;
            }
            _ => {}
        }
    }
    stock[last_i].0 as u64 * stock[last_j].0 as u64
}