use std::collections::HashSet;

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


pub fn solve_part2(input: &str) -> u64 {
    let red: Vec<(i64, i64)> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut p = l.split(',');
            (p.next().unwrap().trim().parse().unwrap(),
             p.next().unwrap().trim().parse().unwrap())
        })
        .collect();

    let n = red.len();

    let mut xs: Vec<i64> = Vec::new();
    let mut ys: Vec<i64> = Vec::new();
    for &(x, y) in &red {
        for d in -1..=1 { xs.push(x + d); ys.push(y + d); }
    }
    xs.sort(); xs.dedup();
    ys.sort(); ys.dedup();
    let ix = |v: i64| xs.binary_search(&v).unwrap();
    let iy = |v: i64| ys.binary_search(&v).unwrap();
    let w = xs.len();
    let h = ys.len();

    // Tracer la boucle
    let mut on_loop = vec![vec![false; h]; w];
    for i in 0..n {
        let (x1, y1) = red[i];
        let (x2, y2) = red[(i + 1) % n];
        if x1 == x2 {
            let (a, b) = (y1.min(y2), y1.max(y2));
            for j in 0..h { if ys[j] >= a && ys[j] <= b { on_loop[ix(x1)][j] = true; } }
        } else {
            let (a, b) = (x1.min(x2), x1.max(x2));
            for i2 in 0..w { if xs[i2] >= a && xs[i2] <= b { on_loop[i2][iy(y1)] = true; } }
        }
    }

    // Flood fill extérieur
    let mut outside = vec![vec![false; h]; w];
    let mut stack = vec![(0usize, 0usize)];
    outside[0][0] = true;
    while let Some((cx, cy)) = stack.pop() {
        for (dx, dy) in [(1i32,0),(-1,0),(0,1),(0,-1)] {
            let (nx, ny) = (cx as i32 + dx, cy as i32 + dy);
            if nx < 0 || ny < 0 || nx >= w as i32 || ny >= h as i32 { continue; }
            let (nx, ny) = (nx as usize, ny as usize);
            if !on_loop[nx][ny] && !outside[nx][ny] {
                outside[nx][ny] = true;
                stack.push((nx, ny));
            }
        }
    }

    // Prefix sum 2D : compter les cellules non-green dans un rectangle
    // non-green = outside. On compte les outside dans chaque rect.
    // Si count_outside == 0, tout est green/red.
    let mut prefix = vec![vec![0i64; h + 1]; w + 1];
    for i in 0..w {
        for j in 0..h {
            let val = if outside[i][j] { 1 } else { 0 };
            prefix[i+1][j+1] = val + prefix[i][j+1] + prefix[i+1][j] - prefix[i][j];
        }
    }
    let count_outside = |x1: usize, y1: usize, x2: usize, y2: usize| -> i64 {
        prefix[x2+1][y2+1] - prefix[x1][y2+1] - prefix[x2+1][y1] + prefix[x1][y1]
    };

    // Tester chaque paire de rouges
    let mut max_area: u64 = 0;
    for i in 0..red.len() {
        for j in (i+1)..red.len() {
            let (x1, y1) = red[i];
            let (x2, y2) = red[j];
            let (lx, hx) = (x1.min(x2), x1.max(x2));
            let (ly, hy) = (y1.min(y2), y1.max(y2));

            let (ci1, ci2) = (ix(lx), ix(hx));
            let (cj1, cj2) = (iy(ly), iy(hy));

            if count_outside(ci1, cj1, ci2, cj2) == 0 {
                let area = (hx - lx + 1) as u64 * (hy - ly + 1) as u64;
                if area > max_area { max_area = area; }
            }
        }
    }

    max_area
}
