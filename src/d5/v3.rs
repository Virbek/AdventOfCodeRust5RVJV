// fn contains(var : u64, tab : &Vec<u64>) -> bool{
//     for i in tab{
//         if var == *i {
//             return true;
//         }
//     }
//     false
// }



pub fn solve_part1(input: &str) -> u32{

    let mut lines = input.lines();

    let index: Vec<(u64, u64)> = lines.by_ref()
        .take_while(|l| !l.trim().is_empty())
        .filter_map(|line| line.split_once('-'))
        .map(|(a, b)| (a.trim().parse().unwrap(), b.trim().parse().unwrap()))
        .collect();

    let mut contenu: Vec<u64> = lines
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim().parse().unwrap())
        .collect();

    contenu.sort_unstable();
    
    let mut used = vec![false; contenu.len()];

    let mut res: u32 = 0;

    for (x,y) in &index{
        let start = contenu.partition_point( |v| v < x);
        let end = contenu.partition_point(|v| v <= y);

        for i in start..end{
            if !used[i]{
                used[i] = true;
                res += 1;
            }
        }
    }

    res

}

pub fn solve_part2(input: &str) -> u64{
    let mut lines = input.lines();

    let mut index: Vec<(u64, u64)> = lines
        .by_ref()
        .take_while(|l| !l.trim().is_empty())
        .filter_map(|line| line.split_once('-'))
        .map(|(a, b)| (a.trim().parse().unwrap(), b.trim().parse().unwrap()))
        .collect();

    index.sort_unstable();

    let mut res: u64 = 0;
    let mut iter = index.into_iter();

    if let Some((first_x, first_y)) = iter.next(){
        let mut cur_x = first_x;
        let mut cur_y = first_y;

        for (x, y) in iter {
            if x <= cur_y &&  y > cur_y{
                cur_y = y;
            } else if x > cur_y {
                res += cur_y - cur_x + 1;
                cur_x = x;
                cur_y = y;
            }      
        }

        res += cur_y - cur_x + 1;

    }

    res
}