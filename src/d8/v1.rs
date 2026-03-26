use std::fs;

pub fn solve_part1(input: &str) -> u32{
    let mut stock : Vec<u32> = Vec::new();
    

    for c in input.lines(){
        let nombres: Vec<u32> = c.trim().split(',').map(|n| n.trim()).map(|n| n.parse().expect("Pas un nombre valide")).collect();

        let somme: u32 = nombres.iter().sum();

        stock.push(somme);

    }

    /*for s in &stock{
        println!("la somme : {}", s);
    }*/

    stock[0]

}